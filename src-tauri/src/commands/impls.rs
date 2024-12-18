use crate::domain::impls::all_impls;
use crate::domain::impls::Document;
use crate::domain::impls::DocumentImplementations;

#[tauri::command]
#[specta::specta]
pub fn all_implementations() -> Result<Vec<DocumentImplementations>, String> {
    Ok(all_impls())
}

#[tauri::command]
#[specta::specta]
pub fn document_title(doc: Document) -> Result<String, String> {
    Ok(doc.name())
}

#[tauri::command]
#[specta::specta]
pub fn friendly_reference(doc: Document) -> Result<String, String> {
    Ok(doc.friendly_reference())
}

#[tauri::command]
#[specta::specta]
pub fn harvard_reference(doc: Document) -> Result<String, String> {
    Ok(doc.harvard_reference())
}
