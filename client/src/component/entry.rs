use hex_color::HexColor;

pub fn view(entry: &crate::model::Entry, active: bool) -> iced::Element<'static, crate::Message> {
    return iced::widget::container(
        iced::widget::row![
            iced::widget::text(clipped_title(entry.title.clone()))
                .size(1. * crate::REM)
                .width(iced::Length::Fill)
                .shaping(iced::widget::text::Shaping::Advanced),
            iced::widget::text(if active { &entry.action } else { "" }).size(1. * crate::REM)
        ]
        .padding(0.5 * crate::REM),
    )
    .style(style(active))
    .into();
}

fn clipped_title(title: String) -> String {
    if title.char_indices().count() <= 57 {
        return title;
    }

    let mut clipped_title: String = title
        .char_indices()
        .map(|(_, character)| character)
        .take(57)
        .collect();
    clipped_title.push_str("...");
    clipped_title
}

fn style(active: bool) -> iced::theme::Container {
    if active {
        iced::theme::Container::Custom(Box::new(Style {}))
    } else {
        iced::theme::Container::Transparent
    }
}


// TODO don't duplicate function
fn hexcolor(color: &str) -> iced::Color {
    let hex_col = HexColor::parse(color).unwrap_or_else(|_| {
        eprintln!(
            "Failed to parse color settings: {} is not a valid color code",
            color
        );
        std::process::exit(0);
    });

    iced::Color::from_rgba8(hex_col.r, hex_col.g, hex_col.b, (hex_col.a as f32) / 255.0)
}



pub struct Style {}

impl iced::widget::container::StyleSheet for Style {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {

        let color_settings = crate::settings::Settings::new();

        iced::widget::container::Appearance {
            background: None,
            border_radius: iced::BorderRadius::from(0.1 * crate::REM),
            border_width: 1.,
            border_color: hexcolor(&color_settings.color.text),
            text_color: None,
        }
    }
}
