use leptos::{logging::log, prelude::*, task::spawn_local};
use tower::{
    common::bincode_decode,
    tauri_model::{note::*, StoreFile, StoreType},
    tauri_web::{
        prelude::*,
        service::store::{async_delete_store, async_save_store},
    },
    web::{common::WebResultExt, component::TreeNodeLi},
};

use super::*;

#[component]
pub fn LeftSide(#[prop(into,default="".into())] class: String) -> impl IntoView {
    let state = expect_context::<NoteState>();
    let side_state = LeftSideState::new();
    provide_context(side_state);
    view! {
        {move || {
            state
                .note_resource
                .to_view(|notes| {
                    view! {
                        <ul class=format!("w-56 lg:w-72 menu bg-base-200 rounded-box  {}", class)>
                            <li>
                                <a on:click=move |ev| {
                                    ev.stop_propagation();
                                    state.open_add_file.set(true);
                                }>新增记事</a>
                            </li>
                            {notes
                                .files
                                .iter()
                                .map(|file| {
                                    log!("file.content:{:?}",file.content);
                                    let data: Vec<NoteDir> = bincode_decode(&file.content);
                                    let (data, children): (Vec<NoteText>, Vec<NoteDir>) = if data
                                        .is_empty()
                                    {
                                        (vec![], vec![])
                                    } else {
                                        let data = data.first().cloned().unwrap();
                                        (data.data, data.children)
                                    };
                                    let root = NoteDir {
                                        id: file.file_id.clone(),
                                        parent_id: None,
                                        label: file.label.clone(),
                                        data,
                                        children,
                                    };
                                    let root = Signal::from(root);
                                    let file = Signal::from(
                                        StoreFile::new(&file.file_id, &file.label, &file.user_id),
                                    );
                                    view! {
                                        <TreeNodeLi
                                            data=root
                                            node_inner_render=move |note| render_node(note, root, file)
                                        />
                                    }
                                })
                                .collect_view()}
                        </ul>
                    }
                        .into_any()
                })
        }}

        <Show when=move || { state.open_add_file.get() }>
            <Dialog open=state.open_add_file title="新增记事">
                <AddFileForm open=state.open_add_file />
            </Dialog>
        </Show>
        <Show when=move || { state.open_add_dir.get() }>
            <Dialog open=state.open_add_dir title="新增目录">
                <AddDirForm open=state.open_add_dir />
            </Dialog>
        </Show>
        <Show when=move || { state.open_update_dir.get() }>
            <Dialog open=state.open_update_dir title="修改目录">
                <UpdateDirForm open=state.open_update_dir />
            </Dialog>
        </Show>
    }
}

fn render_node(data: NoteDir, root: Signal<NoteDir>, file: Signal<StoreFile>) -> impl IntoView {
    let state = expect_context::<NoteState>();
    let data: Signal<NoteDir> = Signal::from(data);
    let side_state = expect_context::<LeftSideState>();
    view! {
        <a
            class="flex flex-col items-start group"
            on:click=move |ev| {
                ev.stop_propagation();
                side_state.set_note_id(data.get_untracked().id);
                state
                    .set_current(
                        Some(file.get_untracked()),
                        Some(root.get_untracked()),
                        Some(data.get_untracked()),
                    );
            }
        >
            <span class=move || {
                side_state.class(data.get_untracked().id)
            }>{data.get_untracked().label}</span>
            <span class="hidden group-hover:flex">
                <button
                    class="btn btn-xs btn-ghost"
                    on:click=move |ev| {
                        ev.stop_propagation();
                        state
                            .set_current(
                                Some(file.get_untracked()),
                                Some(root.get_untracked()),
                                Some(data.get_untracked()),
                            );
                        state.open_add_dir.set(true);
                    }
                >
                    新增子项
                </button>
                <button
                    class="btn btn-xs btn-ghost"
                    on:click=move |ev| {
                        ev.stop_propagation();
                        state
                            .set_current(
                                Some(file.get_untracked()),
                                Some(root.get_untracked()),
                                Some(data.get_untracked()),
                            );
                        state.open_update_dir.set(true);
                    }
                >
                    修改
                </button>
                {move || {
                    if data.get_untracked().parent_id.is_none() {
                        view! { <DeleteFileButton data root file /> }.into_any()
                    } else {
                        view! { <DeleteDirButton data root file /> }.into_any()
                    }
                }}

            </span>
        </a>
    }
}

#[component]
fn DeleteFileButton(
    data: Signal<NoteDir>,
    root: Signal<NoteDir>,
    file: Signal<StoreFile>,
) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let state = expect_context::<NoteState>();
    view! {
        <button
            class="btn btn-xs"
            on:click=move |ev| {
                ev.stop_propagation();
                let file_id = file.get_untracked().file_id;
                spawn_local(async move {
                    let resp = async_delete_store(StoreType::Note, &file_id).await;
                    match resp {
                        Ok(_) => {
                            app_state.success("删除成功。");
                            state.note_resource.refetch();
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

#[component]
fn DeleteDirButton(
    data: Signal<NoteDir>,
    root: Signal<NoteDir>,
    file: Signal<StoreFile>,
) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let state = expect_context::<NoteState>();
    view! {
        <button
            class="btn btn-xs"
            on:click=move |ev| {
                ev.stop_propagation();
                let mut root = root.get_untracked();
                let file_id = root.id.clone();
                root.visit_mut(
                    &mut |n| {
                        let note = data.get_untracked();
                        if n.id.eq(&note.parent_id.clone().unwrap_or_default()) {
                            n.children_mut().retain(|c| { c.id.ne(&note.id) });
                            return true;
                        }
                        return false;
                    },
                );
                spawn_local(async move {
                    let mut file = file.get_untracked();
                    file.content = bincode_encode(root.children());
                    let resp = async_save_store(StoreType::Note, &file_id, &bincode_encode(file))
                        .await;
                    match resp {
                        Ok(_) => {
                            app_state.success("删除成功。");
                            state.note_resource.refetch();
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
    pub current_note_id: RwSignal<String>,
}

impl LeftSideState {
    fn new() -> Self {
        Self {
            current_note_id: RwSignal::new("".to_string()),
        }
    }
    fn set_note_id(&self, note_id: String) {
        self.current_note_id.set(note_id);
    }
    fn class(&self, note_id: String) -> String {
        if self.current_note_id.get().eq(&note_id) {
            "shadow-sm"
        } else {
            ""
        }
        .to_string()
    }
}
