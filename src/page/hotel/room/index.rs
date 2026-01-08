use super::*;
use leptos::prelude::*;
#[component]
pub fn HotelRoomPane() -> impl IntoView {
    let state = ChatState::new();
    provide_context(state);
    view! {
        {move || match state.step.get() {
            ChatStep::SelectOrJoin => view! { <SelectOrJoinStep /> }.into_any(),
            ChatStep::Started(room) => view! { <StartedStep room /> }.into_any(),
        }}
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ChatState {
    pub step: RwSignal<ChatStep>,
}
impl ChatState {
    pub fn new() -> Self {
        Self {
            step: RwSignal::new(ChatStep::SelectOrJoin),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ChatStep {
    SelectOrJoin,
    ///room_key
    Started(String),
}
