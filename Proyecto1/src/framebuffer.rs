use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, Color::BLACK);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color: Color::BLACK,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width as i32, self.height as i32, self.background_color);
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn draw_text(&mut self, text: &str, x: u32, y: u32, font_size: u32, color: Color) {
        self.set_current_color(color);
        
        //Fuente simple de 8x8 píxeles por carácter
        let char_width = font_size;
        let char_height = font_size;
        
        for (i, ch) in text.chars().enumerate() {
            let char_x = x + (i as u32 * char_width);
            self.draw_char(ch, char_x, y, char_width, char_height);
        }
    }
    
    fn draw_char(&mut self, ch: char, x: u32, y: u32, width: u32, height: u32) {
        //Patrón simple de caracteres
        let pattern = match ch {
            '0' => [
                0b01110,
                0b10001,
                0b10001,
                0b10001,
                0b10001,
                0b10001,
                0b01110,
            ],
            '1' => [
                0b00100,
                0b01100,
                0b00100,
                0b00100,
                0b00100,
                0b00100,
                0b01110,
            ],
            '2' => [
                0b01110,
                0b10001,
                0b00001,
                0b00110,
                0b01000,
                0b10000,
                0b11111,
            ],
            '3' => [
                0b01110,
                0b10001,
                0b00001,
                0b00110,
                0b00001,
                0b10001,
                0b01110,
            ],
            '4' => [
                0b10001,
                0b10001,
                0b10001,
                0b11111,
                0b00001,
                0b00001,
                0b00001,
            ],
            '5' => [
                0b11111,
                0b10000,
                0b10000,
                0b11110,
                0b00001,
                0b10001,
                0b01110,
            ],
            '6' => [
                0b01110,
                0b10001,
                0b10000,
                0b11110,
                0b10001,
                0b10001,
                0b01110,
            ],
            '7' => [
                0b11111,
                0b00001,
                0b00010,
                0b00100,
                0b01000,
                0b01000,
                0b01000,
            ],
            '8' => [
                0b01110,
                0b10001,
                0b10001,
                0b01110,
                0b10001,
                0b10001,
                0b01110,
            ],
            '9' => [
                0b01110,
                0b10001,
                0b10001,
                0b01111,
                0b00001,
                0b10001,
                0b01110,
            ],
            '.' => [
                0b00000,
                0b00000,
                0b00000,
                0b00000,
                0b00000,
                0b00000,
                0b01100,
            ],
            'A' => [
                0b01110,
                0b10001,
                0b10001,
                0b11111,
                0b10001,
                0b10001,
                0b10001,
            ],
            'B' => [
                0b11110,
                0b10001,
                0b10001,
                0b11110,
                0b10001,
                0b10001,
                0b11110,
            ],
            'C' => [
                0b01110,
                0b10001,
                0b10000,
                0b10000,
                0b10000,
                0b10001,
                0b01110,
            ],
            'D' => [
                0b11110,
                0b10001,
                0b10001,
                0b10001,
                0b10001,
                0b10001,
                0b11110,
            ],
            'E' => [
                0b11111,
                0b10000,
                0b10000,
                0b11110,
                0b10000,
                0b10000,
                0b11111,
            ],
            'F' => [
                0b11111,
                0b10000,
                0b10000,
                0b11110,
                0b10000,
                0b10000,
                0b10000,
            ],
            'G' => [
                0b01110,
                0b10001,
                0b10000,
                0b10111,
                0b10001,
                0b10001,
                0b01110,
            ],
            'M' => [
                0b10001,
                0b11011,
                0b10101,
                0b10001,
                0b10001,
                0b10001,
                0b10001,
            ],
            'N' => [
                0b10001,
                0b11001,
                0b10101,
                0b10011,
                0b10001,
                0b10001,
                0b10001,
            ],
            'O' => [
                0b01110,
                0b10001,
                0b10001,
                0b10001,
                0b10001,
                0b10001,
                0b01110,
            ],
            'P' => [
                0b11110,
                0b10001,
                0b10001,
                0b11110,
                0b10000,
                0b10000,
                0b10000,
            ],
            'R' => [
                0b11110,
                0b10001,
                0b10001,
                0b11110,
                0b10100,
                0b10010,
                0b10001,
            ],
            'S' => [
                0b01111,
                0b10000,
                0b10000,
                0b01110,
                0b00001,
                0b00001,
                0b11110,
            ],
            'T' => [
                0b11111,
                0b00100,
                0b00100,
                0b00100,
                0b00100,
                0b00100,
                0b00100,
            ],
            'a' => [
                0b00000,
                0b01110,
                0b00001,
                0b01111,
                0b10001,
                0b10001,
                0b01111,
            ],
            'c' => [
                0b00000,
                0b01110,
                0b10000,
                0b10000,
                0b10000,
                0b10001,
                0b01110,
            ],
            'd' => [
                0b00001,
                0b00001,
                0b01111,
                0b10001,
                0b10001,
                0b10001,
                0b01111,
            ],
            'e' => [
                0b00000,
                0b01110,
                0b10001,
                0b11111,
                0b10000,
                0b10001,
                0b01110,
            ],
            'g' => [
                0b00000,
                0b01111,
                0b10001,
                0b10001,
                0b01111,
                0b00001,
                0b01110,
            ],
            'h' => [
                0b10000,
                0b10000,
                0b11110,
                0b10001,
                0b10001,
                0b10001,
                0b10001,
            ],
            'i' => [
                0b00100,
                0b00000,
                0b01100,
                0b00100,
                0b00100,
                0b00100,
                0b01110,
            ],
            'm' => [
                0b00000,
                0b11010,
                0b10101,
                0b10101,
                0b10101,
                0b10001,
                0b10001,
            ],
            'n' => [
                0b00000,
                0b11110,
                0b10001,
                0b10001,
                0b10001,
                0b10001,
                0b10001,
            ],
            'o' => [
                0b00000,
                0b01110,
                0b10001,
                0b10001,
                0b10001,
                0b10001,
                0b01110,
            ],
            'p' => [
                0b00000,
                0b11110,
                0b10001,
                0b10001,
                0b11110,
                0b10000,
                0b10000,
            ],
            'r' => [
                0b00000,
                0b10110,
                0b11001,
                0b10000,
                0b10000,
                0b10000,
                0b10000,
            ],
            's' => [
                0b00000,
                0b01111,
                0b10000,
                0b01110,
                0b00001,
                0b00001,
                0b11110,
            ],
            't' => [
                0b01000,
                0b01000,
                0b11110,
                0b01000,
                0b01000,
                0b01001,
                0b00110,
            ],
            'u' => [
                0b00000,
                0b10001,
                0b10001,
                0b10001,
                0b10001,
                0b10001,
                0b01111,
            ],
            'w' => [
                0b00000,
                0b10001,
                0b10001,
                0b10001,
                0b10101,
                0b11011,
                0b10001,
            ],
            'y' => [
                0b00000,
                0b10001,
                0b10001,
                0b01010,
                0b00100,
                0b01000,
                0b10000,
            ],
            ':' => [
                0b00000,
                0b01100,
                0b01100,
                0b00000,
                0b01100,
                0b01100,
                0b00000,
            ],
            '(' => [
                0b00010,
                0b00100,
                0b01000,
                0b01000,
                0b01000,
                0b00100,
                0b00010,
            ],
            ')' => [
                0b01000,
                0b00100,
                0b00010,
                0b00010,
                0b00010,
                0b00100,
                0b01000,
            ],
            ' ' => [
                0b00000,
                0b00000,
                0b00000,
                0b00000,
                0b00000,
                0b00000,
                0b00000,
            ],
            _ => [
                0b11111,
                0b11111,
                0b11111,
                0b11111,
                0b11111,
                0b11111,
                0b11111,
            ],
        };
        
        let scale_x = width / 5;
        let scale_y = height / 7;
        
        for (row, &pattern_row) in pattern.iter().enumerate() {
            for col in 0..5 {
                if (pattern_row >> (4 - col)) & 1 == 1 {
                    let pixel_x = x + (col as u32) * scale_x;
                    let pixel_y = y + (row as u32) * scale_y;
                    
                    //Dibujar píxel escalado
                    for sx in 0..scale_x {
                        for sy in 0..scale_y {
                            let final_x = pixel_x + sx;
                            let final_y = pixel_y + sy;
                            if final_x < self.width && final_y < self.height {
                                self.set_pixel(final_x, final_y);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn _render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }

    pub fn swap_buffers(
        &self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
    ) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }
}