use image;
use minifb;

use minifb::{Key, Window, WindowOptions};

mod image_transform;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;


//fn update_picture_zoom ()

fn main() {

    let image = image::open("image.png").unwrap().to_rgb8();

    let (image_width, image_height) = image.dimensions();

    let image_buffer = image.into_raw();

    let buffer = image_transform::apply_tranform(image_buffer, image_height, image_width, HEIGHT as u32, WIDTH as u32);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut scroll_zoom = 0.0;

    while window.is_open()
    {
        window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    Key::LeftCtrl => {
                        let scroll = window.get_scroll_wheel().unwrap();

                        if scroll.0 != scroll_zoom
                        {
                            //update_picture_zoom(&buffer, scroll.0 - scroll_zoom);
                            scroll_zoom = scroll.0;
                        }

                    },
                    _ => (),
                }
            }
        });

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}