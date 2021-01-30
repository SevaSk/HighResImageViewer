
pub fn apply_tranform(image : Vec<u8>, y_1 : u32, x_1 : u32, y_2 : u32, x_2 : u32) -> Vec<u32>
{
    let mut output_buffer = vec![0; (x_2*y_2) as usize];

    for h in 0..y_2
    {
        for w in 0..x_2
        {
            let x_transform = w*x_1/x_2;

            let image_index = 3*(x_transform + h*y_1/y_2*x_1) as usize;

            let zrgb = ((image[image_index] as u32) << 16) | 
                    ((image[image_index + 1] as u32) << 8) | ((image[image_index +2] as u32) << 0);

            output_buffer[(w + x_2*h) as usize] = zrgb;
        }
        
    }
    return output_buffer;
}
