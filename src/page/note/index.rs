use super::*;
use leptos::prelude::*;
use tower::{
    tauri_model::{note::*, StoreFile, StoreRootData, StoreType},
    tauri_web::prelude::*,
    tauri_web::service::store::async_load_stores,
};

#[component]
pub fn NotePane() -> impl IntoView {
    let note_resource = LocalResource::new(move || async_load_stores(StoreType::Note));
    let state = NoteState::new(note_resource);
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
                note_resource
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
        </Suspense>
    }
}

#[derive(Clone, Copy)]
pub(crate) struct NoteState {
    pub current_file: RwSignal<Option<StoreFile>>,
    pub current_root: RwSignal<Option<NoteDir>>,
    pub current_note: RwSignal<Option<NoteDir>>,
    pub open_add_file: RwSignal<bool>,
    pub open_add_dir: RwSignal<bool>,
    pub open_update_dir: RwSignal<bool>,
    pub note_resource: WebResult<StoreRootData>,
}

impl NoteState {
    pub fn new(note_resource: WebResult<StoreRootData>) -> Self {
        let current_file = RwSignal::new(None);
        let current_root = RwSignal::new(None);
        let current_note = RwSignal::new(None);
        let open_add_file = RwSignal::new(false);
        let open_add_dir = RwSignal::new(false);
        let open_update_dir = RwSignal::new(false);

        Self {
            current_file,
            current_note,
            current_root,
            open_add_file,
            open_add_dir,
            open_update_dir,
            note_resource,
        }
    }

    pub fn set_current(
        &self,
        file: Option<StoreFile>,
        root: Option<NoteDir>,
        note: Option<NoteDir>,
    ) {
        self.current_file.set(file);
        self.current_root.set(root);
        self.current_note.set(note);
    }
}
