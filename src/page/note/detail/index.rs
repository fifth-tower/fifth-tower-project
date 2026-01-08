use leptos::{logging::log, prelude::*, task::spawn_local};
use tower::{
    tauri_model::{note::NoteText, StoreType},
    tauri_web::{prelude::*, service::store::async_save_store},
};

use super::*;

#[component]
pub(crate) fn Detail() -> impl IntoView {
    let note_state = expect_context::<NoteState>();

    view! {
        <div class="flex flex-wrap flex-1 gap-4 p-4 bg-base-200 min-h-96">
            <Show when=move || {
                note_state.current_note.get().is_some()
            }>
                {move || {
                    let notes = note_state.current_note.get().unwrap();
                    notes
                        .data
                        .iter()
                        .enumerate()
                        .map(|(index, note)| {
                            view! { <Note index data=note.clone() /> }
                        })
                        .collect_view()
                }}
                <Note
                    index={
                        let notes = note_state.current_note.get().unwrap();
                        notes.data.len()
                    }
                    data=NoteText {
                        title: "".to_string(),
                        content: "".to_string(),
                    }
                />
            </Show>
        </div>
    }
}

pub(crate) fn save_text(
    app_state: AppState,
    note_state: NoteState,
    add_text: NoteText,
    index: usize,
    add_or_update: bool,
) {
    let mut root = note_state.current_root.get_untracked().unwrap();
    let mut note = note_state.current_note.get_untracked().unwrap();
    let file_id = root.id.clone();

    log!("add_or_update:{:?}", add_or_update);
    //(is_root, add_or_update)
    match (note.id.eq(&root.id), add_or_update) {
        //(root, add)
        (true, true) => root.add_text(add_text.clone()),
        (true, false) => root.update_text(index, add_text.clone()),
        (false, true) => {
            root.visit_mut(&mut |n| {
                if n.id.eq(&note.id) {
                    n.add_text(add_text.clone());
                    return true;
                }
                return false;
            });
        }
        //(not_root, update)
        (false, false) => {
            root.visit_mut(&mut |n| {
                if n.id.eq(&note.id) {
                    n.update_text(index, add_text.clone());
                    return true;
                }
                return false;
            });
        }
    };
    log!("root:{:?}", root);
    spawn_local(async move {
        let mut file = note_state.current_file.get_untracked().unwrap();
        log!("file:{:?}", file);
        file.content = bincode_encode(vec![root.clone()]);
        let resp = async_save_store(StoreType::Note, &file_id, &bincode_encode(file)).await;
        match resp {
            Ok(_) => {
                app_state.success("保存成功。");
                note.add_text(add_text);
                note_state.current_root.set(Some(root));
                note_state.current_note.set(Some(note));
                note_state.note_resource.refetch();
            }
            Err(err) => app_state.error(err.to_string()),
        }
    });
}

pub(crate) fn delete_text(app_state: AppState, note_state: NoteState, delete_index: usize) {
    let mut root = note_state.current_root.get_untracked().unwrap();
    let mut note = note_state.current_note.get_untracked().unwrap();
    let file_id = root.id.clone();
    root.visit_mut(&mut |n| {
        if n.id.eq(&note.id) {
            n.remove_text(delete_index);
            return true;
        }
        return false;
    });
    spawn_local(async move {
        let mut file = note_state.current_file.get_untracked().unwrap();
        file.content = bincode_encode(vec![root.clone()]);
        let resp = async_save_store(StoreType::Note, &file_id, &bincode_encode(file)).await;
        match resp {
            Ok(_) => {
                app_state.success("删除成功。");
                note.remove_text(delete_index);
                note_state.current_root.set(Some(root));
                note_state.current_note.set(Some(note));
                note_state.note_resource.refetch();
            }
            Err(err) => app_state.error(err.to_string()),
        }
    });
}
