// === External crates ========================================================

use std::{
    path::PathBuf,
    fmt::{self, Display, Formatter},
};

use clap::Parser;

use vmd_api::{
    BASE_PATH,
    API_VERSION,
};
// === Implementations ========================================================

#[derive(Parser, Debug)]
pub(crate) struct Args {
	#[clap(long, required = true)]
	pub addr: String,
	#[clap(long, required = true)]
	pub port: u16,
	#[clap(long, required = true)]
	pub cacert: PathBuf,
	#[clap(long, required = true)]
	pub cert: PathBuf,
	#[clap(long, required = true)]
	pub key: PathBuf,
	#[clap(long)]
	pub oscp: Option<PathBuf>,
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let url = format!("https://{}:{}", self.addr, self.port);
        write!(f, "📌 api-version:           {}\n", API_VERSION)?;
        write!(f, "📡 address:               {}\n", url)?;
        write!(f, "🚩 base-path:             {}\n", BASE_PATH)?;
        write!(f, "🔑 private-key:           {}\n", self.key.display())?;
        write!(f, "🔐 cerificate:            {}\n", self.cert.display())?;
        write!(f, "🔐 certificate-authority: {}\n", self.cacert.display())?;
        if let Some(oscp) = &self.oscp {
            write!(f, "🔐 oscp:                  {}\n", oscp.display())?;
        } else {
            write!(f, "🚫 oscp:                  None\n")?;
        }
        write!(f, "")?;
        Ok(())
    }
}

// ===  EOF  ==================================================================
