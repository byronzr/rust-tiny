use crate::pages;
use iced::{
    event::{self, Event},
    font::{self, Font},
    Application, Command,
};
use log::error;

#[derive(Debug, Clone)]
pub enum Message {
    LoadedFont(Result<(), iced::font::Error>),
    Navigate(usize),
    Action(pages::Page),
    EventOccurred(Event),
}

#[derive(Debug)]
pub enum TinyApp {
    Loading,
    Loaded {
        idx: usize,
        state: State,
        pages: Vec<&'static pages::Page>,
    },
}

#[derive(Debug, Default)]
pub struct State {
    pub font: Option<Font>,
}

const FONT_BYTES_MSYH: &[u8] = include_bytes!("msyh.ttc");
include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

impl Application for TinyApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::theme::Theme;

    fn new(_flags: ()) -> (TinyApp, iced::Command<Self::Message>) {
        (
            TinyApp::Loading,
            Command::batch(vec![font::load(FONT_BYTES_MSYH).map(Message::LoadedFont)]),
        )
    }

    fn title(&self) -> String {
        format!("Tiny build: {}", BUILD_DATE)
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match self {
            // TinyApp 状态机 （只处理字体加载）
            TinyApp::Loading => {
                match message {
                    Message::LoadedFont(result) => match result {
                        Ok(_) => {
                            // 加载字体文件成功后，引入字体资源
                            let font = Font::with_name("Microsoft YaHei");
                            // 初始化应用程序
                            let loaded = TinyApp::Loaded {
                                idx: 0,
                                state: State {
                                    font: Some(font),
                                    ..Default::default()
                                },
                                pages: pages::Page::MENU_VEC.iter().map(|(_, p)| p).collect(),
                            };
                            *self = loaded;
                            return self.update(Message::Navigate(0));
                        }
                        Err(error) => {
                            error!("Failed to load font: {:?}", error);
                        }
                    },
                    _ => {}
                }
            }
            // TinyApp 状态机 （处理应用程序逻辑调度）
            // pages 在这里不会被使用到，所以不展开
            TinyApp::Loaded { idx, state, .. } => {
                match message {
                    // 控制导航页面，但业务逻辑依然在 Message::Action 中处理
                    // 通常是在 Message::Action 处理完成后，需要页面跳转
                    Message::Navigate(ndx) => {
                        *idx = ndx;
                    }
                    // 业务逻辑处理
                    Message::Action(pg) => {
                        // page action
                        println!("pt {:p}", &pg);
                        return pg.update(idx, state);
                    }
                    // 事件处理
                    Message::EventOccurred(_event) => {
                        //println!("{:?}", event);
                    }
                    _ => {}
                }
            }
        }

        iced::Command::none()
    }

    fn view(&self) -> iced::Element<Message> {
        match self {
            TinyApp::Loading => iced::Element::new(iced::widget::Text::new("Loading...")),
            TinyApp::Loaded { idx, state, pages } => {
                // 因为 message 无法从 view 中传递，所以这里使用 idx 与 pages 来获取指定页面
                pages[*idx].view(state).into()
            }
        }
    }
    fn theme(&self) -> Self::Theme {
        iced::theme::Theme::Dark
    }
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::batch(vec![event::listen().map(Message::EventOccurred)])
    }
}
