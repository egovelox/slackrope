use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn on debugging level
    /// (level 1 with -d, level 2 with -d -d)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Fetch the current hotlist
    Hotlist {
        #[arg(
            short,
            long,
            value_name = "FORMAT",
            num_args = 1,
            default_value_t = OutputFormat::Shell,
            value_enum
        )]
        format: OutputFormat,
        /// [default: false] whether the command should start weechat
        /// (when it's not running)
        #[arg(short, long, value_name = "START", action)]
        start: bool,
        /// [default: none] use a template e.g "󰁥{{priority_1}} {{priority_2}} {{priority_3}}"
        /// Note that you can use ANSI escape codes e.g "\e[38;5;208m󰁥\e[0m{{priority_1}}"
        #[arg(short, long, value_name = "TEMPLATE", verbatim_doc_comment)]
        template: Option<String>,
    },
    /// Clear the current hotlist (sets all counters to 0)
    Clear,
    /// Kill weechat daemon
    Kill,
    /// List registered slack teams
    ListTeams,
    /// Register a new slack team
    Register {
        /// (Optional) use only after you got your token from slack
        #[arg(short, long, value_name = "TOKEN")]
        token: Option<String>,
    },
    /// Print health report
    Health,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    /// Shell format: text
    Shell,
    /// Simple format: json
    Simple,
    /// Detailed format: json
    Detailed,
}
