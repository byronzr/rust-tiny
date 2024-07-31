use iced::Command;

#[derive(Debug, Clone)]
pub enum SubMessage {
    None,
}
impl super::PageAction<SubMessage> for SubMessage {
    fn update(&self, _idx: &mut usize, _state: &mut super::State) -> Command<SubMessage> {
        Command::none()
    }

    fn view(&self, _state: &super::State) -> iced::Element<SubMessage> {
        iced::widget::Text::new("Contact").into()
    }
}
