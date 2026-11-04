use oniko_registry::RegistryItem;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/components.rs"));
}

pub struct Component {
    pub name: &'static str,
    pub item: RegistryItem,
}

impl Component {
    pub fn title(&self) -> &str {
        self.item.title.as_deref().unwrap_or(self.name)
    }

    pub fn description(&self) -> &str {
        self.item.description.as_deref().unwrap_or_default()
    }

    /// The source `oniko add` writes, with the site's style compiled in.
    pub fn source(&self) -> &str {
        self.item
            .files
            .first()
            .and_then(|f| f.content.as_deref())
            .unwrap_or_default()
    }
}

/// Every component in the registry, sorted by title. The JSON is produced by
/// `oniko build` and checked by the repository tests, so it always parses.
pub fn components() -> Vec<Component> {
    let mut all: Vec<Component> = generated::COMPONENTS
        .iter()
        .filter_map(|(name, json)| {
            let item = serde_json::from_str(json).ok()?;
            Some(Component { name, item })
        })
        .collect();
    all.sort_by(|a, b| a.title().cmp(b.title()));
    all
}

pub fn component(name: &str) -> Option<Component> {
    components().into_iter().find(|c| c.name == name)
}
