use std::collections::VecDeque;

use crate::service::{async_get_vip_level_item_num, async_init_socket};

use super::*;
use leptos::{prelude::*, task::spawn_local};
use tower::{
    common::{
        chat::{Message, UserMessage},
        dict::VipLevelItem,
        user::UserInfoResp,
    },
    tauri_web::{prelude::to_room_url, AppState},
    web::common::{date, WebResultExt},
};

#[component]
pub fn ChatPane(room: String, #[prop(default = false)] is_roomchat: bool) -> impl IntoView {
    let room_url = to_room_url(&room);
    if room_url.is_err() {
        return view! { "未知的房间号" }.into_any();
    }
    let room_url = room_url.unwrap();
    let app_state = expect_context::<AppState>();
    view! {
        {move || {
            app_state
                .login_resource
                .to_view(|user| {
                    let room_url = room_url.clone();
                    let chat_state = ChatState::new(user.clone(), &room, is_roomchat);
                    chat_state.refresh_users();
                    provide_context(chat_state);
                    let user_clone = user.clone();
                    spawn_local(async move {
                        let room_url = if is_roomchat {
                            let person_total = async_get_vip_level_item_num(
                                    VipLevelItem::RoomPersonNum,
                                )
                                .await
                                .unwrap();
                            chat_state.allow_max.set(person_total);
                            format!("{}?next={}", room_url, person_total)
                        } else {
                            room_url.clone()
                        };
                        async_init_socket(
                                user_clone,
                                room_url,
                                chat_state.sender,
                                chat_state.receiver,
                            )
                            .await;
                    });
                    view! {
                        <div class="flex gap-4 pr-4 pb-4 h-full">
                            <UserList />
                            <div class="flex flex-col justify-between">
                                <ChatInput />
                                <ChatList user=user.clone() />
                            </div>
                        </div>
                    }
                        .into_any()
                })
        }}
    }
    .into_any()
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ChatState {
    pub me: RwSignal<UserInfoResp>,
    pub users: RwSignal<Vec<UserInfoResp>>,
    pub receiver: RwSignal<VecDeque<UserMessage>>,
    pub sender: RwSignal<VecDeque<UserMessage>>,
    pub room_key: RwSignal<String>,
    pub is_roomchat: RwSignal<bool>,
    pub allow_max: RwSignal<usize>,
}
impl ChatState {
    pub fn new(me: UserInfoResp, room_key: &str, is_roomchat: bool) -> Self {
        Self {
            room_key: RwSignal::new(room_key.to_string()),
            users: RwSignal::new(vec![me.clone()]),
            receiver: RwSignal::new(VecDeque::new()),
            sender: RwSignal::new(VecDeque::new()),
            me: RwSignal::new(me),
            is_roomchat: RwSignal::new(is_roomchat),
            allow_max: RwSignal::new(0),
        }
    }

    pub fn find_user_by_id(&self, user_id: &str) -> UserInfoResp {
        let user = self.users.get_untracked();
        user.iter()
            .find(|user| user.user_id == user_id)
            .cloned()
            .unwrap()
    }
}

impl ChatState {
    pub fn send_message(&self, message: Message) {
        let message = UserMessage {
            user_id: self.me.get_untracked().user_id,
            message,
            send_time: date::now(),
        };
        self.receiver
            .update(|messages| messages.push_front(message.clone()));
        self.sender.update(|sender| sender.push_front(message));
    }
    pub fn add_local_message(&self, message: Message) {
        let message = UserMessage {
            user_id: self.me.get_untracked().user_id,
            message,
            send_time: date::now(),
        };
        self.receiver
            .update(|messages| messages.push_front(message.clone()));
    }
    pub fn refresh_users(self) {
        Effect::new(move |_| {
            let messages = self.receiver.get();
            let mut users: Vec<UserInfoResp> = messages
                .iter()
                .filter_map(|m| {
                    if let Message::Joined(user) = &m.message {
                        Some(user.clone())
                    } else {
                        None
                    }
                })
                .collect();
            users.insert(0, self.me.get_untracked());
            self.users.set(users);
        });
    }
}
