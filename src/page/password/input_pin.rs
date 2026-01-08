use super::*;
use leptos::{prelude::*, task::spawn_local};
use tower::tauri_web::prelude::*;

#[component]
pub(crate) fn InputPinForm(open: RwSignal<bool>) -> impl IntoView {
    let password_state = expect_context::<PasswordState>();
    let op_tiper = OpTiper::new();
    view! {
        {move || {
            password_state
                .extra_resource
                .to_view(|pin| {
                    let pin = pin.as_ref().map_or("".to_string(), |pin| { pin.0.to_string() });
                    let form = InputPinFormData::new(pin);

                    view! {
                        <form on:submit=move |ev| {
                            ev.prevent_default();
                            spawn_local(async move {
                                if !form.correct_pin() {
                                    op_tiper.warning("Pin码不正确");
                                    return;
                                }
                                open.set(false);
                            });
                        }>
                            <fieldset class="p-4 border fieldset bg-base-200 border-base-300 rounded-box w-xs">
                                <legend class="fieldset-legend">输入Pin码</legend>
                                <div class="join">
                                    <input
                                        type="password"
                                        class="input join-item"
                                        placeholder="Pin码"
                                        bind:value=form.input_pin
                                    />
                                    <button class="btn join-item" type="submit">
                                        确定
                                    </button>
                                </div>
                                <OpTip content=op_tiper.0 />
                            </fieldset>
                        </form>
                    }
                        .into_any()
                })
        }}
    }
}

#[derive(Clone, Copy)]
struct InputPinFormData {
    pin: Signal<String>,
    input_pin: RwSignal<String>,
}
impl InputPinFormData {
    fn new(pin: String) -> Self {
        Self {
            pin: Signal::from(pin),
            input_pin: RwSignal::new("".to_string()),
        }
    }
    fn correct_pin(&self) -> bool {
        self.pin.get_untracked().eq(&self.input_pin.get_untracked())
    }
}
