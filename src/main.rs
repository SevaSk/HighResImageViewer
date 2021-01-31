use image;
use minifb;

use minifb::{Key, Window, WindowOptions};

mod image_transform;

const WIDTH: usize = 777;
const HEIGHT: usize = 777;


//fn update_picture_zoom ()

fn main() {

    let image = image::open("image.png").unwrap().to_rgb8();

    let (image_width, image_height) = image.dimensions();

    let image_buffer = image.into_raw();

    let mut transform = image_transform::Transform {
        image_buffer, y_1 : image_height as f32,
        x_1 : image_width as f32,
        y_2 : HEIGHT as f32,
        x_2 : WIDTH as f32, 
        scale : 1.0};

    let mut buffer = transform.apply_tranform();

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

    while window.is_open()
    {
        window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    Key::LeftCtrl => {

                        window.get_scroll_wheel().map (|scroll|
                        {
                            transform.scroll_to_scale(scroll.1);
                            buffer = transform.apply_tranform();

                            println!("scrolling {}", transform.scale);
                        });
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