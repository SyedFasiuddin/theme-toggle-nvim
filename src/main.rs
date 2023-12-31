use std::io::BufRead;

use winit::event::Event;
use winit::event::WindowEvent;

#[cfg(not(target_os = "macos"))]
use winit::event_loop::EventLoop;

#[cfg(target_os = "macos")]
use winit::event_loop::EventLoopBuilder;
#[cfg(target_os = "macos")]
use winit::platform::macos::ActivationPolicy;
#[cfg(target_os = "macos")]
use winit::platform::macos::EventLoopBuilderExtMacOS;

use winit::window::Theme;
use winit::window::WindowBuilder;

fn handle_quit() {
    std::thread::spawn(|| {
        for line in std::io::stdin().lock().lines() {
            if line.unwrap().trim() == "quit" {
                std::process::exit(0);
            }
        }
    });
}

fn main() {
    #[cfg(target_os = "macos")]
    let event_loop = EventLoopBuilder::new()
        .with_activation_policy(ActivationPolicy::Prohibited)
        .build();

    #[cfg(not(target_os = "macos"))]
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    handle_quit();

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
