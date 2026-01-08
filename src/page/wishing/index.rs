use std::collections::HashMap;

use crate::page::get_padding_by_shape;
use crate::page::WishingInput;
use leptos::prelude::*;
use tower::script_model::TowerScriptResource;
use tower::script_model::WishingItemData;
use tower::tauri_web::prelude::async_http_without_token_and;
use tower::web::prelude::*;
#[component]
pub fn WishingPane() -> impl IntoView {
    let wishings = LocalResource::new(async move || {
        let ret: Result<Vec<WishingItemData>, ApiError> = async_http_without_token_and(
            App::TowerScriptServer,
            "get",
            &TowerScriptResource::WishingWall.path(ApiMethod::List),
            HashMap::<String, String>::new(),
        )
        .await;
        ret
    });
    view! {
        <WishingList wishings />
        <WishingInput />
        <button
            class="absolute bottom-1 right-15 btn"
            on:click=move |_| {
                wishings.refetch();
            }
        >
            换一批
        </button>
    }
}

#[component]
fn WishingList(wishings: LocalResource<Result<Vec<WishingItemData>, ApiError>>) -> impl IntoView {
    view! {
        <Suspense fallback=move || {
            view! { <span class="self-center loading loading-spinner loading-xl"></span> }
        }>
            <div class="flex flex-wrap gap-8 p-8 min-w-4/5 min-h-4/5">
                {move || {
                    wishings
                        .to_view(|wishings| {
                            wishings
                                .iter()
                                .map(|wishing| {
                                    view! { <WishingItem data=wishing.to_owned() /> }
                                })
                                .collect_view()
                                .into_any()
                        })
                }}
            </div>
        </Suspense>
    }
}

#[component]
fn WishingItem(data: WishingItemData) -> impl IntoView {
    let WishingItemData {
        wishing,
        bg_color,
        color,
        shape,
    } = data;
    view! {
        <div class=move || {
            let padding = get_padding_by_shape(&shape);
            format!(
                "{} {} mask mask-{} {} text-xs  flex items-center hover:text-sm",
                padding,
                bg_color,
                shape,
                color,
            )
        }>{wishing}</div>
    }
}

#[cfg(test)]
mod tests {}
