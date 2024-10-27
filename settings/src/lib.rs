use serde::Deserialize;
use std::sync::OnceLock;

pub mod cli;

pub fn config_directory() -> anyhow::Result<String> {
    let home_directory = std::env::var("HOME")?;
    let config_in_home = format!("{home_directory}/.config");
    Ok(std::env::var("XDG_CONFIG_HOME").unwrap_or(config_in_home))
}

pub fn centerpiece_default_config_path() -> anyhow::Result<String> {
    let config_directory = centerpiece_config_directory()?;
    Ok(format!("{config_directory}/config.yml"))
}

pub fn centerpiece_config_directory() -> anyhow::Result<String> {
    let config_directory = config_directory()?;
    Ok(format!("{config_directory}/centerpiece"))
}

pub fn cache_directory() -> anyhow::Result<String> {
    let home_directory = std::env::var("HOME")?;
    let cache_in_home = format!("{home_directory}/.cache");
    Ok(std::env::var("XDG_CACHE_HOME").unwrap_or(cache_in_home))
}

pub fn centerpiece_cache_directory() -> anyhow::Result<String> {
    let cache_directory = cache_directory()?;
    Ok(format!("{cache_directory}/centerpiece"))
}

pub fn hexcolor(color: &str) -> iced::Color {
    let hex_col = hex_color::HexColor::parse(color).unwrap_or_else(|_| {
        eprintln!(
            "Failed to parse color settings: {} is not a valid color code",
            color
        );
        std::process::exit(0);
    });

    iced::Color::from_rgba8(hex_col.r, hex_col.g, hex_col.b, (hex_col.a as f32) / 255.0)
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct ApplicationsPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for ApplicationsPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct BraveBookmarksPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for BraveBookmarksPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct BraveHistoryPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for BraveHistoryPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct BraveProgressiveWebAppsSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for BraveProgressiveWebAppsSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct ClockPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for ClockPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct FirefoxBookmarksPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for FirefoxBookmarksPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct FirefoxHistoryPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for FirefoxHistoryPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct GitRepositoriesPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
    #[serde(default = "default_true")]
    pub zoxide: bool,
    #[serde(default = "default_commands")]
    pub commands: Vec<Vec<String>>,
}

fn default_white() -> String {
    "#ffffff".into()
}

fn default_black() -> String {
    "#000000".into()
}

fn default_deprecated() -> String {
    "deprecated".into()
}

#[derive(Debug, Deserialize)]
pub struct ColorSettings {
    #[serde(default = "default_white")]
    pub text: String,
    #[serde(default = "default_black")]
    pub background: String,
    #[deprecated(
        since = "1.2.0",
        note = "color.surface has been replaced by automatic shading of the background color. Please remove this field from your configuration."
    )]
    #[serde(default = "default_deprecated")]
    pub surface: String,
}

impl Default for ColorSettings {
    fn default() -> Self {
        #[allow(deprecated)]
        Self {
            text: default_white(),
            background: default_black(),
            surface: default_deprecated(),
        }
    }
}

fn default_commands() -> Vec<Vec<String>> {
    vec![
        vec![
            "alacritty".into(),
            "--command".into(),
            "nvim".into(),
            "$GIT_DIRECTORY".into(),
        ],
        vec![
            "alacritty".into(),
            "--working-directory".into(),
            "$GIT_DIRECTORY".into(),
        ],
    ]
}

