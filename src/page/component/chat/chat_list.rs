use super::*;
use leptos::prelude::*;
use tower::{
    common::{
        chat::{Message, UserMessage},
        user::UserInfoResp,
    },
    tauri_web::prelude::to_avatar,
};

#[component]
pub fn ChatList(#[prop(into)] user: Signal<UserInfoResp>) -> impl IntoView {
    let chat_state = expect_context::<ChatState>();
    view! {
        <div class="w-full">
            {move || {
                chat_state
                    .receiver
                    .get()
                    .iter()
                    .map(|message| {
                        view! { <Chat me=user.get_untracked() message=message.clone() /> }
                    })
                    .collect_view()
            }}
        </div>
    }
}

#[component]
fn Chat(me: UserInfoResp, message: UserMessage) -> impl IntoView {
    let chat_state = expect_context::<ChatState>();
    let (class, user) = if me.user_id == message.user_id {
        ("chat-end", me)
    } else {
        ("chat-start", chat_state.find_user_by_id(&message.user_id))
    };
    view! {
        <div class=format!("chat {}", class)>
            <div class="chat-image avatar">
                <div class="w-10 rounded-full">
                    <img src=to_avatar(user.avatar) />
                </div>
            </div>
            <div class="chat-bubble">
                {match message.message {
                    Message::Joined(user) => {
                        view! { <Joined user send_time=message.send_time /> }.into_any()
                    }
                    Message::Text(text) => {
                        view! { <Text user text send_time=message.send_time /> }.into_any()
                    }
                    Message::Image(image) => {
                        view! { <Image user image send_time=message.send_time /> }.into_any()
                    }
                    Message::Inputing => {
                        view! { <Inputing user send_time=message.send_time /> }.into_any()
                    }
                    Message::Inputed => {
                        view! { <Inputed user send_time=message.send_time /> }.into_any()
                    }
                    Message::Leave => {
                        view! { <Leave user send_time=message.send_time /> }.into_any()
                    }
                    Message::CopyRoom => {
                        view! { <CopyRoom send_time=message.send_time /> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
