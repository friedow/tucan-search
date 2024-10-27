use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(author, version = CliArgs::version(), about, long_about=None) ]
#[command(next_line_help = true)]
pub struct CliArgs {
    #[clap(
        short,
        long,
        help = "The location of the configuration file",
        env = "CENTERPIECE_CONFIGURATION_FILE"
    )]
    pub config: Option<String>,
}

impl CliArgs {
    /// Surface current version together with the current git revision and date,
    /// if available
    pub fn version() -> &'static str {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let date = option_env!("GIT_DATE")
            .map(|date| format!(" - {}", date))
            .unwrap_or_default();
        let rev = option_env!("GIT_REV")
            .map(|rev| format!(" - {}", rev))
            .unwrap_or_default();
        // This is a memory leak, only use sparingly.
        Box::leak(format!("{VERSION}{date}{rev}").into_boxed_str())
    }
}
