// === Cli ====================================================================
//
// Command line interface for the virtual machine daemon server.
//
// === External crates ========================================================

use std::{
    path::PathBuf,
    fmt::{Display, Formatter, Result},
};
use clap::Parser;

// === Internal modules =======================================================

use vmd_rust_server_api::{
    BASE_PATH,
    API_VERSION,
};

// === Implementations ========================================================

/// A virtual machine daemon server
#[derive(Parser, Debug)]
pub(crate) struct Args {

    /// Server hostname
	#[clap(short, long, required = true)]
	pub hostname: String,

    /// Port to listen on
	#[clap(short, long, required = true)]
	pub port: u16,

    /// Path to certificate authority pem file (.pem)
	#[clap(short = 'a', long, required = true)]
	pub cacert: PathBuf,

    /// Path to server certificate pem file (.pem)
	#[clap(short, long, required = true)]
	pub cert: PathBuf,

    /// Path to server private key pem file (.pem)
	#[clap(short, long, required = true)]
	pub key: PathBuf,

    /// Unimplemented!
	#[clap(long)]
	pub ocsp: Option<PathBuf>,
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let url = format!("https://{}:{}", self.hostname, self.port);
        write!(f, "📌 api-version:           {}\n", API_VERSION)?;
        write!(f, "📡 address:               {}\n", url)?;
        write!(f, "🚩 base-path:             {}\n", BASE_PATH)?;
        write!(f, "🔑 private-key:           {}\n", self.key.display())?;
        write!(f, "🔐 certificate:           {}\n", self.cert.display())?;
        write!(f, "🔐 certificate-authority: {}\n", self.cacert.display())?;
        if let Some(ocsp) = &self.ocsp {
            write!(f, "🔐 ocsp:                  {}\n", ocsp.display())?;
        } else {
            write!(f, "🔐 ocsp:                  🚫\n")?;
        }
        write!(f, "")?;
        Ok(())
    }
}

// ===  EOF  ==================================================================
