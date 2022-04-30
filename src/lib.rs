pub mod window;

use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

use window::WindowCfg;

pub fn make_win() {
    let window_cfg = WindowCfg::new("development-window");

    window_cfg.event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window_cfg.window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
