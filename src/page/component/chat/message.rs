use leptos::prelude::*;
use tower::{common::user::UserInfoResp, web::common::date::from_mills_and};

#[component]
pub(crate) fn Joined(user: UserInfoResp, send_time: f64) -> impl IntoView {
    view! {
        <div class="chat-header">
            <Time mills=send_time />
        </div>
        <div class="chat-bubble">{user.nickname}"  来了"</div>
    }
}

#[component]
pub(crate) fn Text(user: UserInfoResp, text: String, send_time: f64) -> impl IntoView {
    view! {
        <div class="chat-header">{user.nickname} <Time mills=send_time /></div>
        <div class="chat-bubble">
            <pre class="text-wrap">{text}</pre>
        </div>
    }
}

#[component]
pub(crate) fn Image(user: UserInfoResp, image: String, send_time: f64) -> impl IntoView {
    view! {
        <div class="chat-header">{user.nickname} <Time mills=send_time /></div>
        <div class="chat-bubble">
            <img src=format!("data:image/png;base64,{}", image) />
        </div>
    }
}

#[component]
pub(crate) fn Inputing(user: UserInfoResp, send_time: f64) -> impl IntoView {
    view! {
        <div class="chat-header">{user.nickname} <Time mills=send_time /></div>
        <div class="chat-bubble"></div>
    }
}

#[component]
pub(crate) fn Inputed(user: UserInfoResp, send_time: f64) -> impl IntoView {
    view! {
        <div class="chat-header">{user.nickname} <Time mills=send_time /></div>
        <div class="chat-bubble"></div>
    }
}

#[component]
pub(crate) fn Leave(user: UserInfoResp, send_time: f64) -> impl IntoView {
    view! {
        <div class="chat-header">
            <Time mills=send_time />
        </div>
        <div class="chat-bubble">{user.nickname}"  走了"</div>
    }
}

#[component]
pub(crate) fn CopyRoom(send_time: f64) -> impl IntoView {
    view! {
        <div class="chat-header">
            <Time mills=send_time />
        </div>
        <div class="chat-bubble text-wrap wrap-anywhere">房间号已复制</div>
    }
}

#[component]
fn Time(mills: f64) -> impl IntoView {
    view! { <time class="text-xs opacity-50">{from_mills_and(mills, "hh:mi")}</time> }
}
