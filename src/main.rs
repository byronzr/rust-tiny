use iced::Application;
mod app;
mod pages;

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init().expect("Initialize logger");
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    let settings = iced::settings::Settings {
        window: iced::window::Settings {
            //exit_on_close_request: false, // 自行控制退出
            //decorations: false, // 设置无边框 无标题栏
            size: iced::Size {
                width: 800f32,
                height: 600f32,
            },
            ..Default::default()
        },
        ..Default::default()
    };

    let _ = app::TinyApp::run(settings).unwrap();
}
