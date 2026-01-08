use super::*;
use leptos::prelude::*;
use tower::{common::dict::VipLevelItem, web::component::Helper};

#[component]
pub fn UserList(#[prop(into,default="".into())] class: Signal<String>) -> impl IntoView {
    let chat_state = expect_context::<ChatState>();
    view! {
        <div class=format!(
            "flex flex-col flex-none w-56 lg:w-72  bg-base-200 rounded-box {}",
            class.get_untracked(),
        )>
            <label class="menu-title">
                用户列表
                {move || {
                    chat_state
                        .is_roomchat
                        .get_untracked()
                        .then(|| {
                            let config_num = chat_state.allow_max.get();
                            let content = VipLevelItem::RoomPersonNum.message(config_num);
                            view! { <Helper>{content}</Helper> }
                        })
                }}
            </label>
            {move || {
                let users = chat_state.users.get();
                let class = if users.len() > 2 { "basis-1/4" } else { "" };
                view! {
                    <div class="flex flex-wrap gap-1 p-1">
                        {users
                            .iter()
                            .map(|user| {
                                view! { <Avatar user=user.clone() class /> }
                            })
                            .collect_view()}
                    </div>
                }
            }}
        </div>
    }
}
