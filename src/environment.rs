use config::Config;
use std::{path::Path, process::exit, sync::OnceLock};

pub const XDG_CONFIG_HOME: &str = "XDG_CONFIG_HOME";
pub const APP_NAME: &str = "slackrope";
pub const CONFIG_FILE: &str = "sr_config_file";
pub const WEECHAT_PROGRAM_NAME: &str = "sr_weechat_program_name";
pub const WEECHAT_HOST: &str = "sr_weechat_host";
pub const WEECHAT_RELAY_PORT: &str = "sr_weechat_relay_port";
pub const WEECHAT_PASSWORD: &str = "sr_weechat_password";
pub const SLACK_REGISTER_BASEURL: &str = "sr_slack_register_baseurl";
pub const SLACK_REGISTER_WEESLACK_CLIENT_ID: &str = "sr_slack_register_weeslack_client_id";
pub const SLACK_REGISTER_SCOPE: &str = "sr_slack_register_scope";
pub const SLACK_REGISTER_REDIRECT_URI: &str = "sr_slack_register_redirect_uri";
pub const WEE_SLACK_PLUGIN_DIRECTORY: &str = "sr_wee_slack_plugin_directory";
pub const WEE_SLACK_PLUGIN_FILENAME: &str = "sr_wee_slack_plugin_filename";

fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        let home_dir = get_home_dir().display().to_string();
        let config_dir = match std::env::var(XDG_CONFIG_HOME) {
            Ok(dir) => format!("{dir}/{APP_NAME}"),
            Err(_) => format!("{home_dir}/.config/{APP_NAME}"),
        };
        let config_filepath = format!("{config_dir}/{APP_NAME}.toml");

        Config::builder()
            .add_source(
                config::File::with_name(&config_filepath)
                    .format(config::FileFormat::Toml)
                    .required(false),
            )
            .set_default(
                CONFIG_FILE,
                Path::new(&config_filepath)
                    .exists()
                    .then(|| config_filepath.clone()),
            )
            .unwrap()
            .set_default(WEECHAT_PROGRAM_NAME, "weechat-headless")
            .unwrap()
            .set_default(WEECHAT_HOST, "127.0.0.1")
            .unwrap()
            .set_default(WEECHAT_RELAY_PORT, "8000")
            .unwrap()
            .set_default(WEECHAT_PASSWORD, "")
            .unwrap()
            .set_default(SLACK_REGISTER_BASEURL, "https://slack.com/oauth/authorize")
            .unwrap()
            .set_default(SLACK_REGISTER_WEESLACK_CLIENT_ID, "2468770254.51917335286")
            .unwrap()
            .set_default(SLACK_REGISTER_SCOPE, "client")
            .unwrap()
            .set_default(
                SLACK_REGISTER_REDIRECT_URI,
                "https%3A%2F%2Fwee-slack.github.io%2Fwee-slack%2Foauth",
            )
            .unwrap()
            .set_default(
                WEE_SLACK_PLUGIN_DIRECTORY,
                "$HOME/.local/share/weechat/python".replace("$HOME", &home_dir),
            )
            .unwrap()
            .set_default(WEE_SLACK_PLUGIN_FILENAME, "wee_slack.py")
            .unwrap()
            .build()
            .unwrap()
    })
}

pub fn get_config<'a, T: serde::Deserialize<'a>>(key: &str) -> T {
    config().get::<T>(key).unwrap()
}

pub fn get_slack_register_url() -> std::string::String {
    format!(
        "{}?client_id={}&scope={}&redirect_uri={}",
        get_config::<String>(SLACK_REGISTER_BASEURL),
        get_config::<String>(SLACK_REGISTER_WEESLACK_CLIENT_ID),
        get_config::<String>(SLACK_REGISTER_SCOPE),
        get_config::<String>(SLACK_REGISTER_REDIRECT_URI)
    )
}

fn get_home_dir() -> std::path::PathBuf {
    match home::home_dir() {
        Some(path) if !path.as_os_str().is_empty() => path,
        _ => {
            eprintln!("Cannot get your home directory");
            exit(1);
        }
    }
}
