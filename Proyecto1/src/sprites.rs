use raylib::prelude::*;
use std::collections::HashMap;
use crate::maze::Maze;
use crate::player::Player;
use crate::framebuffer::Framebuffer;
use image;
use std::fs::File;
use std::io::BufReader;

#[derive(Clone)]
pub struct AnimatedSprite {
    pub position: Vector2,
    pub sprite_type: SpriteType,
    pub current_frame: usize,
    pub frame_timer: f32,
    pub scale: f32,
    pub visible: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpriteType {
    Naruto,
}

//Estructura para almacenar datos de píxeles de una textura
#[derive(Clone)]
pub struct SpritePixelData {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

pub struct SpriteManager {
    pub sprites: Vec<AnimatedSprite>,
    sprite_pixel_data: HashMap<SpriteType, Vec<SpritePixelData>>,
    frame_duration: f32,
}

impl SpriteManager {
    pub fn new() -> Self {
        SpriteManager {
            sprites: Vec::new(),
            sprite_pixel_data: HashMap::new(),
            frame_duration: 0.08, 
        }
    }

    pub fn load_sprite_textures(&mut self, _rl: &mut RaylibHandle, _thread: &RaylibThread) {
        self.load_gif_frames(SpriteType::Naruto, "assets/img/naruto.gif");
    }
    
    fn load_gif_frames(&mut self, sprite_type: SpriteType, path: &str) {
        println!("🎬 Cargando frames REALES de animación de {}", path);
        
        //Intentar cargar el GIF usando la biblioteca gif
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                match gif::DecodeOptions::new().read_info(reader) {
                    Ok(mut decoder) => {
                        println!("GIF decoder creado exitosamente");
                        
                        //Obtener la paleta global si existe
                        let global_palette = decoder.global_palette().map(|p| p.to_vec());
                        
                        let mut pixel_frames = Vec::new();
                        let mut frame_count = 0;
                        
                        //Decodificar todos los frames del GIF
                        loop {
                            match decoder.read_next_frame() {
                                Ok(Some(frame)) => {
                                    frame_count += 1;
                                    println!("Procesando frame {}: {}x{} píxeles", 
                                        frame_count, frame.width, frame.height);
                                    
                                    //Convertir el frame a nuestro formato usando la paleta
                                    let pixel_data = self.convert_gif_frame_with_palette(frame, &global_palette);
                                    pixel_frames.push(pixel_data);
                                    
                                    //Limitar a 8 frames para performance
                                    if frame_count >= 8 {
                                        break;
                                    }
                                },
                                Ok(None) => {
                                    //No hay más frames
                                    break;
                                },
                                Err(e) => {
                                    println!("Error decodificando frame {}: {:?}", frame_count + 1, e);
                                    break;
                                }
                            }
                        }
                        
                        if pixel_frames.is_empty() {
                            println!("No se pudieron cargar frames del GIF");
                        } else {
                            self.sprite_pixel_data.insert(sprite_type, pixel_frames);
                        }
                    },
                    Err(e) => {
                        println!("Error creando decoder GIF: {:?}", e);
                    }
                }
            },
            Err(e) => {
                println!("Error abriendo archivo GIF: {:?}", e);
            }
        }
    }
    
