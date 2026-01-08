use super::*;
use leptos::{prelude::*, task::spawn_local};
use tower::tauri_model::StoreType;
use tower::tauri_web::prelude::*;
use tower::tauri_web::service::store::async_save_store;
use tower::tauri_web::AppState;

#[component]
pub(crate) fn UpdateFileForm(open: RwSignal<bool>) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let password_state = expect_context::<PasswordState>();
    let file = password_state.current_file.get_untracked().unwrap();
    let form = UpdatePasswordFormData::new(file.file_id, file.label);
    let op_tiper = OpTiper::new();
    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            spawn_local(async move {
                let mut file = password_state.current_file.get_untracked().unwrap();
                file.content = bincode_encode(password_state.current_data.get_untracked());
                file.label = form.label.get_untracked();
                let resp = async_save_store(
                        StoreType::Password,
                        &file.file_id.clone(),
                        &bincode_encode(file),
                    )
                    .await;
                tip_or(
                    resp,
                    op_tiper.0,
                    |_| {
                        password_state.password_resource.refetch();
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
struct UpdatePasswordFormData {
    file_id: RwSignal<String>,
    label: RwSignal<String>,
}
impl UpdatePasswordFormData {
    fn new(file_id: String, label: String) -> Self {
        Self {
            file_id: RwSignal::new(file_id),
            label: RwSignal::new(label),
        }
    }
}
