use iced::widget::{column, row};
use tiny_macro::TinyPageMacro;

use crate::app::{Message, State};

pub mod about;
pub mod contact;
pub mod home;

pub trait PageAction<T> {
    fn update(&self, idx: &mut usize, state: &mut State) -> iced::Command<T>;
    fn view<'a>(&self, state: &'a State) -> iced::Element<'a, T>;
}

// update 宏，用于简化 update 方法的实现
// match self {
//     Page::Home(sub_message) => sub_message.update(idx, state).map(Page::Home),
//     Page::About(sub_message) => sub_message.update(idx, state).map(Page::About),
//     Page::Contact(sub_message) => sub_message.update(idx, state).map(Page::Contact),
// }
// .map(Message::Action)
// macro_rules! page_update {
//     ($self:expr,$idx:expr,$state:expr,$($page:path),+) => {
//         match $self {
//             $($page(sub_message) => sub_message.update($idx, $state).map($page),)*
//         }
//         .map(Message::Action)
//     };
// }

// subview 宏，用于简化 subview 方法的实现
// match self {
//     Page::Home(sub_message) => sub_message.view(state).map(Page::Home),
//     Page::About(sub_message) => sub_message.view(state).map(Page::About),
//     Page::Contact(sub_message) => sub_message.view(state).map(Page::Contact),
// }
// .map(Message::Action)
// macro_rules! page_subview {
//     ($self:expr,$state:expr,$($page:path),+) => {
//         match $self {
//             $($page(sub_message) => sub_message.view($state).map($page),)*
//         }
//         .map(Message::Action)
//     };
// }

#[derive(Debug, Clone, TinyPageMacro)]
pub enum Page {
    Home(home::SubMessage),
    About(about::SubMessage),
    Contact(contact::SubMessage),
}

// ------ impl Page ------
impl Page {
    // 集中一个地方，定义所有页面,用常量数组取代
    // pub fn to_vec() -> Vec<(Page, &'static str)> {
    //     vec![
    //         (Page::Home(home::SubMessage::None), "Home"),
    //         (Page::About(about::SubMessage::None), "About"),
    //         (Page::Contact(contact::SubMessage::None), "Contact"),
    //     ]
    // }
    // 顶部菜单与页面实例的映射,以常量的方式初始化
    pub const MENU_VEC: &'static [(&'static str, Page)] = &[
        ("Home", Page::Home(home::SubMessage::None)),
        ("About", Page::About(about::SubMessage::None)),
        ("Contact", Page::Contact(contact::SubMessage::None)),
    ];

    // page 的 update 方法
    // 已经由过程宏实现,安置在 tiny_macro/src/lib.rs
    // pub fn update(&self, idx: &mut usize, state: &mut State) -> iced::Command<Message> {
    //     //page_update!(self, idx, state, Page::Home, Page::About, Page::Contact)
    //     self.variants(idx, state)
    // }

    // 在大布局中的子视图
    // 已经由过程宏实现,安置在 tiny_macro/src/lib.rs
    // pub fn subview(&self, state: &State) -> iced::Element<Message> {
    //     page_subview!(self, state, Page::Home, Page::About, Page::Contact)
    // }

    // 承担 layout 的 view 方法
    pub fn view<'a>(&self, state: &'a State, ndx: usize) -> iced::Element<'a, Message> {
        // 顶部菜单
        let menu = Page::MENU_VEC.iter().enumerate().fold(
            row![].spacing(5),
            |acc, (idx, (title, _page))| {
                let button = iced::widget::Button::new(iced::widget::Text::new(title.to_string()))
                    .padding([5, 15]);

                let button = if idx == ndx {
                    button
                } else {
                    button.on_press(Message::Navigate(idx))
                };

                acc.push(button)
            },
        );

        // 子视图
        let subview = self.subview(state);
        // 布局
        let layout = column![menu, subview].padding(10).spacing(10);
        layout.into()
    }
}
