use super::*;
use crate::page::ChatState;
use leptos::prelude::*;
use tower::{tauri_model::ChatModule, common::random_id};

#[component]
pub fn SelectOrJoinStep() -> impl IntoView {
    let state = expect_context::<ChatState>();
    let room = RwSignal::new("".to_string());
    view! {
        <div class="flex gap-8 items-center">
            <fieldset class="p-4 border fieldset bg-base-200 border-base-300 rounded-box w-xs">
                <div class="join">
                    <label class="w-96 floating-label">
                        <span>房号</span>
                        <input
                            type="text"
                            placeholder="房号"
                            class="input input-md"
                            bind:value=room
                        />
                    </label>
                    <button
                        class="btn join-item"
                        on:click=move |_| {
                            let room = room.get_untracked();
                            let room = room.trim();
                            if room.is_empty() {
                                return;
                            }
                            state.step.set(ChatStep::Started(room.to_string()));
                        }
                    >
                        进入房间
                    </button>
                </div>
            </fieldset>
            <div class="divider divider-horizontal">或</div>
            <button
                class="btn"
                on:click=move |_| {
                    let room = ChatModule::HotelRoom.room(&random_id(5));
                    state.step.set(ChatStep::Started(room));
                }
            >
                创建房间
            </button>
        </div>
    }
}
