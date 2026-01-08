use flow_model::RunStatus;
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn RunStatusIcon(#[prop(into)] status: Signal<RunStatus>) -> impl IntoView {
    let status = status.get_untracked();
    match status {
        RunStatus::Running => view! { <Icon icon=icondata::AiLoadingOutlined /> }.into_any(),
        RunStatus::Stopped(true) => view! { <Icon icon=icondata::AiCheckOutlined /> }.into_any(),
        RunStatus::Stopped(false) => view! { <Icon icon=icondata::AiCloseOutlined /> }.into_any(),
        RunStatus::Aborted => view! { <Icon icon=icondata::AiStopOutlined /> }.into_any(),
        RunStatus::Init => view! {}.into_any(),
    }
}
