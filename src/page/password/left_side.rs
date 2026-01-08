use leptos::{logging::log, prelude::*, task::spawn_local};
use tower::{
    tauri_model::{password::*, StoreFile, StoreType},
    common::bincode_decode,
    tauri_web::{prelude::*, service::store::async_delete_store},
    web::common::WebResultExt,
};

use super::*;

#[component]
pub(crate) fn LeftSide(#[prop(into,default="".into())] class: String) -> impl IntoView {
    let state = expect_context::<PasswordState>();
    let side_state = LeftSideState::new();
    provide_context(side_state);
    view! {
        {move || {
            state
                .password_resource
                .to_view(|passwords| {
                    view! {
                        <ul class=format!("w-56 lg:w-72 menu bg-base-200 rounded-box  {}", class)>
                            <li>
                                <a
                                    class="group"
                                    on:click=move |ev| {
                                        ev.stop_propagation();
                                        state.open_add_file.set(true);
                                    }
                                >
                                    新增密码本

                                    <button
                                        class="hidden group-hover:flex btn btn-xs btn-ghost"
                                        on:click=move |ev| {
                                            ev.stop_propagation();
                                            state.open_set_pin.set(true);
                                        }
                                    >
                                        设置Pin码
                                    </button>
                                </a>
                            </li>
                            {passwords
                                .files
                                .iter()
                                .map(|file| {
                                    let data: Vec<PasswordItem> = bincode_decode(&file.content);
                                    let file = StoreFile::new(
                                        &file.file_id,
                                        &file.label,
                                        &file.user_id,
                                    );
                                    view! { <PasswordLi data file /> }
                                })
                                .collect_view()}
                        </ul>
                    }
                        .into_any()
                })
        }}

        <Show when=move || { state.open_add_file.get() }>
            <Dialog open=state.open_add_file title="新增密码本">
                <AddFileForm open=state.open_add_file />
            </Dialog>
        </Show>
        <Show when=move || { state.open_update_file.get() }>
            <Dialog open=state.open_update_file title="修改密码本">
                <UpdateFileForm open=state.open_update_file />
            </Dialog>
        </Show>
        <Show when=move || { state.open_set_pin.get() }>
            <Dialog open=state.open_set_pin title="设置Pin码">
                <SetPinForm open=state.open_set_pin />
            </Dialog>
        </Show>
    }
}

#[component]
fn PasswordLi(
    #[prop(into)] data: Signal<Vec<PasswordItem>>,
    #[prop(into)] file: Signal<StoreFile>,
) -> impl IntoView {
    let state = expect_context::<PasswordState>();
    let side_state = expect_context::<LeftSideState>();
    view! {
        <li>
            <a
                class="flex flex-col items-start group"
                on:click=move |ev| {
                    ev.stop_propagation();
                    side_state.set_password_id(file.get_untracked().file_id);
                    state.set_current(Some(file.get_untracked()), data.get_untracked());
                }
            >

                <span>{file.get_untracked().label}</span>
                <span class="hidden group-hover:flex">
                    <button
                        class="btn btn-xs btn-ghost"
                        on:click=move |ev| {
                            ev.stop_propagation();
                            state.set_current(Some(file.get_untracked()), data.get_untracked());
                            state.open_update_file.set(true);
                        }
                    >
                        修改
                    </button>
                    <DeleteFileButton file />

                </span>
            </a>
        </li>
    }
}
#[component]
fn DeleteFileButton(file: Signal<StoreFile>) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let state = expect_context::<PasswordState>();
    view! {
        <button
            class="btn btn-xs"
            on:click=move |ev| {
                ev.stop_propagation();
                let file_id = file.get_untracked().file_id;
                spawn_local(async move {
                    let resp = async_delete_store(StoreType::Password, &file_id).await;
                    match resp {
                        Ok(_) => {
                            app_state.success("删除成功。");
                            state.password_resource.refetch();
                        }
                        Err(err) => app_state.error(err.to_string()),
                    }
                });
            }
        >
            删除
        </button>
    }
}

#[derive(Clone, Copy)]
struct LeftSideState {
    pub current_password_id: RwSignal<String>,
}

impl LeftSideState {
    fn new() -> Self {
        Self {
            current_password_id: RwSignal::new("".to_string()),
        }
    }
    fn set_password_id(&self, password_id: String) {
        self.current_password_id.set(password_id);
    }
    fn class(&self, password_id: String) -> String {
        if self.current_password_id.get().eq(&password_id) {
            "shadow-sm"
        } else {
            ""
        }
        .to_string()
    }
}
