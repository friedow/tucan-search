use clap::Parser;

mod component;
mod model;
mod plugin;

pub fn main() -> iced::Result {
    let args = settings::cli::CliArgs::parse();
    settings::Settings::try_from(args).unwrap_or_else(|_| {
        eprintln!("There is an issue with the settings, please check the configuration file.");
        std::process::exit(1);
    });

    simple_logger::init_with_level(log::Level::Info).unwrap();
    iced::application("Centerpiece", update, view)
        .decorations(false)
        .level(iced::window::Level::AlwaysOnTop)
        .position(iced::window::Position::Centered)
        .resizable(false)
        .settings(settings())
        .style(style)
        .subscription(subscription)
        .theme(theme)
        .transparent(true)
        .window_size([650., 380.])
        .run_with(Centerpiece::new)
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded,
    Search(String),
    Event(iced::Event),
    FontLoaded(Result<(), iced::font::Error>),
    RegisterPlugin(model::Plugin),
    UpdateEntries(String, Vec<model::Entry>),
    Exit,
}

struct Centerpiece {
    query: String,
    active_entry_index: usize,
    plugins: Vec<model::Plugin>,
}

pub const APP_ID: &str = "centerpiece";

fn update(centerpiece: &mut Centerpiece, message: Message) -> iced::Task<Message> {
    match message {
        Message::Loaded => focus_search_input(),

        Message::Search(input) => centerpiece.search(input),

        Message::Event(event) => match event {
            iced::Event::Keyboard(event) => match event {
                iced::keyboard::Event::KeyPressed { key, modifiers, .. } => {
                    if let iced::keyboard::Modifiers::CTRL = modifiers {
                        return match key.as_ref() {
                            iced::keyboard::Key::Character("j") => centerpiece.select_next_entry(),
                            iced::keyboard::Key::Character("k") => {
                                centerpiece.select_previous_entry()
                            }
                            iced::keyboard::Key::Character("n") => centerpiece.select_next_plugin(),
                            iced::keyboard::Key::Character("p") => {
                                centerpiece.select_previous_plugin()
                            }
                            _ => iced::Task::none(),
                        };
                    }
                    match key.as_ref() {
                        iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowUp) => {
                            centerpiece.select_previous_entry()
                        }
                        iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowDown) => {
                            centerpiece.select_next_entry()
                        }
                        iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) => {
                            centerpiece
                                .activate_selected_entry()
                                .unwrap_or(iced::Task::none())
                        }
                        _ => iced::Task::none(),
                    }
                }
                iced::keyboard::Event::KeyReleased { key, .. } => {
                    if key == iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) {
                        return iced::window::get_latest().and_then(iced::window::close);
                    }
                    iced::Task::none()
                }

                _ => iced::Task::none(),
            },

            iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) => {
                focus_search_input()
            }

            _ => iced::Task::none(),
        },

        Message::FontLoaded(_) => iced::Task::none(),

        Message::RegisterPlugin(plugin) => centerpiece.register_plugin(plugin),

        Message::UpdateEntries(plugin_id, entries) => {
            centerpiece.update_entries(plugin_id, entries)
        }

        Message::Exit => iced::window::get_latest().and_then(iced::window::close),
    }
}

fn view(centerpiece: &Centerpiece) -> iced::Element<Message> {
    let entries = centerpiece.entries();

    let mut lines = iced::widget::column![];
    let mut divider_added = true;
    let mut header_added = false;
    let mut next_entry_index_to_add = centerpiece.active_entry_index;

    for lines_added in 0..11 {
        if next_entry_index_to_add >= entries.len() {
            break;
        }

        let mut plugin_to_add = None;
        let mut last_plugin_start_index = 0;
        for plugin in centerpiece.plugins.iter() {
            if last_plugin_start_index == next_entry_index_to_add {
                plugin_to_add = Some(plugin);
            }
            last_plugin_start_index += plugin.entries.len();
        }

        if !divider_added && plugin_to_add.is_some() {
            lines = lines.push(component::divider::view());
            divider_added = true;
            continue;
        }

        if !header_added && plugin_to_add.is_some() {
            lines = lines.push(component::plugin_header::view(plugin_to_add.unwrap()));
            header_added = true;
            continue;
        } else if lines_added == 0 {
            lines = lines.push(component::entry::view(
                entries[next_entry_index_to_add - 1],
                false,
            ));
        }

        lines = lines.push(component::entry::view(
            entries[next_entry_index_to_add],
            next_entry_index_to_add == centerpiece.active_entry_index,
        ));
        divider_added = false;
        header_added = false;
        next_entry_index_to_add += 1;
    }

    iced::widget::container(iced::widget::column![
        component::query_input::view(&centerpiece.query, !entries.is_empty()),
        lines
    ])
    .style(|theme: &iced::Theme| {
        let palette = theme.extended_palette();

        iced::widget::container::Style {
            background: Some(iced::Background::Color(palette.background.base.color)),
            border: iced::Border::default().rounded(0.25 * crate::REM),
            ..Default::default()
        }
    })
    .padding(iced::padding::bottom(0.75 * crate::REM))
    .into()
}

