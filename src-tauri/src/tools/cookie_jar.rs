use reqwest_cookie_store::CookieStoreMutex;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest;

#[derive(Serialize, Deserialize, Debug)]
struct SerializableCookie {
    url: String,
    cookies: String,
}

pub fn init_client<R: tauri::Runtime>(
    app_handle: &AppHandle<R>,
) -> Result<(reqwest::Client, Arc<CookieStoreMutex>), Box<dyn std::error::Error>> {
    // Path to the file for storing cookies
    let cookie_file_path = if let Ok(dir) = app_handle.path().app_data_dir() {
        format!("{}/cookies.json", dir.display(),)
    } else {
        panic!("Could not get app data directory");
    };

    let cookie_store = {
        if let Ok(file) = std::fs::File::open(&cookie_file_path).map(std::io::BufReader::new) {
            reqwest_cookie_store::CookieStore::load_json(file).unwrap()
        } else {
            std::fs::File::create(&cookie_file_path).unwrap();
            reqwest_cookie_store::CookieStore::default()
        }
    };

    let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
    let cookie_store = std::sync::Arc::new(cookie_store);
    // Load cookies from files

    // Create reqwest client with the Jar as the cookie store
    let client = reqwest::Client::builder()
        .cookie_provider(std::sync::Arc::clone(&cookie_store))
        .build()?;

    Ok((client, cookie_store))
}

pub fn save_cookies_to_file(
    cookie_store: Arc<CookieStoreMutex>,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = std::fs::File::create(file_path)
        .map(std::io::BufWriter::new)
        .unwrap();
    let store = cookie_store.lock().unwrap();
    store.save_json(&mut writer).unwrap();
    Ok(())
}
