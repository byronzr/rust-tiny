use iced::Command;

#[derive(Debug, Clone)]

pub enum SubMessage {
    None,
    Test,
}

impl super::PageAction<SubMessage> for SubMessage {
    fn update(&self, _idx: &mut usize, _state: &mut super::State) -> iced::Command<SubMessage> {
        match self {
            SubMessage::Test => {
                println!("hello");
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&self, _state: &super::State) -> iced::Element<SubMessage> {
        let label = iced::widget::Text::new("关于").font(_state.font.unwrap());
        iced::widget::Button::new(label)
            .on_press(SubMessage::Test)
            .into()
    }
}
