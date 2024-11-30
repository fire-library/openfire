pub mod builder;

use crate::domain::filesystem::Filetypes;
use crate::domain::method::builder::MethodBuilder;
use crate::domain::method::Method;
pub use builder::TabBuilder;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::sync::{Arc, RwLock};

pub type WrappedTabState = tauri::async_runtime::Mutex<Vec<Arc<RwLock<Tab>>>>;

#[derive(Clone, Type, Deserialize, Serialize, Debug)]
pub struct Tab {
    pub id: String,
    pub state: TabState,
    pub saved: bool,
    pub current: bool,
    pub filename: Option<String>,
    pub title: Option<String>,
}

#[derive(Clone, Type, Deserialize, Serialize, Debug, PartialEq)]
pub struct NoCalc {
    pub id: String,
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TabState {
    Index(NoCalc),
    Method(Method),
    MethodBuilder(MethodBuilder),
}

impl From<Filetypes> for TabState {
    fn from(filetypes: Filetypes) -> Self {
        match filetypes {
            Filetypes::Method(method) => TabState::Method(method.into()),
        }
    }
}

type WrappedTab = Arc<RwLock<Tab>>;

pub trait Tabs {
    fn new_tab(&mut self, tab: Tab, after: Option<u32>);
    fn get_by_id(&self, id: &str) -> Result<WrappedTab, String>;
    fn get_by_filename(&self, filename: &str) -> Result<WrappedTab, String>;
    fn get_index_by_id(&self, id: &str) -> Result<usize, String>;
    fn get_current(&self) -> Result<WrappedTab, String>;
    fn set_current(&mut self, id: String) -> Result<(), String>;
    fn set_current_by_index(&mut self, index: usize) -> Result<(), String>;
}

impl Tabs for Vec<WrappedTab> {
    fn get_by_id(&self, id: &str) -> Result<WrappedTab, String> {
        for tab in self {
            if tab.read().unwrap().id == id {
                return Ok(tab.clone());
            }
        }
        return Err("No tab found".to_string());
    }

    fn get_by_filename(&self, filename: &str) -> Result<WrappedTab, String> {
        for tab in self {
            if let Some(name) = &tab.read().unwrap().filename {
                if name == filename {
                    return Ok(tab.clone());
                }
            }
        }
        return Err("No tab found".to_string());
    }

    fn get_index_by_id(&self, id: &str) -> Result<usize, String> {
        for (index, tab) in self.iter().enumerate() {
            if tab.read().unwrap().id == id {
                return Ok(index);
            }
        }
        return Err("No tab found".to_string());
    }

    fn get_current(&self) -> Result<WrappedTab, String> {
        for tab in self {
            if tab.read().unwrap().current {
                return Ok(tab.clone());
            }
        }
        return Err("No tab found".to_string());
    }

    fn set_current(&mut self, id: String) -> Result<(), String> {
        match self.get_by_id(&id) {
            Ok(_tab) => {
                self.iter_mut().for_each(|t| {
                    if t.read().unwrap().id == id {
                        t.write().unwrap().current = true;
                    } else {
                        t.write().unwrap().current = false;
                    }
                });
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    fn set_current_by_index(&mut self, index: usize) -> Result<(), String> {
        let indexed_tab_id = self[index].read().unwrap().id.clone();
        if self.len() - 1 >= index {
            self.iter().for_each(|t| {
                if t.read().unwrap().id == indexed_tab_id {
                    t.write().unwrap().current = true;
                } else {
                    t.write().unwrap().current = false;
                }
            });
            return Ok(());
        } else {
            return Err("Index out of bounds".to_string());
        }
    }

    fn new_tab(&mut self, tab: Tab, after: Option<u32>) {
        let tab_id = tab.id.clone();
        let wrapped_tab = Arc::new(RwLock::new(tab));

        match after {
            Some(after) => {
                if after > self.len() as u32 {
                    self.push(wrapped_tab);
                } else {
                    self.insert(after as usize, wrapped_tab);
                }
            }
            None => {
                self.push(wrapped_tab);
            }
        }

        self.set_current(tab_id).expect("Failed to set current tab");
    }
}
