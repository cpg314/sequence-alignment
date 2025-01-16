mod align;
mod fasta;
mod sequence;
use sequence::Sequence;
mod web;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use rayon::prelude::*;
use tracing::*;

#[derive(Parser)]
struct Flags {
    #[clap(flatten)]
    aligner: align::Aligner,
    #[clap(subcommand)]
    mode: Mode,
}
#[derive(Subcommand)]
enum Mode {
    /// Align the first two sequences in a FASTA file
    Align {
        fasta: PathBuf,
        #[clap(long, short, default_value_t = 1)]
        runs: u32,
        #[clap(long, conflicts_with = "runs")]
        output: Option<PathBuf>,
    },
    /// Launch alignment HTTP service
    Serve {
        #[clap(long, default_value_t = 3000)]
        port: u32,
    },
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();
    let args = Flags::parse();
    if let Err(e) = main_impl(&args).await {
        error!("Failed: {}", e);
        std::process::exit(1);
    }
}

async fn main_impl(args: &Flags) -> anyhow::Result<()> {
    match &args.mode {
        Mode::Align {
            fasta,
            runs,
            output,
        } => {
            let fasta = fasta::Fasta::<char>::from_path(fasta)?;
            anyhow::ensure!(fasta.0.len() == 2, "Expecting exactly two sequences");

            // Align
            let start = std::time::Instant::now();
            (0..*runs)
                // Parallelism with rayon
                .into_par_iter()
                .try_for_each(|_| {
                    let seq1 = &fasta.0[0];
                    let seq2 = &fasta.0[1];
                    info!("Aligning\n{:?} with\n{:?}", seq1.meta, seq2.meta);
                    let alignment = args.aligner.align([&seq1.sequence, &seq2.sequence]);
                    info!("{}", alignment);
                    if let Some(output) = &output {
                        alignment.write(output)?;
                    }
                    anyhow::Ok(())
                })?;

            info!(
                "Performed {} run(s) in {:?} ({:.0} runs/s)",
                runs,
                start.elapsed(),
                *runs as f32 / start.elapsed().as_secs_f32()
            );
        }
        Mode::Serve { port } => {
            info!("Starting server on http://0.0.0.0:{}", port);
            let app = axum::Router::new()
                .route("/align", axum::routing::post(web::align_post))
                .with_state(args.aligner);
            let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
            axum::serve(listener, app).await?;
        }
    }

    Ok(())
}
