use winit::window::Window;

pub struct WindowCfg {
    pub title: String,
    pub window: Window,
    pub event_loop: winit::event_loop::EventLoop<()>,
}

impl WindowCfg {
    pub fn new(title: &str) -> Self {
        let event_loop = winit::event_loop::EventLoop::new();
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
