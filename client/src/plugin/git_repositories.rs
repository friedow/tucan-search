use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct GitRepositoriesPlugin {
    entries: Vec<crate::model::Entry>,
    settings: settings::Settings,
}

impl Plugin for GitRepositoriesPlugin {
    fn id() -> &'static str {
        "git_repositories"
    }

    fn priority() -> u32 {
        28
    }

    fn title() -> &'static str {
        "󰘬 Git Repositories"
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        self.entries.clone()
    }

    fn set_entries(&mut self, entries: Vec<crate::model::Entry>) {
        self.entries = entries;
    }

    fn new() -> Self {
        Self {
            entries: vec![],
            settings: settings::Settings::new(),
        }
    }

    // This lint seems to be a false positive
    #[allow(clippy::unnecessary_filter_map)]
    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.entries.clear();

        let git_repository_paths: Vec<String> =
            crate::plugin::utils::read_index_file("git-repositories-index.json")?;

        let home = std::env::var("HOME").unwrap_or(String::from(""));

        let entries = git_repository_paths
            .into_iter()
            .filter_map(|git_repository_path| {
                let git_repository_display_name = git_repository_path.replacen(&home, "~", 1);

                Some(crate::model::Entry {
                    id: git_repository_path,
                    title: git_repository_display_name,
                    action: String::from("focus"),
                    meta: String::from("Git Repositories"),
                    command: None,
                })
            })
            .collect::<Vec<_>>();

        self.set_entries(entries);
        self.sort();

        Ok(())
    }

    fn activate(
        &mut self,
        entry: crate::model::Entry,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        for command in self.settings.plugin.git_repositories.commands.clone() {
            let parsed_command: Vec<String> = command
                .into_iter()
                .map(|command_part| match command_part.as_ref() {
                    "$GIT_DIRECTORY" => entry.id.clone(),
                    "$GIT_DIRECTORY_NAME" => std::path::Path::new(&entry.id)
                        .file_name()
                        // We match on a git directory
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .into(),
                    _ => command_part,
                })
                .collect();
            std::process::Command::new(&parsed_command[0])
                .args(&parsed_command[1..])
                .spawn()?;
        }

        plugin_channel_out
            .try_send(crate::Message::Exit)
            .context(format!(
                "Failed to send message to exit application while activating entry with id '{}'.",
                entry.id
            ))?;

        Ok(())
    }

    fn sort(&mut self) {
        let mut entries = self.entries.clone();
        entries.sort_by_key(|entry| entry.title.clone());
        self.set_entries(entries);

        if self.use_zoxide() {
            match Zoxide::query() {
                Ok(zoxide) => self.sort_with_zoxide(zoxide),
                Err(e) => log::warn!("Zoxide Error: {}", e),
            }
        }
    }
}

impl GitRepositoriesPlugin {
    fn use_zoxide(&self) -> bool {
        self.settings.plugin.git_repositories.zoxide
    }
    /// Sorts the returned paths, by their respective zoxide query score
    fn sort_with_zoxide(&mut self, index: Zoxide) {
        let mut scored_entries: Vec<(crate::model::Entry, f64)> = self
            .entries()
            .into_iter()
            .map(|entry| {
                let score = index
                    .scored_paths
                    .iter()
                    .find(|zoxide_result| zoxide_result.path == entry.id)
                    .map_or(0.0, |zoxide_result| zoxide_result.score);
                (entry.clone(), score)
            })
            .collect();

        scored_entries.sort_by(|(_, score1), (_, score2)| {
            score2
                .partial_cmp(score1)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        self.entries = scored_entries.into_iter().map(|(entry, _)| entry).collect();
    }
}

#[derive(Debug)]
pub struct Zoxide {
    scored_paths: Vec<ScoredPath>,
}

impl Zoxide {
    pub fn query() -> anyhow::Result<Self> {
        let out = std::process::Command::new("zoxide")
            .args(["query", "--score", "--list"])
            .output()
            .map_err(|e| anyhow::anyhow!("Failed to execute zoxide query command:\n{e}"))?;

        if !out.status.success() {
            return Err(anyhow::anyhow!(
                "Zoxide query command returned non-zero exit status."
            ));
        }

        let stdout = String::from_utf8(out.stdout)
            .map_err(|_| anyhow::anyhow!("Failed to convert output to utf8"))?;

        let res = stdout
            .lines()
            .map(ScoredPath::parse_line)
            .collect::<anyhow::Result<_>>()?;

        Ok(Self { scored_paths: res })
    }
}

#[derive(Debug)]
struct ScoredPath {
    path: String,
    score: f64,
}

impl ScoredPath {
    fn parse_line(line: &str) -> anyhow::Result<ScoredPath> {
        let (unparsed_score, path): (&str, &str) = line
            .trim()
            .split_once(' ')
            .ok_or(anyhow::anyhow!("Invalid zoxide query result line: {line}"))?;

        let score = unparsed_score
            .parse::<f64>()
            .map_err(|_| anyhow::anyhow!("Failed to parse score"))?;

        Ok(ScoredPath {
            path: path.into(),
            score,
        })
    }
}
