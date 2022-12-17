use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas};

fn main() -> Result<(), String>{
    println!("Hello, world!");

    let (mut event_pump, mut canvas) = show_window()?;

    loop {
        let quit = handle_events(&mut event_pump);
        if quit {
            break
        }

        // canvas.copy(&texture, None, None).unwrap();
        canvas.present();
    }

    println!("Successfully finished!");
    Ok(())
}

fn handle_events(event_pump: &mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
               return true;
            },
            _ => {},
        }
    }
    return false;
}

fn show_window() -> Result<(EventPump, WindowCanvas), String> {
    let sdl_context = sdl2::init()?;
    let window = match sdl_context.video()?.window("", 1280, 720).position_centered().build() {
        Ok(w) => w,
        Err(e) => return Err(e.to_string()),
    };
    let canvas = match window.into_canvas().build() {
        Ok(c) => c,
        Err(e) => return Err(e.to_string()),
    };
    let event_pump = sdl_context.event_pump()?;

    Ok((event_pump, canvas))
}