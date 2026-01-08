use serde_json::json;
use serde_wasm_bindgen::to_value;
use tower::{
    common::{dict::VipLevelItem, ApiError},
    tauri_web::ipc::{try_invoke_for_json, try_invoke_for_json_without_args},
};

///校验当前许愿是否已到最大
pub async fn async_check_wishing_num() -> Result<(), ApiError> {
    try_invoke_for_json_without_args("check_wishing_num").await
}

///获取聊天室允许的最大人数
pub async fn async_get_vip_level_item_num(item: VipLevelItem) -> Result<usize, ApiError> {
    let args = to_value(&json!({"item":item})).unwrap();
    try_invoke_for_json("get_vip_level_item_num", args).await
}
