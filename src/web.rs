use serde::Deserialize;
use tracing::*;

use super::{align, Sequence};

#[derive(Deserialize)]
pub struct AlignData {
    seq1: String,
    seq2: String,
}
pub async fn align_post(
    axum::extract::State(aligner): axum::extract::State<align::Aligner>,
    axum::Json(data): axum::Json<AlignData>,
) -> axum::Json<align::Alignment<char>> {
    info!("Processing request");
    let seq1 = Sequence::from(&data.seq1);
    let seq2 = Sequence::from(&data.seq2);
    axum::Json(aligner.align([&seq1, &seq2]))
}
