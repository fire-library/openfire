use crate::impls::DocumentImplementations;
use ansi_term::Style;
use comfy_table::Table;

pub fn list(document_id: String) {
    let docs = crate::impls::all_impls();
    let doc = docs.iter().find(|doc| doc.document_id == document_id);

    match doc {
        Some(doc) => {
            list_doc_methods(doc.clone());
        }
        None => {
            println!("Document id {} does not exist", document_id);
        }
    }
}

pub fn list_documents() {
    let docs = crate::impls::all_impls();
    let mut table = Table::new();
    table.set_header(vec![
        "Document Name",
        "Document ID (use this in CLI commands)",
    ]);
    for doc in docs {
        table.add_row(vec![doc.document, doc.document_id]);
    }
    println!("{table}");
}

pub fn list_doc_methods(doc: DocumentImplementations) {
    print!("\n");
    println!(
        "================ {} - {} ================",
        Style::new().bold().paint(doc.clone().document),
        doc.clone().document_id
    );

    let mut table = Table::new();
    table.set_header(vec!["Method", "Method ID (use this in CLI commands)"]);
    for method in doc.clone().implementations {
        table.add_row(vec![method.name, method.id]);
    }
    println!("{table}");
    print!("\n");
}
