use itertools::Itertools;
use serde::Deserialize;

/// A sequence represented as a list of `T`
#[derive(Deserialize, Debug)]
pub struct Sequence<T = char>(pub Vec<T>);

impl<T> Default for Sequence<T> {
    fn default() -> Self {
        Self(vec![])
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Sequence<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().join(""))
    }
}

impl<T> Sequence<T> {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
impl<S: AsRef<str>> From<S> for Sequence {
    fn from(source: S) -> Self {
        Self(source.as_ref().chars().collect())
    }
}
