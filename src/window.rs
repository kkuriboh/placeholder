use winit::{
    window::{Window as winitWindow, WindowBuilder as WinitBuilder},
    platform::unix::EventLoopExtUnix
};

pub struct Window;

pub struct WindowCfg<'a> {
    title: Option<&'a str>,
    window: Option<Window>,
    event_loop: Option<winit::event_loop::EventLoop<()>>,
}

impl<'a> WindowCfg<'a> {
    pub fn new() -> Self {
        Self {
            title: None,
            window: None,
            event_loop: None,
        }
    }

    pub fn set_title(self, title: &'a str) -> Self {
        Self {
            title: Some(title),
            ..self
        }
    }

    pub fn build(self) {}

}
