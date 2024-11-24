use std::sync::{Arc, RwLock};

use crate::domain::method::builder::MethodBuilder;
use crate::domain::method::MethodType;
use crate::domain::tab::{Tab, TabBuilder, TabState, Tabs, WrappedTabState};
use tauri::{AppHandle, Emitter, State};

#[tauri::command]
#[specta::specta]
pub async fn new_tab<R: tauri::Runtime>(
    app: AppHandle<R>,
    all_tabs_state: State<'_, WrappedTabState>,
    new_tab_state: Option<TabState>,
    after: Option<u32>,
) -> Result<(), String> {
    let tabs = all_tabs_state.inner();
    let mut builder = TabBuilder::new();
    if let Some(s) = new_tab_state {
        builder = builder.state(s);
    }
    let new_tab = builder.build();
    tabs.lock().await.new_tab(new_tab, after);
    app.emit("tabs_updated", ()).unwrap();
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn new_method_builder<R: tauri::Runtime>(
    app: AppHandle<R>,
    all_tabs_state: State<'_, WrappedTabState>,
) -> Result<(), String> {
    let builder = MethodBuilder::new("New Method".to_string());
    let tabs = all_tabs_state.inner().lock().await;

    let tab = tabs.get_current().unwrap();
    let mut mut_tab = tab.write().unwrap();
    mut_tab.state = TabState::MethodBuilder(builder);
    mut_tab.saved = false;
    app.emit("tabs_updated", ()).unwrap();
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn update_tab<R: tauri::Runtime>(
    app: AppHandle<R>,
    all_tabs_state: State<'_, WrappedTabState>,
    id: String,
    new_tab_state: TabState,
) -> Result<(), String> {
    let tabs = all_tabs_state.inner().lock().await;

    if let Ok(index) = tabs.get_index_by_id(&id) {
        let mut tab = tabs[index].write().unwrap();
        tab.state = new_tab_state;
        tab.saved = false;
        app.emit("tabs_updated", ()).unwrap();
    }
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn update_tab_fire_and_forget<R: tauri::Runtime>(
    app: AppHandle<R>,
    all_tabs_state: State<'_, WrappedTabState>,
    id: String,
    new_tab_state: TabState,
) -> Result<(), String> {
    let tabs = all_tabs_state.inner().lock().await;

    if let Ok(index) = tabs.get_index_by_id(&id) {
        let mut tab = tabs[index].write().unwrap();
        tab.state = new_tab_state;
        if tab.saved {
            tab.saved = false;
            app.emit("tabs_updated", ()).unwrap();
        }
    }
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn set_current_tab_method<R: tauri::Runtime>(
    app: AppHandle<R>,
    all_tabs_state: State<'_, WrappedTabState>,
    method_type: MethodType,
) -> Result<(), String> {
    let tabs = all_tabs_state.inner().lock().await;
    tabs.get_current()?.write().unwrap().state = TabState::Method(method_type.method());

    app.emit("tabs_updated", ()).unwrap();
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_current_tab(
    tabs_state: State<'_, WrappedTabState>,
) -> Result<Arc<RwLock<Tab>>, String> {
    let tabs = tabs_state.inner();
    Ok(tabs.lock().await.get_current()?)
}

#[tauri::command]
#[specta::specta]
pub async fn delete_tab<R: tauri::Runtime>(
    app: AppHandle<R>,
    tabs_state: State<'_, WrappedTabState>,
    index: u32,
) -> Result<(), String> {
    let mut tabs = tabs_state.inner().lock().await;
    let current_tab_id = tabs.get_current().unwrap().read().unwrap().id.clone();
    let removed_id = tabs[index as usize].read().unwrap().id.clone();
    tabs.remove(index as usize);
    if tabs.len() == 0 {
        let t = TabBuilder::new().build();
        tabs.new_tab(t, None);
    }

    if current_tab_id == removed_id {
        let tabs_length = tabs.len();
        if index as usize >= tabs_length {
            tabs.set_current_by_index(tabs_length - 1).unwrap();
        } else {
            tabs.set_current_by_index(index as usize).unwrap();
        }
    }

    app.emit("tabs_updated", ()).unwrap();
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn set_current_tab<R: tauri::Runtime>(
    app: AppHandle<R>,
    id: String,
    tabs_state: State<'_, WrappedTabState>,
) -> Result<(), String> {
    let mut tabs = tabs_state.inner().lock().await;
    match tabs.set_current(id) {
        Ok(()) => {
            app.emit("tabs_updated", ()).unwrap();
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_tabs(
    tabs_state: State<'_, WrappedTabState>,
) -> Result<Vec<Arc<RwLock<Tab>>>, String> {
    let tabs = tabs_state.inner().lock().await.clone();
    Ok(tabs)
}

#[cfg(test)]
mod tests {
    use crate::domain::tab::{TabBuilder, Tabs};
    use std::sync::{Arc, RwLock};
    use tauri::Manager;

    #[tokio::test]
    async fn set_current_tab() {
        let (app, webview) = crate::test_app();

        let mut tabs = app
            .state::<crate::domain::tab::WrappedTabState>()
            .inner()
            .lock()
            .await;

        tabs.push(Arc::new(RwLock::new(TabBuilder::new().build())));
        tabs.push(Arc::new(RwLock::new(TabBuilder::new().build())));

        let second_tab_id = tabs[1].read().unwrap().id.clone();

        assert_eq!(
            tabs.get_current().unwrap().read().unwrap().id,
            tabs[0].read().unwrap().id
        );

        drop(tabs);

        tauri::test::assert_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "set_current_tab".into(),
                headers: tauri::http::header::HeaderMap::new(),
                url: tauri::Url::parse("http://localhost").unwrap(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                body: tauri::ipc::InvokeBody::Json(serde_json ::json!({ "id": second_tab_id })),
                invoke_key: tauri::test::INVOKE_KEY.into(),
            },
            Ok(()),
        );

        let tabs = app
            .state::<crate::domain::tab::WrappedTabState>()
            .inner()
            .lock()
            .await;

        assert_eq!(tabs[0].read().unwrap().current, false);
        assert_eq!(
            tabs.get_current().unwrap().clone().read().unwrap().id,
            tabs[1].read().unwrap().id
        );
    }

    #[tokio::test]
    async fn new_tab() {
        let (app, webview) = crate::test_app();

        tauri::test::assert_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "new_tab".into(),
                headers: tauri::http::header::HeaderMap::new(),
                url: tauri::Url::parse("http://localhost").unwrap(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                body: tauri::ipc::InvokeBody::Json(serde_json::Value::Null),
                invoke_key: tauri::test::INVOKE_KEY.into(),
            },
            Ok(()),
        );

        let tabs = app
            .state::<crate::domain::tab::WrappedTabState>()
            .inner()
            .lock()
            .await;

        assert_eq!(tabs.len(), 2);
        assert_eq!(
            tabs.get_current().unwrap().clone().read().unwrap().id,
            tabs[1].read().unwrap().id
        );
    }

    #[tokio::test]
    async fn get_current_tab() {
        let (app, webview) = crate::test_app();
        let mut tabs = app
            .state::<crate::domain::tab::WrappedTabState>()
            .inner()
            .lock()
            .await;

        tabs.push(Arc::new(RwLock::new(TabBuilder::new().build())));
        tabs.push(Arc::new(RwLock::new(TabBuilder::new().build())));

        let expected = tabs[0].clone();
        drop(tabs);

        tauri::test::assert_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "get_current_tab".into(),
                headers: tauri::http::header::HeaderMap::new(),
                url: tauri::Url::parse("http://localhost").unwrap(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                body: tauri::ipc::InvokeBody::Json(serde_json::Value::Null),
                invoke_key: tauri::test::INVOKE_KEY.into(),
            },
            Ok(expected),
        );
    }

    #[tokio::test]
    async fn get_tabs() {
        let (app, webview) = crate::test_app();
        let mut tabs = app
            .state::<crate::domain::tab::WrappedTabState>()
            .inner()
            .lock()
            .await;

        tabs.push(Arc::new(RwLock::new(TabBuilder::new().build())));
        tabs.push(Arc::new(RwLock::new(TabBuilder::new().build())));

        let expected = tabs.clone();
        drop(tabs);

        tauri::test::assert_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "get_tabs".into(),
                headers: tauri::http::header::HeaderMap::new(),
                url: tauri::Url::parse("http://localhost").unwrap(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                body: tauri::ipc::InvokeBody::Json(serde_json::Value::Null),
                invoke_key: tauri::test::INVOKE_KEY.into(),
            },
            Ok(expected),
        );
    }
}
