use std::cmp::PartialEq;
use std::collections::VecDeque;
use std::path::Path;

use clap::Parser;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use tracing::*;

use super::Sequence;

/// An alignment of two sequences
#[derive(Debug, Serialize, Deserialize)]
pub struct Alignment<T> {
    pub alignment: VecDeque<[Option<T>; 2]>,
    /// Note that the score depends on the aligner parameters
    score: f32,
}
impl<T> Default for Alignment<T> {
    fn default() -> Self {
        Self {
            score: f32::NAN,
            alignment: Default::default(),
        }
    }
}
impl<T: std::fmt::Display + PartialEq> std::fmt::Display for Alignment<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "Alignment with score {:.2}, {:.2}% aligned",
            self.score,
            100.0 * self.matching_ratio()
        )?;
        for i in 0..2 {
            write!(
                f,
                "{}{}",
                self.alignment
                    .iter()
                    .map(|a| a[i]
                        .as_ref()
                        .map(|a| a.to_string())
                        .unwrap_or("-".to_string()))
                    .join(""),
                if i == 0 { "\n" } else { "" }
            )?;
        }
        Ok(())
    }
}
impl<T: Serialize> Alignment<T> {
    pub fn write(&self, filename: &Path) -> anyhow::Result<()> {
        // Swap JSON for your favourite format (e.g. bincode, cbor...)
        Ok(std::fs::write(filename, serde_json::to_string(&self)?)?)
    }
}
impl<T: PartialEq> Alignment<T> {
    /// Ratio of the number of aligned pairs divided by the length including gaps
    fn matching_ratio(&self) -> f32 {
        self.alignment
            .iter()
            .filter(|[a, b]| a.is_some() && a == b)
            .count() as f32
            / self.alignment.len() as f32
    }
}

#[derive(Parser, Debug, Copy, Clone)]
pub struct Aligner {
    #[clap(long, default_value_t=-2.0)]
    mismatch_penalty: f32,
    #[clap(long, default_value_t=-1.0)]
    gap_penalty: f32,
}

#[derive(Copy, Clone)]
enum AlignKind {
    Match,
    Delete,
    Insert,
}
impl Aligner {
    /// Align two sequences and return an alignment
    #[tracing::instrument(skip_all)]
    pub fn align<T: std::cmp::PartialEq + Copy>(&self, seqs: [&Sequence<T>; 2]) -> Alignment<T> {
        let start = std::time::Instant::now();
        let mut scores: Vec<Vec<f32>> = vec![vec![0.0; seqs[1].len() + 1]; seqs[0].len() + 1];
        let mut choices: Vec<Vec<AlignKind>> =
            vec![vec![AlignKind::Match; seqs[1].len() + 1]; seqs[0].len() + 1];
        #[allow(clippy::needless_range_loop)]
        for i in 0..=seqs[0].len() {
            scores[i][0] = i as f32 * self.gap_penalty;
        }
        for i in 0..=seqs[1].len() {
            scores[0][i] = i as f32 * self.gap_penalty;
        }
        for i in 1..=seqs[0].len() {
            for j in 1..=seqs[1].len() {
                let (kind, score) = [
                    (
                        AlignKind::Match,
                        scores[i - 1][j - 1]
                            + if seqs[0].0[i - 1] != seqs[1].0[j - 1] {
                                self.mismatch_penalty
                            } else {
                                0.0
                            },
                    ),
                    (AlignKind::Delete, scores[i - 1][j] + self.gap_penalty),
                    (AlignKind::Insert, scores[i][j - 1] + self.gap_penalty),
                ]
                .into_iter()
                .max_by_key(|(_, x)| OrderedFloat(*x))
                .unwrap();
                scores[i][j] = score;
                choices[i][j] = kind;
            }
        }
        let mut i = seqs[0].len();
        let mut j = seqs[1].len();
        let mut alignment = Alignment::default();
        while i > 0 && j > 0 {
            match choices[i][j] {
                AlignKind::Match => {
                    alignment
                        .alignment
                        .push_front([Some(seqs[0].0[i - 1]), Some(seqs[1].0[j - 1])]);
                    i -= 1;
                    j -= 1;
                }
                AlignKind::Delete => {
                    alignment
                        .alignment
                        .push_front([Some(seqs[0].0[i - 1]), None]);
                    i -= 1;
                }
                AlignKind::Insert => {
                    alignment
                        .alignment
                        .push_front([None, Some(seqs[1].0[j - 1])]);
                    j -= 1;
                }
            }
        }
        debug!(duration = ?start.elapsed(), "Aligned");
        alignment.score = scores[seqs[0].len()][seqs[1].len()];
        alignment
    }
}
