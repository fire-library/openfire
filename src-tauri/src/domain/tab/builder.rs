use super::{NoCalc, Tab, TabState};
use std::path::Path;
use uuid::Uuid;

pub struct TabBuilder {
    id: Option<String>,
    state: Option<TabState>,
    saved: Option<bool>,
    current: Option<bool>,
    filename: Option<String>,
    title: Option<String>,
}

impl TabBuilder {
    pub fn new() -> TabBuilder {
        TabBuilder {
            id: None,
            state: Some(TabState::Index(NoCalc {
                id: "Index".to_string(),
            })),
            saved: None,
            current: None,
            filename: None,
            title: Some("Index".to_string()),
        }
    }

    pub fn state(mut self, state: TabState) -> TabBuilder {
        self.state = Some(state);
        self
    }

    pub fn filename(mut self, filename: String) -> TabBuilder {
        self.filename = Some(filename);
        self
    }

    pub fn saved(mut self, saved: bool) -> TabBuilder {
        self.saved = Some(saved);
        self
    }

    pub fn current(mut self, current: bool) -> TabBuilder {
        self.current = Some(current);
        self
    }

    fn update_title(&mut self) {
        if let Some(ref filename) = self.filename {
            let path = Path::new(filename);
            let name = path
                .file_name()
                .expect("invalid filename")
                .to_str()
                .expect("invalid filename");

            self.title = Some(name.to_string());
            return;
        }
        match self.state {
            Some(TabState::Index(_)) => self.title = Some("Index".to_string()),
            Some(TabState::ParametricFireAbout(_)) => {
                self.title = Some("About - Parametric Fires".to_string())
            }
            _ => self.title = Some("Untitled".to_string()),
        }
    }

    fn update_saved(&mut self) {
        match self.state {
            Some(TabState::Index(_)) => self.saved = Some(true),
            Some(TabState::ParametricFireAbout(_)) => self.saved = Some(true),
            _ => return,
        }
    }

    pub fn build(mut self) -> Tab {
        self.update_title();
        self.update_saved();

        Tab {
            id: self.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            state: self.state.unwrap_or(TabState::Index(NoCalc {
                id: "Index".to_string(),
            })),
            saved: self.saved.unwrap_or(false),
            current: self.current.unwrap_or(false),
            filename: self.filename,
            title: self.title,
        }
    }
}
