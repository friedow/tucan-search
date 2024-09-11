use crate::Settings;
pub fn view(plugin: &crate::model::Plugin) -> iced::Element<'static, crate::Message> {
    let font_size = Settings::get_or_init().font.size;
    iced::widget::row![iced::widget::text(&plugin.title)
        .font(iced::Font {
            family: iced::font::Family::Name("FiraCode Nerd Font"),
            weight: iced::font::Weight::Light,
            stretch: iced::font::Stretch::Normal,
            style: iced::font::Style::default(),
        })
        .size(0.75 * font_size)]
    // We're fixing the height here to unify it
    // with the height of entries for a smooth
    // scrolling experience
    .height(Settings::entry_height())
    .padding(iced::Padding::from([
        0.8  * font_size,
        1.25 * font_size,
        0.5  * font_size,
        1.25 * font_size,
    ]))
    .into()
}
