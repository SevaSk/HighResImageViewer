extern crate sdl2;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::render::{WindowCanvas, Texture};
use sdl2::pixels::Color;


fn render (canvas : &mut WindowCanvas, color : Color, texture: &Texture) -> Result<(), String>
{
    canvas.set_draw_color(color);

    canvas.clear();

    canvas.copy(texture, None, None).unwrap();

    canvas.present();

    Ok (())
}

fn main() {

    let sdl = sdl2::init().unwrap();

    let video_subsystem = sdl.video().unwrap();

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);

    let window = video_subsystem
        .window("Image viewer", 900, 700)
        .resizable()
        .build().unwrap();

    let mut canvas = window.into_canvas().build()
    .expect("could not make a canvas");


    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("image.png").unwrap();

    let mut i = 0;

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event 
            {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        i = (i + 1) % 255;

        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture).unwrap();
    }
}
