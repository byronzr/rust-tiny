use iced::Application;
mod app;
mod pages;

#[tokio::main]
async fn main() {
    let settings = iced::settings::Settings {
        window: iced::window::Settings {
            //exit_on_close_request: false, // 自行控制退出
            //decorations: false, // 设置无边框 无标题栏
            size: iced::Size {
                width: 1920f32,
                height: 1080f32,
            },
            ..Default::default()
        },
        ..Default::default()
    };

    let _ = app::TinyApp::run(settings).unwrap();
}
