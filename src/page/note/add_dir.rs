use super::*;
use leptos::logging::log;
use leptos::{prelude::*, task::spawn_local};
use tower::tauri_model::note::NoteDir;
use tower::tauri_model::StoreType;
use tower::tauri_web::prelude::*;
use tower::tauri_web::service::store::async_save_store;
use tower::tauri_web::AppState;

#[component]
pub(crate) fn AddDirForm(open: RwSignal<bool>) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let dir_state = expect_context::<NoteState>();
    let form = AddDirFormData::new(dir_state.current_note.get_untracked());
    let op_tiper = OpTiper::new();
    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            spawn_local(async move {
                let mut root = dir_state.current_root.get_untracked().unwrap();
                let file_id = root.id.clone();
                root.visit_mut(
                    &mut |n| {
                        let note = form.to_req();
                        if n.id.eq(&note.parent_id.clone().unwrap_or_default()) {
                            n.add_child(note);
                            return true;
                        }
                        return false;
                    },
                );
                log!("root:{:?}",root);
                let mut file = dir_state.current_file.get_untracked().unwrap();
                file.content = bincode_encode(vec![root]);
                let resp = async_save_store(StoreType::Note, &file_id, &bincode_encode(file)).await;
                tip_or(
                    resp,
                    op_tiper.0,
                    |_| {
                        dir_state.note_resource.refetch();
                        open.set(false);
                        app_state.success("新增成功。");
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
                <label class="label">上级目录</label>
                <input
                    type="text"
                    class="w-full input validator"
                    minlength="1"
                    maxlength="50"
                    disabled
                    required
                    prop:value=form.parent_label
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
struct AddDirFormData {
    label: RwSignal<String>,
    parent_id: RwSignal<String>,
    parent_label: RwSignal<String>,
}
impl AddDirFormData {
    fn new(parent: Option<NoteDir>) -> Self {
        let now = now_and("yyyy-mm-dd");
        let (parent_id, parent_label) = parent.map(|d| (d.id, d.label)).unzip();
        Self {
            label: RwSignal::new(now),
            parent_id: RwSignal::new(parent_id.unwrap_or_default()),
            parent_label: RwSignal::new(parent_label.unwrap_or_default()),
        }
    }

    fn to_req(&self) -> NoteDir {
        let id = random_id(6);
        NoteDir::new(
            id,
            str_to_option(self.parent_id.get_untracked()),
            self.label.get_untracked(),
        )
    }
}