    fn convert_gif_frame_with_palette(&self, frame: &gif::Frame, global_palette: &Option<Vec<u8>>) -> SpritePixelData {
        let width = frame.width as u32;
        let height = frame.height as u32;
        let mut pixels = Vec::with_capacity((width * height) as usize);
        
        let buffer = frame.buffer.as_ref();
        
        //Obtener la paleta (local del frame o global)
        let palette = frame.palette.as_ref().or(global_palette.as_ref());
        
        if let Some(palette) = palette {
            println!("Usando paleta de {} colores", palette.len() / 3);
            
            //Obtener el índice de color transparente si existe
            let transparent_index = frame.transparent;
            
            for &color_index in buffer.iter() {
                if let Some(transparent_idx) = transparent_index {
                    if color_index == transparent_idx {
                        // Píxel transparente
                        pixels.push(Color::new(0, 0, 0, 0));
                        continue;
                    }
                }
                
                //Obtener color de la paleta
                let palette_index = (color_index as usize) * 3;
                if palette_index + 2 < palette.len() {
                    let r = palette[palette_index];
                    let g = palette[palette_index + 1];
                    let b = palette[palette_index + 2];
                    pixels.push(Color::new(r, g, b, 255));
                } else {
                    pixels.push(Color::new(255, 0, 255, 255)); 
                }
            }
        } else {
            println!("No hay paleta disponible, usando escala de grises");
            for &index in buffer.iter() {
                pixels.push(Color::new(index, index, index, 255));
            }
        }
        
        //Asegurar que hay un número correcto de píxeles
        while pixels.len() < (width * height) as usize {
            pixels.push(Color::new(0, 0, 0, 0)); //Transparente para píxeles faltantes
        }
        
        pixels.truncate((width * height) as usize);
        
        let opaque_pixels = pixels.iter().filter(|p| p.a > 0).count();
        println!("Frame convertido: {}x{} = {} píxeles ({} opacos, {} transparentes)", 
            width, height, pixels.len(), opaque_pixels, pixels.len() - opaque_pixels);
        
        SpritePixelData {
            width,
            height,
            pixels,
        }
    }
    
    
    
    pub fn spawn_sprites_in_maze(&mut self, maze: &Maze, block_size: usize) {
        self.sprites.clear();
        
        let target_sprites = 8;
        let maze_width = if !maze.is_empty() { maze[0].len() } else { 0 };
        let maze_height = maze.len();
        
        if maze_width == 0 || maze_height == 0 {
            println!("ERROR: Laberinto inválido para spawning");
            return;
        }
        
        let mut valid_positions = Vec::new();
        
        for y in 1..(maze_height - 1) { 
            for x in 1..(maze_width - 1) { 
                if maze[y][x] == ' ' {
                    let neighbors_clear = [
                        y > 0 && maze[y-1][x] == ' ',
                        y < maze_height - 1 && maze[y+1][x] == ' ',
                        x > 0 && maze[y][x-1] == ' ',
                        x < maze_width - 1 && maze[y][x+1] == ' ',
                    ].iter().filter(|&&clear| clear).count();
                    
                    if neighbors_clear >= 3 {
                        let mut near_goal = false;
                        for dy in -1i32..=1i32 {
                            for dx in -1i32..=1i32 {
                                let check_y = (y as i32 + dy) as usize;
                                let check_x = (x as i32 + dx) as usize;
                                
                                if check_y < maze_height && check_x < maze_width && 
                                   maze[check_y][check_x] == 'g' {
                                    near_goal = true;
                                    break;
                                }
                            }
                            if near_goal { break; }
                        }
                        
                        if !near_goal {
                            valid_positions.push((x, y));
                        }
                    }
                }
            }
        }
        
        println!("Encontradas {} posiciones válidas para spawning", valid_positions.len());
        
        if valid_positions.is_empty() {
            println!("ERROR: No hay posiciones válidas para spawning");
            return;
        }
        
        //Mezclar posiciones para distribución aleatoria
        for i in 0..valid_positions.len() {
            let j = fastrand::usize(i..valid_positions.len());
            valid_positions.swap(i, j);
        }
        
        //Spawning con separación mínima
        let mut spawned = 0;
        let min_distance_squared = (block_size * 3) * (block_size * 3); //Distancia mínima entre sprites
        
        for &(x, y) in &valid_positions {
            if spawned >= target_sprites {
                break;
            }
            
            let new_pos = Vector2::new(
                x as f32 * block_size as f32 + block_size as f32 / 2.0,
                y as f32 * block_size as f32 + block_size as f32 / 2.0,
            );
            
            //Verificar que no esté muy cerca de sprites existentes
            let too_close = self.sprites.iter().any(|sprite| {
                let dx = sprite.position.x - new_pos.x;
                let dy = sprite.position.y - new_pos.y;
                (dx * dx + dy * dy) < min_distance_squared as f32
            });
            
            if !too_close {
                let sprite = AnimatedSprite {
                    position: new_pos,
                    sprite_type: SpriteType::Naruto, 
                    current_frame: fastrand::usize(0..4), //Frame inicial aleatorio
                    frame_timer: fastrand::f32() * self.frame_duration, //Timer inicial aleatorio
                    scale: 1.0,
                    visible: true,
                };
                
                self.sprites.push(sprite);
                spawned += 1;
                
                println!("Sprite {} spawneado en ({}, {}) - Posición mundo: ({:.1}, {:.1})", 
                    spawned, x, y, new_pos.x, new_pos.y);
            }
        }
        
        //Si no hay suficientes sprites, ser menos estricto con la distancia
        if spawned < target_sprites {
            println!("Solo {} sprites spawneados, intentando completar con distancia menor...", spawned);
            let reduced_distance_squared = (block_size * 2) * (block_size * 2);
            
            for &(x, y) in &valid_positions {
                if spawned >= target_sprites {
                    break;
                }
                
                let new_pos = Vector2::new(
                    x as f32 * block_size as f32 + block_size as f32 / 2.0,
                    y as f32 * block_size as f32 + block_size as f32 / 2.0,
                );
                
                let too_close = self.sprites.iter().any(|sprite| {
                    let dx = sprite.position.x - new_pos.x;
                    let dy = sprite.position.y - new_pos.y;
                    (dx * dx + dy * dy) < reduced_distance_squared as f32
                });
                
                if !too_close {
                    let sprite = AnimatedSprite {
                        position: new_pos,
                        sprite_type: SpriteType::Naruto,
                        current_frame: fastrand::usize(0..4),
                        frame_timer: fastrand::f32() * self.frame_duration,
                        scale: 1.0,
                        visible: true,
                    };
                    
                    self.sprites.push(sprite);
                    spawned += 1;
                    
                    println!("Sprite {} spawneado (distancia reducida) en ({}, {})", spawned, x, y);
                }
            }
        }
        
        println!("Spawning completado: {} sprites de Naruto en el laberinto", spawned);
        
        if spawned < target_sprites {
            println!("Advertencia: Solo se pudieron spawner {} de {} sprites objetivo", spawned, target_sprites);
        }
    }

