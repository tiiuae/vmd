// === External crates ========================================================

use std::path::PathBuf;
use clap::Parser;

// === Implementations ========================================================

#[derive(Parser, Debug)]
pub(crate) struct Args {
	#[clap(long, required = true)]
	pub addr: String,
	#[clap(long, required = true)]
	pub port: u16,
	#[clap(long, required = true)]
	pub ca: PathBuf,
	#[clap(long, required = true)]
	pub crt: PathBuf,
	#[clap(long, required = true)]
	pub key: PathBuf,
	#[clap(long)]
	pub oscp: Option<PathBuf>,
}

// ===  EOF  ==================================================================
