use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct HistoryPlugin {
    entries: Vec<crate::model::Entry>,
}

impl Plugin for HistoryPlugin {
    fn id() -> &'static str {
        "brave_history"
    }

    fn priority() -> u32 {
        0
    }

    fn title() -> &'static str {
        "󰋚 History"
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        self.entries.clone()
    }

    fn set_entries(&mut self, entries: Vec<crate::model::Entry>) {
        self.entries = entries;
    }

    fn new() -> Self {
        Self { entries: vec![] }
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.entries.clear();

        let config_directory = settings::config_directory()?;
        let history_file_path =
            format!("{config_directory}/BraveSoftware/Brave-Browser/Default/History");

        let cache_directory = settings::centerpiece_cache_directory()?;
        let history_cache_file_path = format!("{cache_directory}/brave-history.sqlite");

        std::fs::copy(history_file_path, &history_cache_file_path)
            .context("Error while creating cache directory")?;

        let connection = sqlite::open(history_cache_file_path).unwrap();
        let query = "SELECT title, url FROM urls ORDER BY visit_count DESC, last_visit_time DESC";
        connection.execute(query).unwrap();
        let url_rows = connection
            .prepare(query)
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

        self.entries = url_rows
            .map(|row| {
                let title = row.read::<&str, _>("title");
                let url = row.read::<&str, _>("url");

                crate::model::Entry {
                    id: url.to_string(),
                    title: title.to_string(),
                    action: String::from("open"),
                    meta: String::from("History"),
                    command: None,
                }
            })
            .collect();

        Ok(())
    }

    fn activate(
        &mut self,
        entry: crate::model::Entry,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        std::process::Command::new("brave")
            .arg(&entry.id)
            .spawn()
            .context(format!(
                "Failed to launch brave while activating entry with id '{}'.",
                entry.id
            ))?;

        plugin_channel_out
            .try_send(crate::Message::Exit)
            .context(format!(
                "Failed to send message to exit application while activating entry with id '{}'.",
                entry.id
            ))?;

        Ok(())
    }
}
