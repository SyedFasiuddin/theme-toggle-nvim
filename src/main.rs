use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::window::Theme;
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    if let Some(theme) = window.theme() {
        match theme {
            Theme::Light => println!("light"),
            Theme::Dark => println!("dark"),
        }
    } else {
        println!("dark");
    }

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::ThemeChanged(theme),
                ..
            } => match theme {
                Theme::Light => println!("light"),
                Theme::Dark => println!("dark"),
            },
            _ => (),
        }
    });
}
