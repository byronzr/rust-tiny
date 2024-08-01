mod update;
mod view;

#[derive(Debug, Clone)]
pub enum SubMessage {
    None,
    Test,
}

impl super::PageAction<SubMessage> for SubMessage {
    fn update(&self, idx: &mut usize, state: &mut super::State) -> iced::Command<SubMessage> {
        return update::proccess(idx, state);
    }

    fn view<'a>(&self, state: &'a super::State) -> iced::Element<'a, SubMessage> {
        view::show(state)
    }
}
