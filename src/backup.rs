extern crate sdl2;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use std::time::Duration;
use sdl2::pixels::Color;



fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("SDL2", 640, 480)
        .position_centered().build().unwrap();

    let mut canvas = window.into_canvas()
        .accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));

    let mut timer = sdl_context.timer().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // animation sheet and extras are available from
    // https://opengameart.org/content/a-platformer-in-the-forest
    let mut temp_surface = 
sdl2::surface::Surface::load_bmp(Path::new("assets/tileSet.bmp")).unwrap();
    temp_surface.set_color_key(true,Color::RGB(255,0,255));
    let texture = 
texture_creator.create_texture_from_surface(&temp_surface).unwrap();

    let frames_per_anim = 4;
    let sprite_tile_size = (256,32);

    // Baby - walk animation
    let mut source_rect_0 = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_0 = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.0);

    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), 
..} => {
                    running = false;
                },
                _ => {}
            }
        }

        let ticks = timer.ticks() as i32;

        // set the current frame for time
//        source_rect_0.set_x(32 * ((ticks / 100) % frames_per_anim));
        dest_rect_0.set_x(1 * ((ticks / 14) % 768) - 128);

        canvas.clear();
        // copy the frame to the canvas
        canvas.copy_ex(&texture, Some(source_rect_0), Some(dest_rect_0), 0.0, None, false, false).unwrap();
       canvas.present();

        std::thread::sleep(Duration::from_millis(100));
    }
}

