use raylib::color::Color;
use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub tx: f32, 
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool,
) -> Intersect {
    let mut d = 0.0;
    let step_size = 1.0;
    let max_distance = 1000.0;
    
    framebuffer.set_current_color(Color::WHITESMOKE);
    
    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;
        
        let i = x / block_size;
        let j = y / block_size;
        
        //Verificar límites del laberinto
        if j >= maze.len() || i >= maze[0].len() || d > max_distance {
            return Intersect {
                distance: d,
                impact: '+', //Pared por defecto
                tx: 0.0,
            };
        }
        
        //Verificar si golpeamos una pared
        if maze[j][i] != ' ' {
            //Cálculo simplificado de coordenada de textura
            let hit_x = player.pos.x + cos;
            let hit_y = player.pos.y + sin;
            
            //Usar una coordenada de textura simple y estable
            let tx = ((hit_x + hit_y) / block_size as f32).fract();
            
            return Intersect {
                distance: d,
                impact: maze[j][i],
                tx: tx.abs(), // Asegurar que sea positivo
            };
        }
        
        //Dibujar línea si es necesario
        if draw_line {
            if x < framebuffer.width as usize && y < framebuffer.height as usize {
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
        
        d += step_size;
    }
}