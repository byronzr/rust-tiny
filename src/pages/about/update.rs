use iced::Command;

pub fn proccess<'a>(
    idx: &mut usize,
    state: &'a mut crate::app::State,
) -> iced::Command<super::SubMessage> {
    println!("idx: {} state: {:?}", idx, state);
    Command::none()
}
