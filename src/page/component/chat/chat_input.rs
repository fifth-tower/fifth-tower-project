use super::*;
use leptos::{html::Textarea, prelude::*};
use leptos_use::{use_clipboard, UseClipboardReturn};
use tower::{common::chat::Message, web_hotkey::*};

#[component]
pub fn ChatInput(#[prop(into,default="".into())] class: Signal<String>) -> impl IntoView {
    let context = expect_context::<ChatState>();
    let message = RwSignal::new("".to_string());
    let node_ref = NodeRef::<Textarea>::new();

    provide_hotkeys_context(node_ref, false, scopes!("*"));
    use_hotkeys_ref!((node_ref,"alt+keyS", "*") => move |_| {
        context.send_message(Message::Text(message.get_untracked()));
        message.set("".to_string());
    });

    view! {
        <div class=format!(
            "flex flex-wrap gap-4 justify-start items-end {}",
            class.get_untracked(),
        )>
            <MessageTextarea
                node_ref
                class="overscroll-auto flex-auto lg:flex-initial".to_string()
                message
            ></MessageTextarea>
            <SendButton class="flex-initial".to_string() message />
            <ClearButton class="flex-initial".to_string() />
            <CopyRoomButton class="flex-initial".to_string() />
        </div>
    }
    .into_any()
}

#[component]
fn MessageTextarea(
    class: String,
    message: RwSignal<String>,
    node_ref: NodeRef<Textarea>,
) -> impl IntoView {
    view! {
        <textarea
            node_ref=node_ref
            placeholder="在这里可以发言噢~"
            class=format!("textarea textarea-xl textarea-secondary lg:w-xl {}", class)
            rows="5"
            prop:value=move || message.get()
            on:input:target=move |ev| message.set(ev.target().value())
        ></textarea>
    }
}

#[component]
fn SendButton(class: String, message: RwSignal<String>) -> impl IntoView {
    let context = expect_context::<ChatState>();
    let send = move |context: ChatState| {
        context.send_message(Message::Text(message.get_untracked()));
        message.set("".to_string());
    };
    view! {
        <button
            class=format!("btn btn-soft btn-secondary {}", class)
            on:click=move |_| { send(context) }
        >
            "发送（ALT+S）"
        </button>
    }
}

#[component]
fn ClearButton(class: String) -> impl IntoView {
    let context = expect_context::<ChatState>();
    let clear = |context: ChatState| {
        context.receiver.update(|m| m.clear());
    };
    view! {
        <button
            class=format!("btn btn-soft btn-secondary {}", class)
            on:click=move |_| { clear(context.clone()) }
        >
            清屏
        </button>
    }
}

#[component]
pub(crate) fn CopyRoomButton(class: String) -> impl IntoView {
    let context = expect_context::<ChatState>();
    let UseClipboardReturn { copy, copied, .. } = use_clipboard();
    Effect::new(move |_| {
        copied
            .get()
            .then(|| context.add_local_message(Message::CopyRoom));
    });
    view! {
        <button
            class=format!("btn btn-soft btn-secondary {}", class)
            on:click=move |_| {
                copy(&context.room_key.get_untracked());
            }
        >
            复制房间号
        </button>
    }
}
