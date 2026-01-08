use super::*;
use leptos::prelude::*;
use tower::{common::user::UserInfoResp, tauri_web::prelude::to_avatar};

#[component]
pub(crate) fn Avatar(
    user: UserInfoResp,
    #[prop(into,default="".into())] class: Signal<String>,
) -> impl IntoView {
    view! {
        {move || {

            view! {
                <div class=format!("shadow-sm card bg-base-100 card-sm {}", class.get_untracked())>
                    <figure>
                        <img src=to_avatar(user.avatar) />
                    </figure>
                    <div class="card-body">
                        <p>{user.nickname.clone()}</p>
                    </div>
                </div>
            }
        }}
    }
}
