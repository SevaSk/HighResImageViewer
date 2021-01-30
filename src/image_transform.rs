
pub struct Transform
{
    pub image_buffer : Vec<u8>, //3 elements represent 1 pixel with RGB
    pub y_1 :u32, //image height
    pub x_1 :u32, //image width
    pub y_2 :u32, //output buffer height
    pub x_2 :u32  //otput buffer width
}


impl Transform
{
pub fn apply_tranform(&self) -> Vec<u32>
{
        let mut output_buffer = vec![0; (self.x_2*self.y_2) as usize];

        let slope = self.y_1/self.y_2;

        for h in 0..self.y_2
        {
            for w in 0..self.x_2
            {
                let b = self.x_1 as i64 - (slope*self.x_2) as i64;
                let x_transformed_int = (w*slope) as i64 + b/2;

                if x_transformed_int >= 0 && x_transformed_int <= self.x_1 as i64
                {
                    let x_transformed = x_transformed_int as u32;
                    let y_transformed = h*slope;
                    let image_index = 3*(x_transformed + self.x_1*y_transformed) as usize;

                    let zrgb = self.get_u32_x0rgb_from_image (image_index);
                    output_buffer[(w + self.x_2*h) as usize] = zrgb;
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
}
