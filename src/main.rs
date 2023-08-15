use std::io::BufRead;

use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::EventLoopBuilder;
use winit::window::Theme;
use winit::window::WindowBuilder;
use winit::platform::macos::*;

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
    let event_loop = EventLoopBuilder::new()
        .with_activation_policy(ActivationPolicy::Prohibited)
        .build();
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