impl Default for GitRepositoriesPluginSettings {
    fn default() -> Self {
        Self {
            enable: true,
            zoxide: true,
            commands: default_commands(),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct GitmojiPluginSettings {
    #[serde(default = "default_false")]
    pub enable: bool,
}

#[derive(Debug, Deserialize)]
pub struct ResourceMonitorBatteryPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for ResourceMonitorBatteryPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct ResourceMonitorCpuPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for ResourceMonitorCpuPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct ResourceMonitorDisksSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for ResourceMonitorDisksSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct ResourceMonitorMemoryPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for ResourceMonitorMemoryPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct SystemPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for SystemPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct WifiPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for WifiPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct SwayWindowsPluginSettings {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for SwayWindowsPluginSettings {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct PluginSettings {
    #[serde(default)]
    pub applications: ApplicationsPluginSettings,
    #[serde(default)]
    pub brave_bookmarks: BraveBookmarksPluginSettings,
    #[serde(default)]
    pub brave_history: BraveHistoryPluginSettings,
    #[serde(default)]
    pub brave_progressive_web_apps: BraveProgressiveWebAppsSettings,
    #[serde(default)]
    pub clock: ClockPluginSettings,
    #[serde(default)]
    pub firefox_bookmarks: FirefoxBookmarksPluginSettings,
    #[serde(default)]
    pub firefox_history: FirefoxHistoryPluginSettings,
    #[serde(default)]
    pub git_repositories: GitRepositoriesPluginSettings,
    #[serde(default)]
    pub gitmoji: GitmojiPluginSettings,
    #[serde(default)]
    pub resource_monitor_battery: ResourceMonitorBatteryPluginSettings,
    #[serde(default)]
    pub resource_monitor_cpu: ResourceMonitorCpuPluginSettings,
    #[serde(default)]
    pub resource_monitor_disks: ResourceMonitorDisksSettings,
    #[serde(default)]
    pub resource_monitor_memory: ResourceMonitorMemoryPluginSettings,
    #[serde(default)]
    pub sway_windows: SwayWindowsPluginSettings,
    #[serde(default)]
    pub system: SystemPluginSettings,
    #[serde(default)]
    pub wifi: WifiPluginSettings,
}

#[derive(Debug, Default, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub plugin: PluginSettings,
    #[serde(default)]
    pub color: ColorSettings,
}

impl Settings {
    pub fn new() -> Self {
        let config_directory_result = centerpiece_config_directory();
        if let Err(error) = config_directory_result {
            log::error!(
            error = log::error!("{:?}", error);
            "Unable to find config directory.",
            );
            panic!();
        }
        let config_directory = config_directory_result.unwrap();
        let config_file_path = format!("{config_directory}/config.yml");

        let config_file_result = std::fs::File::open(config_file_path);
        if config_file_result.is_err() {
            log::info!("No custom config file found, falling back to default.");
            return Self::default();
        }
        let config_file = config_file_result.unwrap();
        let config_result = serde_yaml::from_reader(config_file);
        if let Err(error) = config_result {
            log::error!(
            error = log::error!("{:?}", error);
            "Config file does not match settings struct.",
            );
            panic!();
        }
        let settings: Settings = config_result.unwrap();

        #[allow(deprecated)]
        if settings.color.surface != String::from("deprecated") {
            log::warn!("color.surface has been replaced by automatic shading of the background color in cernterpiece version 1.2.0. Please remove this field from your configuration.")
        }

        settings
    }

    pub fn get_or_init() -> &'static Self {
        static SETTINGS: OnceLock<Settings> = OnceLock::new();
        SETTINGS.get_or_init(Self::new)
    }
}

impl std::convert::TryFrom<crate::cli::CliArgs> for Settings {
    type Error = anyhow::Error;

    fn try_from(args: crate::cli::CliArgs) -> Result<Self, Self::Error> {
        let maybe_config_file_path = args.config;
        let config_file_path = maybe_config_file_path.unwrap_or_else(|| {
            centerpiece_default_config_path().unwrap_or_else(|error| {
                log::error!(
                    error = log::error!("{:?}", error);
                    "Unable to find default config file.",
                );
                panic!();
            })
        });
        let config_file_result = std::fs::File::open(config_file_path);
        if config_file_result.is_err() {
            log::info!("No custom config file found, falling back to default.");
            return Ok(Self::default());
        }
        let config_file = config_file_result?;
        let config_result = serde_yaml::from_reader(config_file);
        if let Err(ref error) = config_result {
            log::error!(
            error = log::error!("{:?}", error);
            "Config file does not match settings struct.",
            );
        }
        Ok(config_result?)
    }
}
