use super::*;
use leptos::{prelude::*, task::spawn_local};
use tower::tauri_model::note::NoteDir;
use tower::tauri_model::StoreType;
use tower::tauri_web::prelude::*;
use tower::tauri_web::service::store::async_save_store;
use tower::tauri_web::AppState;

#[component]
pub(crate) fn UpdateDirForm(open: RwSignal<bool>) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let dir_state = expect_context::<NoteState>();
    let form = UpdateDirFormData::new(dir_state.current_note.get_untracked().unwrap());
    let op_tiper = OpTiper::new();
    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            spawn_local(async move {
                let mut root = dir_state.current_root.get_untracked().unwrap();
                let file_id = root.id.clone();
                root.visit_mut(
                    &mut |n| {
                        let id = form.id.get_untracked();
                        if n.id.eq(&id) {
                            n.label = form.label.get_untracked();
                            return true;
                        }
                        return false;
                    },
                );
                let mut file = dir_state.current_file.get_untracked().unwrap();
                file.content = bincode_encode(vec![root]);
                let resp = async_save_store(StoreType::Note, &file_id, &bincode_encode(file)).await;
                tip_or(
                    resp,
                    op_tiper.0,
                    |_| {
                        dir_state.note_resource.refetch();
                        open.set(false);
                        app_state.success("修改成功。");
                    },
                );
            });
        }>
            <fieldset class="p-4 w-full fieldset">
                <label class="label">名称</label>
                <input
                    type="text"
                    class="w-full input validator"
                    minlength="1"
                    maxlength="50"
                    required
                    bind:value=form.label
                />
                <p class="validator-hint">必须输入</p>
                <OpTip content=op_tiper.0 />
                <button class="mt-4 btn btn-neutral" type="submit">
                    确定
                </button>
            </fieldset>
        </form>
    }
}

#[derive(Debug, Clone, Copy)]
struct UpdateDirFormData {
    id: RwSignal<String>,
    label: RwSignal<String>,
}
impl UpdateDirFormData {
    fn new(note: NoteDir) -> Self {
        Self {
            id: RwSignal::new(note.id),
            label: RwSignal::new(note.label),
        }
    }
}
