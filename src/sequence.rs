use itertools::Itertools;
use serde::Deserialize;

/// A sequence represented as a list of `char`s.
#[derive(Deserialize, Debug, Default)]
pub struct Sequence(pub Vec<char>);

impl std::fmt::Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().join(""))
    }
}

impl Sequence {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
impl<S: AsRef<str>> From<S> for Sequence {
    fn from(source: S) -> Self {
        Self(source.as_ref().chars().collect())
    }
}
