use crate::domain::tab::{TabState, Tabs, WrappedTabState};
use tauri::{AppHandle, Emitter, State};

#[tauri::command]
#[specta::specta]
pub async fn method_builder_add_metadata<R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    all_tabs_state: State<'_, WrappedTabState>,
) -> Result<(), String> {
    let tabs = all_tabs_state.inner().lock().await;
    match &mut tabs
        .get_current()
        .expect("Tab not found")
        .write()
        .unwrap()
        .state
    {
        TabState::MethodBuilder(builder) => {
            builder.add_metadata();
            app_handle.emit("tabs_updated", ()).unwrap();
        }
        _ => unreachable!(),
    };

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn method_builder_delete_metadata<R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    all_tabs_state: State<'_, WrappedTabState>,
    id: String,
) -> Result<(), String> {
    let tabs = all_tabs_state.inner().lock().await;
    match &mut tabs
        .get_current()
        .expect("Tab not found")
        .write()
        .unwrap()
        .state
    {
        TabState::MethodBuilder(builder) => {
            builder.delete_metadata(id);
            app_handle.emit("tabs_updated", ()).unwrap();
        }
        _ => unreachable!(),
    };

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn method_builder_update_metadata_name(
    all_tabs_state: State<'_, WrappedTabState>,
    id: String,
    name: String,
) -> Result<(), String> {
    let tabs = all_tabs_state.inner().lock().await;
    match &mut tabs
        .get_current()
        .expect("Tab not found")
        .write()
        .unwrap()
        .state
    {
        TabState::MethodBuilder(builder) => {
            builder.update_metadata_name(id, name);
        }
        _ => unreachable!(),
    };

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn method_builder_update_metadata_required(
    all_tabs_state: State<'_, WrappedTabState>,
    id: String,
    required: bool,
) -> Result<(), String> {
    let tabs = all_tabs_state.inner().lock().await;
    match &mut tabs
        .get_current()
        .expect("Tab not found")
        .write()
        .unwrap()
        .state
    {
        TabState::MethodBuilder(builder) => {
            builder.update_metadata_required(id, required);
        }
        _ => unreachable!(),
    };

    Ok(())
}
