use super::*;
use leptos::prelude::*;
use tower::{
    tauri_model::{password::*, StoreFile, StoreRootData, StoreType},
    tauri_web::{
        prelude::*,
        service::store::{async_get_store_extra_info, async_load_stores},
    },
};

#[component]
pub fn PasswordPane() -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let password_resource = LocalResource::new(move || async_load_stores(StoreType::Password));
    let extra_resource =
        LocalResource::new(async move || async_get_store_extra_info(StoreType::Password).await);
    let state = PasswordState::new(password_resource, extra_resource);
    provide_context(state);

    view! {
        <Suspense fallback=move || {
            view! {
                <div class="content-center w-full text-center min-h-96">
                    <span class="loading loading-spinner loading-xl"></span>
                </div>
            }
        }>
            {move || {
                state
                    .extra_resource
                    .apply(
                        &app_state,
                        |pin| {
                            if pin.is_some() {
                                state.open_input_pin.set(true);
                            }
                        },
                    )
            }} <Show when=move || { state.open_input_pin.get() }>
                <div class="flex justify-center items-center w-full min-h-96">
                    <InputPinForm open=state.open_input_pin />
                </div>
            </Show>
            <Show when=move || {
                !state.open_input_pin.get()
            }>
                {move || {
                    password_resource
                        .to_view(|_| {
                            view! {
                                <div class="flex gap-2 w-full">
                                    <LeftSide class="min-h-96" />
                                    <Detail />
                                </div>
                            }
                                .into_any()
                        })
                }}
            </Show>
        </Suspense>
    }
}

#[derive(Clone, Copy)]
pub(crate) struct PasswordState {
    pub current_file: RwSignal<Option<StoreFile>>,
    pub current_data: RwSignal<Vec<PasswordItem>>,
    pub open_add_file: RwSignal<bool>,
    pub open_update_file: RwSignal<bool>,
    pub open_set_pin: RwSignal<bool>,
    pub open_input_pin: RwSignal<bool>,
    pub password_resource: WebResult<StoreRootData>,
    pub extra_resource: WebResult<Option<(String, i64)>>,
}

impl PasswordState {
    pub fn new(
        password_resource: WebResult<StoreRootData>,
        extra_resource: WebResult<Option<(String, i64)>>,
    ) -> Self {
        let current_file = RwSignal::new(None);
        let current_data = RwSignal::new(vec![]);
        let open_add_file = RwSignal::new(false);
        let open_update_file = RwSignal::new(false);
        let open_set_pin = RwSignal::new(false);
        let open_input_pin = RwSignal::new(false);

        Self {
            current_file,
            current_data,
            open_add_file,
            open_update_file,
            open_set_pin,
            open_input_pin,
            password_resource,
            extra_resource,
        }
    }

    pub fn set_current(&self, file: Option<StoreFile>, data: Vec<PasswordItem>) {
        self.current_file.set(file);
        self.current_data.set(data);
    }
}
