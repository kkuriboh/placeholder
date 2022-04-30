use winit::window::Window;
use winit::platform::unix::EventLoopExtUnix;

pub struct WindowCfg {
    pub title: String,
    pub window: Window,
    pub event_loop: winit::event_loop::EventLoop<()>,
}

impl WindowCfg {
    pub fn new(title: &str) -> Self {
        let event_loop = winit::event_loop::EventLoop::new_any_thread();
        let window = winit::window::WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .unwrap();

        Self {
            title: title.to_string(),
            window,
            event_loop,
        }
    }
}
