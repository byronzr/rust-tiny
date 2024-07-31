use crate::pages;
use iced::{
    event::{self, Event},
    font::{self, Font},
    Application, Command,
};
use log::error;

// App 消息,
// Action 会传递给页面处理，Navigate 用于控制页面跳转
#[derive(Debug, Clone)]
pub enum Message {
    LoadedFont(Result<(), iced::font::Error>), // 字体加载
    Navigate(usize),                           // 页面导航
    Action(pages::Page),                       // 传递给页面处理
    EventOccurred(Event),                      // 事件处理(键盘等)
}

// App 状态机
#[derive(Debug)]
pub enum TinyApp {
    // Loading 状态
    Loading,
    // Loaded 状态
    Loaded {
        idx: usize,                       // 当前所在页面
        state: State,                     // 共享数据
        pages: Vec<&'static pages::Page>, // 所有页面实例集合
    },
}

// 共享数据
#[derive(Debug, Default)]
pub struct State {
    pub font: Option<Font>,
}

// 字体加载(字体文件在编译时被包含进来)
const FONT_BYTES_MSYH: &[u8] = include_bytes!("msyh.ttc");

// 构建编译时可能用的信息
include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

// 实现 TinyApp
impl Application for TinyApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::theme::Theme;

    // 初始化应用程序
    fn new(_flags: ()) -> (TinyApp, iced::Command<Self::Message>) {
        (
            TinyApp::Loading,
            Command::batch(vec![font::load(FONT_BYTES_MSYH).map(Message::LoadedFont)]),
        )
    }

    // 设置标题
    fn title(&self) -> String {
        format!("Tiny build: {}", BUILD_DATE)
    }

    // 处理(状态与消息）
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

    // 处理界面显示
    fn view(&self) -> iced::Element<Message> {
        match self {
            TinyApp::Loading => iced::Element::new(iced::widget::Text::new("Loading...")),
            TinyApp::Loaded { idx, state, pages } => {
                // 因为 message 无法从 view 中传递，所以这里使用 idx 与 pages 来获取指定页面
                pages[*idx].view(state).into()
            }
        }
    }

    // 控制主题
    fn theme(&self) -> Self::Theme {
        iced::theme::Theme::Dark
    }

    // 订阅事件
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::batch(vec![event::listen().map(Message::EventOccurred)])
    }
}
