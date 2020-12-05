use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Arn(String);

impl Arn {
    pub fn str(&self) -> &str {
        &self.0
    }
}

impl<T: Into<String>> From<T> for Arn {
    fn from(s: T) -> Self {
        Arn(s.into())
    }
}

// FIXME: Replace to Display macro  from derive_more
impl fmt::Display for Arn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str() {
        assert_eq!(
            "arn:aws:logs:ap-northeast-1:000000000000:log-group:log_group_name",
            Arn::from("arn:aws:logs:ap-northeast-1:000000000000:log-group:log_group_name").str()
        )
    }
}
