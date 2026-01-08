use leptos::{prelude::*, task::spawn_local};
use tower::{
    tauri_model::{password::*, StoreType},
    tauri_web::{prelude::*, service::store::async_save_store},
};

use super::*;

#[component]
pub(crate) fn Detail() -> impl IntoView {
    let password_state = expect_context::<PasswordState>();

    view! {
        <div class="flex flex-wrap flex-1 gap-4 p-4 bg-base-200 min-h-96">
            <Show when=move || {
                password_state.current_file.get().is_some()
            }>
                {move || {
                    let passwords = password_state.current_data.get();
                    passwords
                        .iter()
                        .enumerate()
                        .map(|(index, password)| {
                            view! { <Password index data=password.clone() /> }
                        })
                        .collect_view()
                }}
                <Password
                    index={
                        let passwords = password_state.current_data.get();
                        passwords.len()
                    }
                    data=PasswordItem {
                        title: "".to_string(),
                        username: "".to_string(),
                        password: "".to_string(),
                        remark: "".to_string(),
                    }
                />
            </Show>
        </div>
    }
}

#[component]
pub(crate) fn Password(index: usize, data: PasswordItem) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let password_state = expect_context::<PasswordState>();
    let title = RwSignal::new(data.title);
    let username = RwSignal::new(data.username);
    let password = RwSignal::new(data.password);
    let remark = RwSignal::new(data.remark);
    view! {
        <div class="w-64 shadow-sm card bg-base-100">
            <div class="card-body group">
                <h2 class="card-title">
                    <input
                        type="text"
                        bind:value=title
                        class="input input-ghost"
                        placeholder="请输入标题"
                    />
                </h2>
                <p>
                    <label class="input input-ghost">
                        <kbd class="kbd kbd-sm">账号</kbd>
                        <input
                            type="text"
                            bind:value=username
                            class="grow"
                            placeholder="请输入账号"
                        />
                    </label>
                    <label class="input input-ghost">
                        <kbd class="kbd kbd-sm">密码</kbd>
                        <input
                            type="text"
                            bind:value=password
                            class="grow"
                            placeholder="请输入密码"
                        />
                    </label>
                    <textarea
                        class="mt-2 textarea textarea-ghost"
                        placeholder="请输入内容"
                        on:input:target=move |ev| { remark.set(ev.target().value()) }
                    >
                        {move || remark.get()}
                    </textarea>
                </p>
                <div class="hidden justify-end group-hover:flex card-actions">
                    <button
                        class="btn btn-xs"
                        on:click=move |_| {
                            add_password(
                                app_state,
                                password_state,
                                PasswordItem {
                                    title: title.get_untracked(),
                                    username: username.get_untracked(),
                                    password: password.get_untracked(),
                                    remark: remark.get_untracked(),
                                },
                            );
                        }
                    >
                        保存
                    </button>
                    <button
                        class="btn btn-xs"
                        on:click=move |_| {
                            delete_text(app_state, password_state, index);
                        }
                    >
                        删除
                    </button>
                </div>
            </div>
        </div>
    }
}

pub(crate) fn add_password(app_state: AppState, password_state: PasswordState, add: PasswordItem) {
    spawn_local(async move {
        let mut file = password_state.current_file.get_untracked().unwrap();

        let mut data = password_state.current_data.get_untracked();
        data.push(add);
        file.content = bincode_encode(&data);
        let resp = async_save_store(
            StoreType::Password,
            &file.file_id.clone(),
            &bincode_encode(file),
        )
        .await;
        match resp {
            Ok(_) => {
                app_state.success("保存成功。");
                password_state.current_data.set(data);
                password_state.password_resource.refetch();
            }
            Err(err) => app_state.error(err.to_string()),
        }
    });
}

pub(crate) fn delete_text(app_state: AppState, password_state: PasswordState, delete_index: usize) {
    spawn_local(async move {
        let mut file = password_state.current_file.get_untracked().unwrap();

        let mut data = password_state.current_data.get_untracked();
        data.remove(delete_index);
        file.content = bincode_encode(data);
        let resp = async_save_store(
            StoreType::Password,
            &file.file_id.clone(),
            &bincode_encode(file),
        )
        .await;
        match resp {
            Ok(_) => {
                app_state.success("删除成功。");
                password_state.password_resource.refetch();
            }
            Err(err) => app_state.error(err.to_string()),
        }
    });
}
