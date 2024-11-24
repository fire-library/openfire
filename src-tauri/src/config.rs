pub fn is_mobile() -> bool {
    cfg!(target_os = "android") || cfg!(target_os = "ios")
}

pub fn server_url() -> String {
    if is_mobile() {
        if is_dev() {
            "http://10.0.2.2:9000".to_string()
        } else {
            "https://api.example.com".to_string()
        }
    } else {
        if is_dev() {
            "http://localhost:9000".to_string()
        } else {
            "https://api.example.com".to_string()
        }
    }
}

pub fn is_dev() -> bool {
    cfg!(debug_assertions)
}

pub fn graphql_url() -> String {
    format!("{}/graphql", server_url())
}
