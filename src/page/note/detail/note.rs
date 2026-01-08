use leptos::prelude::*;
use tower::{tauri_model::note::*, tauri_web::prelude::*};

use super::*;

#[component]
pub(crate) fn Note(index: usize, data: NoteText) -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let note_state = expect_context::<NoteState>();
    let title = RwSignal::new(data.title);
    let content = RwSignal::new(data.content);
    //true meanings add,otherwise update
    let add_or_update = move || {
        let notes = note_state.current_note.get().unwrap();
        index == notes.data.len()
    };
    view! {
        <div class="shadow-sm w-92 card bg-base-100">
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
                    <textarea
                        class="textarea textarea-ghost"
                        placeholder="请输入内容"
                        rows="5"
                        on:input:target=move |ev| { content.set(ev.target().value()) }
                    >
                        {move || content.get()}
                    </textarea>
                </p>
                <div class="hidden justify-end group-hover:flex card-actions">
                    <button
                        class="btn"
                        on:click=move |_| {
                            save_text(
                                app_state,
                                note_state,
                                NoteText {
                                    title: title.get_untracked(),
                                    content: content.get_untracked(),
                                },
                                index,
                                add_or_update(),
                            );
                        }
                    >
                        保存
                    </button>
                    <Show when=move || { !add_or_update() }>
                        <button
                            class="btn"
                            on:click=move |_| {
                                delete_text(app_state, note_state, index);
                            }
                        >
                            删除
                        </button>
                    </Show>
                </div>
            </div>
        </div>
    }
}
