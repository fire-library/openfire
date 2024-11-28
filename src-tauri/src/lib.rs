// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod domain;

use commands::forms::{
    calculate_form, get_equation_inputs, get_equation_inputs_symbols, get_equation_with_numbers,
    get_equation_with_symbols, update_field, ValidationErrorEvent,
};
use commands::method::builder::{
    method_builder_add_metadata, method_builder_delete_metadata,
    method_builder_update_metadata_name, method_builder_update_metadata_required,
};
use commands::tab::*;
use domain::tab::{NoCalc, TabBuilder, TabState, WrappedTabState};
use specta_typescript::Typescript;
use std::sync::{Arc, RwLock};
use tauri::test::MockRuntime;
use tauri::Manager;
use tauri::{Emitter, LogicalPosition, Position};
use tauri::{LogicalSize, Size};
use tauri_plugin_deep_link::DeepLinkExt;
use tauri_specta::{collect_commands, collect_events, Builder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init());

    let specta_builder = Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            new_tab::<tauri::Wry>,
            get_current_tab,
            get_tabs,
            set_current_tab::<tauri::Wry>,
            delete_tab::<tauri::Wry>,
            update_tab::<tauri::Wry>,
            update_tab_fire_and_forget::<tauri::Wry>,
            update_field,
            get_equation_with_symbols,
            get_equation_with_numbers,
            get_equation_inputs,
            get_equation_inputs_symbols,
            set_current_tab_method::<tauri::Wry>,
            calculate_form::<tauri::Wry>,
            new_method_builder::<tauri::Wry>,
            method_builder_delete_metadata::<tauri::Wry>,
            method_builder_add_metadata::<tauri::Wry>,
            method_builder_update_metadata_required,
            method_builder_update_metadata_name,
        ])
        .events(collect_events![ValidationErrorEvent]);

    if !config::is_mobile() && config::is_dev() {
        specta_builder
            .export(Typescript::default(), "../src/bindings.ts")
            .expect("Failed to export typescript bindings");
    }

    builder
        .invoke_handler(specta_builder.invoke_handler())
        .setup(move |app| {
            // This is also required if you want to use events
            specta_builder.mount_events(app);

            Ok(())
        })
        .setup(some_setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn some_setup<R: tauri::Runtime>(
    app: &mut tauri::App<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    let main_window = app.get_webview_window("main").unwrap();
    if let Some(screen) = main_window.current_monitor().unwrap() {
        let size = screen.size();
        main_window
            .set_size(Size::Logical(LogicalSize {
                width: (size.width / 2) as f64,
                height: size.height as f64,
            }))
            .unwrap();

        main_window
            .set_position(Position::Logical(LogicalPosition { x: 0.0, y: 0.0 }))
            .unwrap();
    }

    let handle = app.app_handle().clone();
    let other_handle = app.app_handle().clone();

    #[cfg(desktop)]
    handle
        .plugin(tauri_plugin_single_instance::init(
            move |app, argv, _cwd| {
                let _ = app
                    .get_webview_window("main")
                    .expect("no main window")
                    .set_focus();

                if let Ok(url) = tauri::Url::parse(argv.first().unwrap_or(&String::new())) {
                    let params = url.query_pairs().collect::<Vec<_>>();
                    let token = params.iter().find(|(key, _)| key == "ott");

                    if let Some((_, token)) = token {
                        other_handle.emit("ott", token).unwrap();
                    }
                }
                ()
            },
        ))
        .unwrap();

    handle.plugin(tauri_plugin_deep_link::init()).unwrap();

    #[cfg(any(windows, target_os = "linux"))]
    {
        use tauri_plugin_deep_link::DeepLinkExt;
        app.deep_link().register_all()?;
    }

    app.deep_link().on_open_url(move |request| {
        // The event will be triggered twice on mac. Once with an empty string
        // and once with the actual request. We can ignore the empty string.
        // if request.is_empty() {
        //     return;
        // }

        if let Some(url) = &request.urls().first() {
            let params = url.query_pairs().collect::<Vec<_>>();
            let token = params.iter().find(|(key, _)| key == "ott");

            if let Some((_, token)) = token {
                handle.emit("ott", token).unwrap();
            }
        }

        ()
    });

    let initial_tab = TabBuilder::new()
        .state(TabState::Index(NoCalc {
            id: "Index".to_string(),
        }))
        .current(true)
        .build();

    let wrapped_tab = Arc::new(RwLock::new(initial_tab));

    let tabs = WrappedTabState::new(vec![wrapped_tab]);

    app.manage(tabs);

    Ok(())
}

pub fn test_app() -> (tauri::App<MockRuntime>, tauri::WebviewWindow<MockRuntime>) {
    let initial_tab = TabBuilder::new()
        .state(TabState::Index(NoCalc {
            id: "Index".to_string(),
        }))
        .current(true)
        .build();
    let wrapped_tab = Arc::new(RwLock::new(initial_tab));
    let tabs = WrappedTabState::new(vec![wrapped_tab]);

    let builder = tauri::test::mock_builder();
    let app = builder
        .invoke_handler(tauri::generate_handler![
            new_tab,
            get_current_tab,
            get_tabs,
            set_current_tab,
            delete_tab,
            update_tab,
            update_tab_fire_and_forget,
        ])
        .setup(some_setup)
        .manage(tabs)
        .build(tauri::test::mock_context(tauri::test::noop_assets()))
        .unwrap();

    let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();

    (app, webview)
}
