pub struct Screen {
    palette: [u32; 5],
    buffer: [u8; 256],
    f_buffer: Vec<u32>,
}

impl Screen {
    pub fn new(scale: usize) -> Self {
        Screen {
            palette: [0x00_00_00, 0xFF_FF_FF, 0xFF_00_00, 0x00_FF_00, 0x00_00_FF],
            buffer: [0x00; 256],
            f_buffer: vec![0x00; 32*scale*16*scale]
        }
    }

    pub fn update(&mut self, mmio: &[u8]) {
        for i in 0..255 {
            self.buffer[i] = mmio[i]
        }
    }

    pub fn convert(&mut self) {
        let mut f_index = 0;
        for i in 0..256 {
            let hi: u8 = (self.buffer[i] & 0b1111_0000 >> 4) as u8;
            let lo: u8 = (self.buffer[i] & 0b0000_1111) as u8;

            self.f_buffer[f_index] = self.palette[lo as usize];
            self.f_buffer[f_index as usize + 1] = self.palette[hi as usize];

            f_index += 1;
        }
    }

    pub fn scale(&mut self, scale: usize) {
        for y in 0..16 {

            for x in 0..32 {

                let color = self.f_buffer[y*32 + x];

                let corner_x = x*scale;
                let corner_y = y*scale;

                for row in 0..scale {
                    for col in 0..scale {
                        let px = corner_x + col;
                        let py = corner_y + row;
                        let index = (py*(32*scale))+px;

                        if index < (16*scale)*(32*scale) {
                            self.f_buffer[index] = color;
                        }
                    }
                }
            }
        }
    }

    pub fn push(&mut self, window: &mut minifb::Window, scale: usize) {
        window.update_with_buffer(&self.f_buffer, 32*scale, 16*scale).expect("couldn't draw to window")
    }
}
