use leptos::prelude::*;
use leptos_router::{components::*, path};
use tower::{
    common::App,
    tauri_web::{
        page::component::{Header, LeftSideBar},
        AppState,
    },
    web::component::OpTip,
    web_model::MenuData,
};

use crate::page::{HotelRoomPane, NotePane, PasswordPane, WishingPane};

#[component]
pub fn App() -> impl IntoView {
    let state = AppState::new();
    provide_context(state);
    state.startup();
    view! {
        <Router>
            <div class="flex gap-4">
                <LeftSideBar current_app=App::TowerProjectWebsite menus=get_menus() />
                <div class="flex relative flex-col gap-2 w-full">
                    <Header />
                    <OpTip content=state.op_tip />
                    <Routes fallback=|| "即将上线.">
                        <Route path=path!("/wishing") view=WishingPane />
                        <Route path=path!("/hotel/chat") view=HotelRoomPane />
                        <Route path=path!("/note") view=NotePane />
                        <Route path=path!("/password") view=PasswordPane />
                    </Routes>
                </div>
            </div>
        </Router>
    }
}

fn get_menus() -> MenuData {
    MenuData::new("/", "第五塔灵", icondata::AiHeartOutlined)
        .add_child_menu(
            MenuData::new("", "酒馆", icondata::AiAccountBookOutlined)
                .add_child_menu(MenuData::new(
                    "/person",
                    "找人",
                    icondata::AiUsergroupAddOutlined,
                ))
                .add_child_menu(MenuData::new(
                    "/task",
                    "接活",
                    icondata::AiPropertySafetyOutlined,
                ))
                .add_child_menu(MenuData::new(
                    "/hotel/chat",
                    "唠嗑",
                    icondata::AiCommentOutlined,
                ))
                .to_owned(),
        )
        .add_child_menu(
            MenuData::new("", "工具", icondata::AiCarOutlined)
                .add_child_menu(MenuData::new("/note", "记事本", icondata::AiFormOutlined))
                .add_child_menu(MenuData::new(
                    "/password",
                    "密码本",
                    icondata::AiLockOutlined,
                ))
                .to_owned(),
        )
        .add_child_menu(MenuData::new(
            "/wishing",
            "许愿墙",
            icondata::AiHeartOutlined,
        ))
        .to_owned()
}
