pub fn list_all() {
    let docs = crate::impls::all_impls();
    for doc in docs {
        super::documents::list_doc_methods(doc);
    }
}
