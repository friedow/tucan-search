use crate::Settings;
pub const SEARCH_INPUT_ID: &str = "search_input";

pub fn view(query: &str, add_horizontal_rule: bool) -> iced::Element<'static, crate::Message> {
    let font_size = Settings::get_or_init().font.size;
    let prompt_symbol = &Settings::get_or_init().font.prompt_symbol;

    let mut view = iced::widget::column![iced::widget::row![
        iced::widget::container(iced::widget::text(prompt_symbol).size(font_size))
            .padding(iced::Padding::from([0.14 * font_size, 0., 0., 0.])),
        iced::widget::text_input("Search", query)
            .id(iced::widget::text_input::Id::new(SEARCH_INPUT_ID))
            .on_input(crate::Message::Search)
            .size(1. * font_size)
            .style(style())
    ]
    .padding(iced::Padding::from([0.8 * font_size, 1.2 * font_size])),]
    .padding(iced::Padding::from([0., 0., 2., 0.]));

    if add_horizontal_rule {
        view = view.push(iced::widget::horizontal_rule(1));
    }

    view.into()
}

fn style() -> iced::theme::TextInput {
    iced::theme::TextInput::Custom(Box::new(Style {}))
}

pub struct Style {}

impl iced::widget::text_input::StyleSheet for Style {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::text_input::Appearance {
        use iced::color;
        iced::widget::text_input::Appearance {
            background: iced::Background::Color(iced::Color::TRANSPARENT),
            border: iced::Border {
                color: iced::Color::TRANSPARENT,
                width: 0.,
                radius: iced::border::Radius::from(0.),
            },
            icon_color: iced::color!(0xf3f3f3, 1.),
        }
    }

    fn focused(&self, style: &Self::Style) -> iced::widget::text_input::Appearance {
        self.active(style)
    }

    fn disabled(&self, style: &Self::Style) -> iced::widget::text_input::Appearance {
        self.active(style)
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        let color_settings = crate::settings::Settings::get_or_init();
        crate::settings::hexcolor(&color_settings.color.surface)
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        let color_settings = crate::settings::Settings::get_or_init();
        crate::settings::hexcolor(&color_settings.color.text)
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        let color_settings = crate::settings::Settings::get_or_init();
        crate::settings::hexcolor(&color_settings.color.surface)
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        let color_settings = crate::settings::Settings::get_or_init();
        crate::settings::hexcolor(&color_settings.color.surface)
    }
}
