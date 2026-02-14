use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
    window::WindowAttributes,
};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_decorations(false)
            .with_blur(false)
            .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));

        let window = event_loop
            .create_window(window_attributes)
            .unwrap();

        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }
    }
}

pub fn test() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}