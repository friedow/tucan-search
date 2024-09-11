use crate::Settings;
pub fn view() -> iced::Element<'static, crate::Message> {

    let font_size = Settings::get_or_init().font.size;
    iced::widget::column![iced::widget::horizontal_rule(1)]
        .padding(iced::Padding::from([
            1. * font_size,
            0.,
            0.5 * font_size,
            0.,
        ]))
        // We're fixing the height here to unify it
        // with the height of entries for a smooth
        // scrolling experience
        .height(Settings::entry_height())
        .into()
}
