#[derive(Debug, clap::Parser)]
#[command(author, about, version)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub(crate) operation: Operation,

    #[clap(flatten)]
	pub(crate) global: Global,
}

#[derive(Debug, Clone, clap::Args)]
pub(crate) struct Global {
    #[clap(flatten)]
	pub(crate) connection: Connection,
	#[clap(long, default_value_t = false)]
	pub (crate) verbose: bool,
	#[clap(long)]
	pub (crate) output: OutputFormat,
}

#[derive(Debug, Clone, clap::Args)]
pub(crate) struct Connection {
    #[clap(long, required = true)]
    pub(crate) hostname: String,
    #[clap(short, long, required = true)]
    pub(crate) port: u16,

    #[clap(long, required = true)]
    pub(crate) cacert: std::path::PathBuf,
    #[clap(long, required = true)]
    pub(crate) key: std::path::PathBuf,
    #[clap(long, required = true)]
    pub(crate) cert: std::path::PathBuf,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub(crate) enum OutputFormat {
	Json,
	Yaml,
	Text,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum Operation {
	List,
	Info { id: u32 },
	Action {
		id: u32,
		#[clap(long, value_enum)]
		action: VmAction
	},
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub(crate) enum VmAction {
	Start,
	Stop,
	Pause,
}

// Implement `to_json` for `VmAction` so that we can use it in `serde_json::json!`.
impl serde::Serialize for VmAction {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(match self {
			VmAction::Start => "start",
			VmAction::Stop => "stop",
			VmAction::Pause => "pause",
		})
	}
}