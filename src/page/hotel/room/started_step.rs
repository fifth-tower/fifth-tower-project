use crate::page::component::ChatPane;

use leptos::prelude::*;

#[component]
pub fn StartedStep(room: String) -> impl IntoView {
    view! { <ChatPane room is_roomchat=true /> }
}
