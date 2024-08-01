use iced::Command;

#[derive(Debug, Clone)]
pub enum SubMessage {
    None,
}
impl super::PageAction<SubMessage> for SubMessage {
    fn update(&self, _idx: &mut usize, _state: &mut super::State) -> Command<SubMessage> {
        Command::none()
    }

    fn view<'a>(&self, _state: &'a super::State) -> iced::Element<'a, SubMessage> {
        iced::widget::Text::new("Contact").into()
    }
}