    pub fn update(&mut self, dt: f32) {
        for sprite in &mut self.sprites {
            sprite.frame_timer += dt;
            if sprite.frame_timer >= self.frame_duration {
                sprite.frame_timer = 0.0;
                
                //Obtener el número de frames disponibles para este sprite
                let num_frames = if let Some(pixel_frames) = self.sprite_pixel_data.get(&sprite.sprite_type) {
                    pixel_frames.len().max(1)
                } else {
                    1
                };
                
                sprite.current_frame = (sprite.current_frame + 1) % num_frames;
            }
        }
    }

    fn has_wall_between(&self, from: Vector2, to: Vector2, maze: &Maze, block_size: usize) -> bool {
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let steps = (distance / 5.0).ceil() as i32;
        
        if steps == 0 {
            return false;
        }
        
        let step_x = dx / steps as f32;
        let step_y = dy / steps as f32;
        
        let maze_width = if !maze.is_empty() { maze[0].len() } else { 0 };
        let maze_height = maze.len();
        
        for i in 1..steps {
            let check_x = from.x + step_x * i as f32;
            let check_y = from.y + step_y * i as f32;
            
            let maze_x = (check_x / block_size as f32) as usize;
            let maze_y = (check_y / block_size as f32) as usize;
            
            if maze_x < maze_width && maze_y < maze_height {
                if maze[maze_y][maze_x] != ' ' {
                    return true;
                }
            }
        }
        
        false
    }

