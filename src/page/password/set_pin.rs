use super::*;
use leptos::logging::log;
use leptos::{prelude::*, task::spawn_local};
use tower::tauri_model::StoreType;
use tower::tauri_web::prelude::*;
use tower::tauri_web::service::store::async_set_store_extra_info;
use tower::tauri_web::AppState;

#[component]
pub(crate) fn SetPinForm(open: RwSignal<bool>) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let password_state = expect_context::<PasswordState>();
    let op_tiper = OpTiper::new();
    view! {
        <Suspense fallback=move || {
            view! {
                <div class="content-center w-full text-center min-h-96">
                    <span class="loading loading-spinner loading-xl"></span>
                </div>
            }
        }>
            {move || {
                password_state
                    .extra_resource
                    .to_view(|pin| {
                        let pin = pin.as_ref().map_or("".to_string(), |pin| { pin.0.to_string() });
                        let form = SetPinFormData::new(pin);

                        view! {
                            <form on:submit=move |ev| {
                                ev.prevent_default();
                                spawn_local(async move {
                                    if !form.correct_pin() {
                                        op_tiper.warning("Pin码不正确");
                                        return;
                                    }
                                    if !form.confirm_correct() {
                                        op_tiper.warning("两次输入新Pin码不一样");
                                        return;
                                    }
                                    let resp = async_set_store_extra_info(
                                            StoreType::Password,
                                            Some(
                                                bincode_encode((form.new_pin.get_untracked(), now() as i64)),
                                            ),
                                        )
                                        .await;
                                    tip_or(
                                        resp,
                                        op_tiper.0,
                                        |_| {
                                            password_state.password_resource.refetch();
                                            password_state.extra_resource.refetch();
                                            open.set(false);
                                            app_state.success("设置成功。");
                                        },
                                    );
                                });
                            }>
                                <fieldset class="p-4 w-full fieldset">
                                    <label class="label">旧Pin码</label>
                                    <input
                                        type="password"
                                        class="w-full input validator"
                                        minlength="0"
                                        maxlength="50"
                                        bind:value=form.old_pin
                                    />
                                    <p class="validator-hint">必须输入</p>
                                    <label class="label">新Pin码</label>
                                    <input
                                        type="password"
                                        class="w-full input validator"
                                        minlength="4"
                                        maxlength="4"
                                        pattern=".{4,4}"
                                        required
                                        bind:value=form.new_pin
                                    />
                                    <p class="validator-hint">必须输入.4位</p>
                                    <label class="label">确认新Pin码</label>
                                    <input
                                        type="password"
                                        class="w-full input validator"
                                        minlength="4"
                                        maxlength="4"
                                        pattern=".{4,4}"
                                        required
                                        bind:value=form.new_pin_confirm
                                    />
                                    <p class="validator-hint">必须输入.4位</p>
                                    <OpTip content=op_tiper.0 />
                                    <button class="mt-4 btn btn-neutral" type="submit">
                                        确定
                                    </button>
                                </fieldset>
                            </form>
                        }
                            .into_any()
                    })
            }}

        </Suspense>
    }
}

#[derive(Clone, Copy)]
struct SetPinFormData {
    pin: Signal<String>,
    old_pin: RwSignal<String>,
    new_pin: RwSignal<String>,
    new_pin_confirm: RwSignal<String>,
}
impl SetPinFormData {
    fn new(pin: String) -> Self {
        Self {
            pin: Signal::from(pin),
            old_pin: RwSignal::new("".to_string()),
            new_pin: RwSignal::new("".to_string()),
            new_pin_confirm: RwSignal::new("".to_string()),
        }
    }
    fn correct_pin(&self) -> bool {
        self.pin.get_untracked().eq(&self.old_pin.get_untracked())
    }
    fn confirm_correct(&self) -> bool {
        self.new_pin
            .get_untracked()
            .eq(&self.new_pin_confirm.get_untracked())
    }
}
