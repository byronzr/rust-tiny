pub fn show<'a>(state: &'a crate::app::State) -> iced::Element<'a, super::SubMessage> {
    let label = iced::widget::Text::new("关于").font(state.font.unwrap());
    iced::widget::Button::new(label)
        .on_press(super::SubMessage::Test)
        .into()
}
