use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AdaptIntent {
    pub name: String,
    requires: Vec<Vec<String>>,
    at_least_one: Vec<Vec<String>>,
    optional: Vec<Vec<String>>
}

impl AdaptIntent {
    #[allow(dead_code)]
    /// Create a new Adapt intent
    pub fn new (name: &str) -> AdaptIntent {
        AdaptIntent {
            name: name.to_string(),
            requires: Vec::new(),
            at_least_one: Vec::new(),
            optional: Vec::new()
        }
    }

    #[allow(dead_code)]
    /// Add a required keyword
    pub fn requiring(mut self, keyword: &str) -> AdaptIntent {
        self.requires.push(vec![keyword.to_string(), keyword.to_string()]);
        self
    }

    #[allow(dead_code)]
    /// Add an optional keyword
    pub fn optionally(mut self, keyword: &str) -> AdaptIntent {
        self.optional.push(vec![keyword.to_string(), keyword.to_string()]);
        self
    }
}


#[derive(Serialize, Deserialize)]
pub struct AdaptKeyword {
    pub entity_value: String,
    pub entity_type: String
}

#[derive(Serialize, Deserialize)]
pub struct AdaptKeywordAlias {
    start: String,
    end: String,
    alias_of: String
}
