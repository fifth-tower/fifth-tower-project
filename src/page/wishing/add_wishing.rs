use leptos::{prelude::*, task::spawn_local};
use leptos_icons::Icon;
use tower::script_model::TowerScriptResource;
use tower::tauri_web::prelude::async_http;
use tower::tauri_web::AppState;
use tower::web::common::to_href;
use tower::web::prelude::*;
use tower::web_model::Tipable;
use tower::{script_model::AddWishingReq, web::component::Dialog};

use crate::service::async_check_wishing_num;

#[component]
pub(crate) fn WishingInput() -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let open_wishing = RwSignal::new(false);
    view! {
        <button
            class="absolute right-1 bottom-1 btn btn-circle"
            on:click=move |_| {
                spawn_local(async move {
                    let ret = async_check_wishing_num().await;
                    match ret {
                        Ok(_) => {
                            open_wishing.set(true);
                        }
                        Err(ApiError::Custom(msg)) => {
                            app_state.warning(msg);
                        }
                        Err(err) => {
                            app_state.error(err.to_string());
                        }
                    }
                });
            }
        >
            <Icon icon=icondata::AiPlusOutlined />
        </button>
        <Dialog title="许愿" open=open_wishing class="max-w-2xl w-160">
            <WishingForm />
        </Dialog>
    }
}

#[component]
fn WishingForm() -> impl IntoView {
    let form = WishingFormData::new();
    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            let data = form.to_req();
            spawn_local(async move {
                let ret: ApiResult<()> = async_http(
                        App::TowerScriptServer,
                        "post",
                        &TowerScriptResource::WishingWall.path(ApiMethod::Insert),
                        data,
                    )
                    .await;
                ret.map(|_| to_href("/wishing")).unwrap();
            });
        }>
            <label class="floating-label">
                <span>祝愿词</span>
                <input
                    type="text"
                    placeholder="愿天下人皆有出路"
                    class="w-full input input-md validator"
                    pattern=".{6,20}"
                    minlength="6"
                    maxlength="20"
                    title="6-20位"
                    required
                    bind:value=form.wishing
                />
                <p class="validator-hint">"必须输入，6-20位"</p>
            </label>
            <div class="flex gap-4 mb-2">
                <button class="flex-none w-32 btn" on:click=move |_| {}>
                    设置形状
                </button>
                <ShapePicker form />
            </div>
            <div class="flex gap-4">
                <div class="flex flex-col flex-none gap-2 w-32">
                    <button
                        class="btn"
                        on:click=move |_| {
                            form.is_bg_color.set(false);
                        }
                    >
                        设置前景色
                    </button>
                    <button
                        class="btn"
                        on:click=move |_| {
                            form.is_bg_color.set(true);
                        }
                    >
                        设置背景色
                    </button>
                    {move || {
                        view! { <WishingItem form /> }
                    }}
                </div>
                {move || {
                    view! { <ColorPicker form /> }
                }}
            </div>
            <div class="modal-action">
                <button class="mt-4 btn btn-neutral" type="submit">
                    确定
                </button>
            </div>
        </form>
    }
}

#[derive(Debug, Clone, Copy)]
struct WishingFormData {
    bg_color: RwSignal<String>,
    color: RwSignal<String>,
    wishing: RwSignal<String>,
    shape: RwSignal<String>,
    is_bg_color: RwSignal<bool>,
}

impl WishingFormData {
    fn new() -> Self {
        Self {
            bg_color: RwSignal::new("bg-rose-300".into()),
            color: RwSignal::new("text-neutral-900".into()),
            wishing: RwSignal::new("".into()),
            shape: RwSignal::new("squircle".into()),
            is_bg_color: RwSignal::new(false),
        }
    }

    fn to_req(&self) -> AddWishingReq {
        AddWishingReq {
            wishing: self.wishing.get_untracked(),
            bg_color: self.bg_color.get_untracked(),
            color: self.color.get_untracked(),
            shape: self.shape.get_untracked(),
        }
    }
    fn set_color(&self, color: &str) {
        if self.is_bg_color.get_untracked() {
            self.bg_color.set(format!("bg-{}", color));
        } else {
            self.color.set(format!("text-{}", color));
        }
    }
}

#[component]
fn WishingItem(form: WishingFormData) -> impl IntoView {
    view! {
        <div class=move || {
            let shape = form.shape.get();
            let padding = get_padding_by_shape(&shape);
            format!(
                "{} {} mask mask-{} {} text-xs  flex items-center",
                padding,
                form.bg_color.get(),
                shape,
                form.color.get(),
            )
        }>{move || form.wishing.get()}</div>
    }
}
pub fn get_padding_by_shape(shape: &str) -> String {
    match shape {
        "squircle" => "p-4 size-24 hover:size-32",
        "heart" => "p-4 size-30 hover:size-32",
        "hexagon" => "p-4 size-24 hover:size-32",
        "hexagon-2" => "p-4 size-24 hover:size-32",
        "decagon" => "p-4 size-24 hover:size-32",
        "diamond" => "p-6.5 size-30 hover:size-32",
        "square" => "p-4 size-24 hover:size-32",
        "circle" => "p-4 size-24 hover:size-32",
        _ => "",
    }
    .into()
}
#[component]
fn ShapePicker(form: WishingFormData) -> impl IntoView {
    let shapes = [
        "squircle",
        "heart",
        "hexagon",
        "hexagon-2",
        "decagon",
        "diamond",
        "square",
        "circle",
    ];
    view! {
        <div class="flex gap-4 justify-center">
            {shapes
                .into_iter()
                .map(|shape| {
                    view! {
                        <div
                            class=format!("bg-red-400 mask mask-{} size-8 cursor-pointer", shape)
                            on:click=move |_| {
                                form.shape.set(shape.to_string());
                            }
                        ></div>
                    }
                })
                .collect_view()}

        </div>
    }
}
#[component]
fn ColorPicker(form: WishingFormData) -> impl IntoView {
    let colors = ["red", "yellow", "teal", "blue", "purple", "rose", "neutral"];
    let depths = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];

    view! {
        <div class="flex flex-col gap-2">
            {colors
                .iter()
                .map(|color| {
                    view! {
                        <div class="flex gap-2">
                            {depths
                                .iter()
                                .map(|depth| {
                                    let color = format!("{}-{}", color, depth);
                                    view! {
                                        <div
                                            class=format!("bg-{} size-8 cursor-pointer", color)
                                            on:click=move |_| {
                                                form.set_color(&color);
                                            }
                                        ></div>
                                    }
                                })
                                .collect_view()}
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_log_colors() {
        let colors = ["red", "yellow", "teal", "blue", "purple", "rose", "neutral"];
        let depths = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];

        let log = colors
            .iter()
            .map(|color| {
                depths
                    .iter()
                    .map(|depth| {
                        let color = format!("{}-{}", color, depth);

                        format!("bg-{} text-{}", color, color)
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{:?}", log);
    }
    #[test]
    fn test_log_shapes() {
        let shapes = [
            "squircle",
            "heart",
            "hexagon",
            "hexagon-2",
            "decagon",
            "diamond",
            "square",
            "circle",
        ];

        let log = shapes
            .into_iter()
            .map(|shape| format!("mask-{}", shape))
            .collect::<Vec<String>>()
            .join(" ");
        println!("{:?}", log);
    }
}
