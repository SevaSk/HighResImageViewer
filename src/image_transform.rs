
pub struct Transform
{
    pub image_buffer : Vec<u8>, //3 elements represent 1 pixel with RGB
    pub y_1 :f32, //image height
    pub x_1 :f32, //image width
    pub y_2 :f32, //output buffer height
    pub x_2 :f32,  //output buffer width
    pub scale : f32
}


impl Transform
{
pub fn apply_tranform(&self) -> Vec<u32>
{
        let mut output_buffer = vec![0; (self.x_2*self.y_2) as usize];

        //TODO why does scaling dupicate image

        let x_1 = self.x_1*self.scale;
        let y_1 = self.y_1*self.scale;

        //TODO why does that have to be whole numeber?
        let slope = ((y_1/self.y_2) as u32)  as f32;
        let b = x_1 - slope*self.x_2;

        for h in 0..self.y_2 as u32
        {
            for w in 0..self.x_2 as u32
            {
                let f_h = h as f32;
                let f_w = w as f32;

                let x_transformed = f_w*slope + b/2.0;

                if x_transformed >= 0.0 && x_transformed <= x_1
                {
                    let y_transformed = f_h*slope;
                    let image_index = std::cmp::min ((self.x_1*self.y_1) as usize *3 -3, (x_transformed + self.x_1*y_transformed) as usize *3);

                    let zrgb = self.get_u32_x0rgb_from_image (image_index);
                    output_buffer[(w + (self.x_2 as u32)*h) as usize] = zrgb;
                }
            }
        }
        return output_buffer;
    }

    fn get_u32_x0rgb_from_image (&self, index : usize) -> u32
    {
        ((self.image_buffer[index] as u32) << 16) | 
        ((self. image_buffer[index + 1] as u32) << 8) | ((self.image_buffer[index +2] as u32) << 0)
    }

    pub fn scroll_to_scale (&mut self, scroll : f32)
    {
        if scroll > 0.0
        {
        self.scale = self.scale + 0.1;
        }
        else 
        {
        self.scale = self.scale - 0.1;
        }

    }
}