    pub fn render_sprites_3d(&self, framebuffer: &mut Framebuffer, player: &Player, maze: &Maze, block_size: usize) {
        for sprite in &self.sprites {
            if !sprite.visible {
                continue;
            }
            
            let dx = sprite.position.x - player.pos.x;
            let dy = sprite.position.y - player.pos.y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance > 300.0 {
                continue;
            }
            
            if self.has_wall_between(player.pos, sprite.position, maze, block_size) {
                continue;
            }
            
            let sprite_angle = dy.atan2(dx);
            let mut angle_diff = sprite_angle - player.a;
            
            while angle_diff > std::f32::consts::PI {
                angle_diff -= 2.0 * std::f32::consts::PI;
            }
            while angle_diff < -std::f32::consts::PI {
                angle_diff += 2.0 * std::f32::consts::PI;
            }
            
            if angle_diff.abs() > player.fov / 2.0 {
                continue;
            }
            
            let screen_x = ((angle_diff / player.fov) + 0.5) * framebuffer.width as f32;
            let sprite_height = (framebuffer.height as f32 / distance) * 50.0 * sprite.scale;
            let sprite_width = sprite_height;
            
            if screen_x + sprite_width / 2.0 > 0.0 && screen_x - sprite_width / 2.0 < framebuffer.width as f32 {
                //Usar los datos de píxeles reales para el renderizado 3D 
                if let Some(pixel_frames) = self.sprite_pixel_data.get(&sprite.sprite_type) {
                    if !pixel_frames.is_empty() {
                        //Usar el frame actual de la animación
                        let frame_index = sprite.current_frame.min(pixel_frames.len() - 1);
                        let pixel_data = &pixel_frames[frame_index];
                        
                        let start_x = (screen_x - sprite_width / 2.0).max(0.0) as u32;
                        let end_x = (screen_x + sprite_width / 2.0).min(framebuffer.width as f32) as u32;
                        let start_y = ((framebuffer.height as f32 / 2.0) - sprite_height / 2.0).max(0.0) as u32;
                        let end_y = ((framebuffer.height as f32 / 2.0) + sprite_height / 2.0).min(framebuffer.height as f32) as u32;
                        
                        //Renderizar usando los píxeles reales del sprite
                        for screen_y in start_y..end_y {
                            for screen_x_pos in start_x..end_x {
                                if screen_x_pos < framebuffer.width && screen_y < framebuffer.height {
                                    //Mapear coordenadas de pantalla a coordenadas de textura
                                    let tex_x = ((screen_x_pos - start_x) * pixel_data.width / (end_x - start_x).max(1)) as usize;
                                    let tex_y = ((screen_y - start_y) * pixel_data.height / (end_y - start_y).max(1)) as usize;
                                    
                                    if tex_x < pixel_data.width as usize && tex_y < pixel_data.height as usize {
                                        let pixel_index = tex_y * pixel_data.width as usize + tex_x;
                                        if pixel_index < pixel_data.pixels.len() {
                                            let pixel_color = pixel_data.pixels[pixel_index];
                                            
                                            //Solo dibujar píxeles no transparentes
                                            if pixel_color.a > 128 { 
                                                framebuffer.set_current_color(pixel_color);
                                                framebuffer.set_pixel(screen_x_pos, screen_y);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        //Fallback: usar color sólido si no hay datos de píxeles
                        self.render_3d_fallback(framebuffer, sprite, screen_x, sprite_width, sprite_height);
                    }
                } else {
                    //Fallback si no hay sprite data
                    self.render_3d_fallback(framebuffer, sprite, screen_x, sprite_width, sprite_height);
                }
            }
        }
    }
    
    fn render_3d_fallback(&self, framebuffer: &mut Framebuffer, _sprite: &AnimatedSprite, screen_x: f32, sprite_width: f32, sprite_height: f32) {
        let color = Color::new(255, 165, 0, 255); 
        
        framebuffer.set_current_color(color);
        
        let start_x = (screen_x - sprite_width / 2.0).max(0.0) as u32;
        let end_x = (screen_x + sprite_width / 2.0).min(framebuffer.width as f32) as u32;
        let start_y = ((framebuffer.height as f32 / 2.0) - sprite_height / 2.0).max(0.0) as u32;
        let end_y = ((framebuffer.height as f32 / 2.0) + sprite_height / 2.0).min(framebuffer.height as f32) as u32;
        
        for x in start_x..end_x {
            for y in start_y..end_y {
                if x < framebuffer.width && y < framebuffer.height {
                    framebuffer.set_pixel(x, y);
                }
            }
        }
    }
}