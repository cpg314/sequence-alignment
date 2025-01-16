use std::path::Path;

use super::*;

/// A single sequence with metadata
#[derive(Debug)]
pub struct FastaSequence<T> {
    pub meta: String,
    pub sequence: Sequence<T>,
}

/// A decoded FASTA file as a list of sequences
#[derive(Debug)]
pub struct Fasta<T>(pub Vec<FastaSequence<T>>);

impl<T: From<char>> Fasta<T> {
    /// Decode FASTA file
    #[tracing::instrument]
    pub fn from_path(p: &Path) -> anyhow::Result<Self> {
        info!("Parsing FASTA file");
        let data = std::fs::read_to_string(p)?;
        let lines = data.lines();
        let mut sequences = vec![];
        for line in lines {
            if let Some(meta) = line.strip_prefix(">") {
                sequences.push(FastaSequence {
                    meta: meta.into(),
                    sequence: Default::default(),
                });
            } else {
                match sequences.last_mut() {
                    Some(l) => l.sequence.0.extend(line.chars().map(T::from)),
                    None => anyhow::bail!("Sequence without metadata"),
                }
            }
        }
        Ok(Self(sequences))
    }
}