fn subscription(_centerpiece: &Centerpiece) -> iced::Subscription<Message> {
    let mut subscriptions = vec![iced::event::listen_with(
        |event, _status, _id| match event {
            iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { .. }) => {
                Some(Message::Event(event))
            }
            iced::Event::Keyboard(iced::keyboard::Event::KeyReleased { .. }) => {
                Some(Message::Event(event))
            }
            iced::Event::Mouse(iced::mouse::Event::ButtonPressed(_)) => Some(Message::Event(event)),
            _ => None,
        },
    )];

    let settings = settings::Settings::get_or_init();

    if settings.plugin.applications.enable {
        subscriptions.push(crate::plugin::utils::spawn::<
            crate::plugin::applications::ApplicationsPlugin,
        >());
    }

    if settings.plugin.brave_bookmarks.enable {
        subscriptions.push(crate::plugin::utils::spawn::<
            crate::plugin::brave::bookmarks::BookmarksPlugin,
        >());
    }

    if settings.plugin.brave_progressive_web_apps.enable {
        subscriptions.push(crate::plugin::utils::spawn::<
            crate::plugin::brave::progressive_web_apps::ProgressiveWebAppsPlugin,
        >());
    }

    if settings.plugin.brave_history.enable {
        subscriptions.push(crate::plugin::utils::spawn::<
            crate::plugin::brave::history::HistoryPlugin,
        >());
    }

    if settings.plugin.clock.enable {
        subscriptions.push(crate::plugin::utils::spawn::<
            crate::plugin::clock::ClockPlugin,
        >());
    }

    if settings.plugin.firefox_bookmarks.enable {
        subscriptions.push(crate::plugin::utils::spawn::<
            crate::plugin::firefox::bookmarks::BookmarksPlugin,
        >());
    }

    if settings.plugin.firefox_history.enable {
        subscriptions.push(crate::plugin::utils::spawn::<
            crate::plugin::firefox::history::HistoryPlugin,
        >());
    }

    if settings.plugin.git_repositories.enable {
        subscriptions.push(crate::plugin::utils::spawn::<
            crate::plugin::git_repositories::GitRepositoriesPlugin,
        >());
    }

    if settings.plugin.gitmoji.enable {
        subscriptions.push(crate::plugin::utils::spawn::<
            crate::plugin::gitmoji::GitmojiPlugin,
        >());
    }

    if settings.plugin.resource_monitor_battery.enable {
        subscriptions.push(crate::plugin::utils::spawn::<
            crate::plugin::resource_monitor::battery::BatteryPlugin,
        >());
    }

    if settings.plugin.resource_monitor_cpu.enable {
        subscriptions.push(crate::plugin::utils::spawn::<
            crate::plugin::resource_monitor::cpu::CpuPlugin,
        >());
    }

    if settings.plugin.resource_monitor_disks.enable {
        subscriptions.push(crate::plugin::utils::spawn::<
            crate::plugin::resource_monitor::disks::DisksPlugin,
        >());
    }

    if settings.plugin.resource_monitor_memory.enable {
        subscriptions.push(crate::plugin::utils::spawn::<
            crate::plugin::resource_monitor::memory::MemoryPlugin,
        >());
    }

    if settings.plugin.system.enable {
        subscriptions.push(crate::plugin::utils::spawn::<
            crate::plugin::system::SystemPlugin,
        >());
    }

    if settings.plugin.wifi.enable {
        subscriptions.push(crate::plugin::utils::spawn::<crate::plugin::wifi::WifiPlugin>());
    }

    if settings.plugin.sway_windows.enable {
        subscriptions.push(crate::plugin::utils::spawn::<
            crate::plugin::sway_windows::SwayWindowsPlugin,
        >());
    }

    iced::Subscription::batch(subscriptions)
}

fn theme(_centerpiece: &Centerpiece) -> iced::Theme {
    let settings = settings::Settings::get_or_init();
    iced::Theme::custom(
        "centerpiece theme".to_string(),
        iced::theme::Palette {
            background: settings::hexcolor(&settings.color.background),
            text: settings::hexcolor(&settings.color.text),
            primary: settings::hexcolor(&settings.color.text),
            success: settings::hexcolor(&settings.color.text),
            danger: settings::hexcolor(&settings.color.text),
        },
    )
}

fn style(_centerpiece: &Centerpiece, _theme: &iced::Theme) -> iced::application::Appearance {
    let color_settings = settings::Settings::get_or_init();

    iced::application::Appearance {
        background_color: iced::Color::TRANSPARENT,
        text_color: settings::hexcolor(&color_settings.color.text),
    }
}

fn settings() -> iced::Settings {
    iced::Settings {
        id: Some(APP_ID.into()),
        default_font: iced::Font {
            family: iced::font::Family::Name("FiraCode Nerd Font"),
            weight: iced::font::Weight::Normal,
            stretch: iced::font::Stretch::Normal,
            style: iced::font::Style::default(),
        },
        default_text_size: iced::Pixels(crate::REM),
        ..Default::default()
    }
}

