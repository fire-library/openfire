use crate::domain::filesystem::Filetypes;
use crate::domain::tab::{TabBuilder, TabState, Tabs, WrappedTabState};
use std::fs::File;
use std::io::prelude::*;
use tauri::{AppHandle, Emitter, State};

#[tauri::command]
#[specta::specta]
pub async fn save<R: tauri::Runtime>(
    app: AppHandle<R>,
    all_tabs_state: State<'_, WrappedTabState>,
    filename: &str,
) -> Result<(), String> {
    let tabs = all_tabs_state.inner().lock().await;
    let current_tab = tabs.get_current()?;
    let mut current_tab = current_tab.write().unwrap();
    let tab_title = filename.split('/').last().unwrap().to_string();

    match current_tab.state.clone() {
        TabState::Method(output) => {
            let filetypes = Filetypes::Method(output.into());
            filetypes.save(filename)?;
            current_tab.saved = true;
            current_tab.title = Some(tab_title);
            app.emit("tabs_updated", {})
                .expect("Failed to emit tabs_updated");
            Ok(())
        }
        _ => Err("Cannot save to file".to_string()),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn open<R: tauri::Runtime>(
    app: AppHandle<R>,
    all_tabs_state: State<'_, WrappedTabState>,
    filename: &str,
) -> Result<(), String> {
    let mut tabs = all_tabs_state.inner().lock().await;

    if let Ok(tab) = tabs.get_by_filename(filename) {
        tabs.set_current(tab.read().unwrap().id.clone())
            .expect("Failed to set current tab");
        app.emit("tabs_updated", {})
            .expect("Failed to emit tabs_updated");
        return Ok(());
    }

    let yaml = File::open(filename).and_then(|mut f| {
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        Ok(contents)
    });
    let unwrapped_yaml = match yaml {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    let deserialized_map = serde_yaml::from_str::<Filetypes>(&unwrapped_yaml);

    let data = match deserialized_map {
        Ok(map) => map,
        Err(e) => return Err(e.to_string()),
    };

    let tab = TabBuilder::new()
        .state(data.into())
        .saved(true)
        .filename(filename.to_string())
        .build();

    tabs.new_tab(tab, None);
    app.emit("tabs_updated", {})
        .expect("Failed to emit tabs_updated");

    Ok(())
}
