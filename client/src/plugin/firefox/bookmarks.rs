use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct BookmarksPlugin {
    entries: Vec<crate::model::Entry>,
}

impl Plugin for BookmarksPlugin {
    fn id() -> &'static str {
        "firefox_bookmarks"
    }

    fn priority() -> u32 {
        26
    }

    fn title() -> &'static str {
        "󰃃 Bookmarks"
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
        let profile_path = crate::plugin::firefox::utils::profile_path()?;
        let bookmarks_file_path = format!("{profile_path}/places.sqlite");
        let cache_directory = settings::centerpiece_cache_directory()?;
        let bookmarks_cache_file_path = format!("{cache_directory}/firefox-bookmarks.sqlite");

        std::fs::copy(bookmarks_file_path, &bookmarks_cache_file_path)
            .context("Error while creating cache directory")?;

        let connection = sqlite::open(bookmarks_cache_file_path)?;
        let query = "
            SELECT moz_bookmarks.title, moz_places.url
            FROM 
        	    moz_bookmarks
            	LEFT JOIN moz_places
                ON moz_bookmarks.fk = moz_places.id
            WHERE moz_bookmarks.type = 1
            ORDER BY moz_places.visit_count DESC";

        connection.execute(query)?;
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
                    meta: String::from("Bookmarks"),
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
        std::process::Command::new("firefox")
            .arg(&entry.id)
            .spawn()
            .context(format!(
                "Failed to launch firefox while activating entry with id '{}'.",
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
