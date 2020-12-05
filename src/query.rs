#[derive(Debug, Clone, Default)]
pub struct Query {
    word: String,
}

impl Query {
    pub fn word(&self) -> &str {
        self.word.as_str()
    }
}

impl Into<String> for Query {
    fn into(self) -> String {
        self.word
    }
}

pub fn query_from(s: String) -> Option<Query> {
    if s.is_empty() {
        None
    } else {
        Some(Query { word: s })
    }
}
