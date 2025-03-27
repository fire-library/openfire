pub use br_187;
pub use bs9999;
pub use cibse_guide_e;
pub use introduction_to_fire_dynamics;
pub use pd_7974;
pub use sfpe_handbook;
pub use tr17;

pub use framework::*;

use framework::method::runner::MethodRunner;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub use openfire_cli::impls::DocumentImplementations;
pub use openfire_cli::impls::all_impls;
pub use openfire_cli::impls::register_runners;

pub use serde;
pub use specta;

lazy_static! {
    static ref DOCUMENT_REGISTRY: Arc<RwLock<HashMap<String, Box<dyn Fn() -> Box<dyn MethodRunner> + Send + Sync>>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

pub fn register_runner<T: MethodRunner + Default + 'static>() {
    let name = T::default().id();
    let runner: Box<dyn Fn() -> Box<dyn MethodRunner> + Send + Sync> =
        Box::new(|| Box::new(T::default()));

    DOCUMENT_REGISTRY
        .write()
        .unwrap()
        .insert(name.to_string(), runner);
}

pub fn get_runner(name: &str) -> Box<dyn MethodRunner> {
    DOCUMENT_REGISTRY
        .read()
        .unwrap()
        .get(name)
        .map(|f| f())
        .unwrap()
}

pub fn fetch_runner(name: &str) -> Option<Box<dyn MethodRunner>> {
    DOCUMENT_REGISTRY.read().unwrap().get(name).map(|f| f())
}

pub fn fetch_runners_by_doc(doc_id: &str) -> Vec<Box<dyn MethodRunner>> {
    DOCUMENT_REGISTRY
        .read()
        .unwrap()
        .values()
        .filter(|f| f().reference().document_id() == doc_id)
        .map(|f| f())
        .collect()
}