fn focus_search_input() -> iced::Task<Message> {
    iced::widget::text_input::focus(iced::widget::text_input::Id::new(
        component::query_input::SEARCH_INPUT_ID,
    ))
}

impl Centerpiece {
    fn new() -> (Self, iced::Task<Message>) {
        (
            Self {
                query: String::from(""),
                active_entry_index: 0,
                plugins: vec![],
            },
            iced::Task::batch(vec![
                iced::font::load(
                    include_bytes!("../assets/FiraCode/FiraCodeNerdFont-Regular.ttf").as_slice(),
                )
                .map(Message::FontLoaded),
                iced::font::load(
                    include_bytes!("../assets/FiraCode/FiraCodeNerdFont-Light.ttf").as_slice(),
                )
                .map(Message::FontLoaded),
                iced::Task::perform(async {}, move |()| Message::Loaded),
            ]),
        )
    }

    fn entries(&self) -> Vec<&model::Entry> {
        return self
            .plugins
            .iter()
            .flat_map(|plugin| &plugin.entries)
            .collect();
    }

    fn active_entry_id(&self) -> Option<&String> {
        let entries = self.entries();
        let active_entry = entries.get(self.active_entry_index);
        match active_entry {
            Some(entry) => Some(&entry.id),
            None => None,
        }
    }

    fn search(&mut self, input: String) -> iced::Task<Message> {
        for plugin in self.plugins.iter_mut() {
            let _ = plugin
                .app_channel_out
                .try_send(crate::model::PluginRequest::Search(input.clone()));
        }

        self.query = input;
        self.select_first_entry()
    }

    fn select_first_entry(&mut self) -> iced::Task<Message> {
        self.active_entry_index = 0;
        iced::Task::none()
    }

    fn select_previous_entry(&mut self) -> iced::Task<Message> {
        let entries = self.entries();
        if entries.is_empty() {
            return self.select_first_entry();
        }

        if self.active_entry_index == 0 {
            self.active_entry_index = entries.len() - 1;
            return iced::Task::none();
        }

        self.active_entry_index -= 1;
        iced::Task::none()
    }

    fn select_next_entry(&mut self) -> iced::Task<Message> {
        let entries = self.entries();
        if entries.is_empty() || self.active_entry_index == entries.len() - 1 {
            return self.select_first_entry();
        }

        self.active_entry_index += 1;
        iced::Task::none()
    }

    fn select_next_plugin(&mut self) -> iced::Task<Message> {
        let accumulated_entries = self
            .plugins
            .iter()
            .map(|plugin| plugin.entries.len())
            .scan(0, |acc, len| {
                let prev = *acc;
                *acc += len;
                Some(prev)
            })
            .find(|&total| total > self.active_entry_index)
            .unwrap_or(self.active_entry_index);

        self.active_entry_index = accumulated_entries;
        iced::Task::none()
    }

    fn select_previous_plugin(&mut self) -> iced::Task<Message> {
        if self.plugins.is_empty() || self.active_entry_index == 0 {
            return self.select_first_entry();
        }

        let accumulated_entries = self
            .plugins
            .iter()
            .map(|plugin| plugin.entries.len())
            .scan(0, |acc, len| {
                let prev = *acc;
                *acc += len;
                Some(prev)
            })
            .take_while(|&total| total < self.active_entry_index)
            .last()
            .unwrap_or(0);

        self.active_entry_index = accumulated_entries;
        iced::Task::none()
    }

    fn register_plugin(&mut self, mut plugin: crate::model::Plugin) -> iced::Task<Message> {
        let _ = plugin
            .app_channel_out
            .try_send(crate::model::PluginRequest::Search(self.query.clone()));
        self.plugins.push(plugin);
        self.plugins.sort_by(|a, b| b.priority.cmp(&a.priority));
        iced::Task::none()
    }

    fn update_entries(
        &mut self,
        plugin_id: String,
        entries: Vec<crate::model::Entry>,
    ) -> iced::Task<Message> {
        let plugin = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.id == plugin_id);
        if plugin.is_none() {
            println!(
                "Appending entry failed. Could not find plugin with id {:?}",
                plugin_id
            );
            return iced::Task::none();
        }

        let plugin = plugin.unwrap();
        plugin.entries = entries;
        iced::Task::none()
    }

    fn activate_selected_entry(&mut self) -> Option<iced::Task<Message>> {
        let active_entry_id = self.active_entry_id()?.clone();

        let entry = self
            .entries()
            .into_iter()
            .find(|entry| entry.id == *active_entry_id)?
            .clone();

        let plugin = self.plugins.iter_mut().find(|plugin| {
            plugin
                .entries
                .iter()
                .any(|entry| entry.id == *active_entry_id)
        })?;

        plugin
            .app_channel_out
            .try_send(model::PluginRequest::Activate(entry))
            .ok();
        Some(iced::Task::none())
    }
}

pub const REM: f32 = 14.0;
pub const ENTRY_HEIGHT: f32 = 2.3 * crate::REM;
