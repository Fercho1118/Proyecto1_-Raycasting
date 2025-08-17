#![allow(unused_imports)]
#![allow(dead_code)]

mod line;
mod framebuffer;
mod maze;
mod caster;
mod player;

use line::line;
use maze::{Maze, load_maze};
use caster::{cast_ray, Intersect};
use framebuffer::Framebuffer;
use player::{Player, process_events, get_gamepad_info, check_gamepad_mode_change, check_victory};
use raylib::prelude::*;
use std::thread;
use std::time::{Duration, Instant};
use std::f32::consts::PI;

fn cell_to_color(cell: char) -> Color {
    match cell {
        '+' => Color::BLUE,
        '-' => Color::BLUE,
        '|' => Color::BLUE,
        'g' => Color::GREEN,
        ' ' => Color::BLACK,
        _ => Color::WHITE,
    }
}

fn draw_cell(
    framebuffer: &mut Framebuffer,
    xo: usize,
    yo: usize,
    block_size: usize,
    cell: char,
) {
    let color = cell_to_color(cell);
    framebuffer.set_current_color(color);
    
    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            if x < framebuffer.width as usize && y < framebuffer.height as usize {
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }
}

fn draw_minimap(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
) {
    //Configuración del minimapa
    let minimap_scale = 12; //Cada celda del maze será de 12x12 píxeles en el minimapa
    let maze_width = if maze.len() > 0 { maze[0].len() } else { 0 };
    let maze_height = maze.len();
    let minimap_width = maze_width * minimap_scale;
    let minimap_height = maze_height * minimap_scale;
    let minimap_x = framebuffer.width as usize - minimap_width - 10; //10 píxeles del borde derecho
    let minimap_y = 10; //10 píxeles del borde superior
    
    //Fondo del minimapa
    framebuffer.set_current_color(Color::new(0, 0, 0, 150));
    for x in minimap_x..minimap_x + minimap_width {
        for y in minimap_y..minimap_y + minimap_height {
            if x < framebuffer.width as usize && y < framebuffer.height as usize {
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }
    
    //Dibujar el maze en el minimapa
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let cell_x = minimap_x + col_index * minimap_scale;
            let cell_y = minimap_y + row_index * minimap_scale;
            
            let color = match cell {
                '+' | '-' | '|' => Color::WHITE,
                'g' => Color::GREEN,
                ' ' => Color::BLACK,
                _ => Color::GRAY,
            };
            
            if cell != ' ' {
                framebuffer.set_current_color(color);
                for x in 0..minimap_scale {
                    for y in 0..minimap_scale {
                        let pixel_x = cell_x + x;
                        let pixel_y = cell_y + y;
                        if pixel_x < framebuffer.width as usize && pixel_y < framebuffer.height as usize {
                            framebuffer.set_pixel(pixel_x as u32, pixel_y as u32);
                        }
                    }
                }
            }
        }
    }
    
    //Dibujar la posición del jugador
    let player_minimap_x = minimap_x + ((player.pos.x as usize) / block_size) * minimap_scale + minimap_scale / 2;
    let player_minimap_y = minimap_y + ((player.pos.y as usize) / block_size) * minimap_scale + minimap_scale / 2;
    
    //Jugador como círculo rojo
    framebuffer.set_current_color(Color::RED);
    let radius = 3;
    for dx in -radius..=radius {
        for dy in -radius..=radius {
            if dx * dx + dy * dy <= radius * radius {
                let x = player_minimap_x as i32 + dx;
                let y = player_minimap_y as i32 + dy;
                if x >= 0 && y >= 0 && x < framebuffer.width as i32 && y < framebuffer.height as i32 {
                    framebuffer.set_pixel(x as u32, y as u32);
                }
            }
        }
    }
    
    //Dibujar la dirección del jugador
    framebuffer.set_current_color(Color::YELLOW);
    let direction_length = 15.0;
    let end_x = player_minimap_x as f32 + direction_length * player.a.cos();
    let end_y = player_minimap_y as f32 + direction_length * player.a.sin();
    
    line(
        framebuffer,
        Vector2::new(player_minimap_x as f32, player_minimap_y as f32),
        Vector2::new(end_x, end_y),
    );
    
    //Borde del minimapa
    framebuffer.set_current_color(Color::WHITE);
    //Borde superior e inferior
    for x in minimap_x..minimap_x + minimap_width {
        framebuffer.set_pixel(x as u32, minimap_y as u32);
        framebuffer.set_pixel(x as u32, (minimap_y + minimap_height - 1) as u32);
    }
    //Bordes laterales
    for y in minimap_y..minimap_y + minimap_height {
        framebuffer.set_pixel(minimap_x as u32, y as u32);
        framebuffer.set_pixel((minimap_x + minimap_width - 1) as u32, y as u32);
    }
}

pub fn render_maze(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
) {
    //Dibujar el laberinto
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }
    
    //Dibujar al jugador como un círculo rojo
    framebuffer.set_current_color(Color::RED);
    let player_x = player.pos.x as i32;
    let player_y = player.pos.y as i32;
    let radius = 8;
    
    for dx in -radius..=radius {
        for dy in -radius..=radius {
            if dx * dx + dy * dy <= radius * radius {
                let x = player_x + dx;
                let y = player_y + dy;
                if x >= 0 && y >= 0 && x < framebuffer.width as i32 && y < framebuffer.height as i32 {
                    framebuffer.set_pixel(x as u32, y as u32);
                }
            }
        }
    }
    
    //Dibujar la dirección del jugador
    framebuffer.set_current_color(Color::YELLOW);
    let direction_length = 30.0;
    let end_x = player.pos.x + direction_length * player.a.cos();
    let end_y = player.pos.y + direction_length * player.a.sin();
    
    line(
        framebuffer,
        Vector2::new(player.pos.x, player.pos.y),
        Vector2::new(end_x, end_y),
    );
    
    //Dibujar algunos rayos para visualizar el FOV
    framebuffer.set_current_color(Color::WHITESMOKE);
    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / (num_rays - 1) as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, &player, a, block_size, true);
    }
}

fn render_world(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
) {
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32 / 2.0;
    
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);
        
        let distance_to_wall = intersect.distance.max(1.0); 
        let distance_to_projection_plane = 70.0;
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;
        
        let stake_top = ((hh - (stake_height / 2.0)) as usize).max(0);
        let stake_bottom = ((hh + (stake_height / 2.0)) as usize).min(framebuffer.height as usize);
        
        //Renderizar piso con color sólido
        for y in stake_bottom..framebuffer.height as usize {
            if i < framebuffer.width && y < framebuffer.height as usize {
                //Color sólido para el piso
                let floor_color = Color::new(192, 201, 135, 255);
                framebuffer.set_current_color(floor_color);
                framebuffer.set_pixel(i, y as u32);
            }
        }
        
        //Renderizar la columna vertical con validaciones
        if stake_top < stake_bottom && stake_bottom > 0 {
            for y in stake_top..stake_bottom {
                if i < framebuffer.width && y < framebuffer.height as usize {
                    //Calcular la coordenada Y de la textura de forma segura
                    let wall_height = stake_bottom - stake_top;
                    let ty = if wall_height > 0 {
                        ((y - stake_top) as f32 / wall_height as f32).clamp(0.0, 1.0)
                    } else {
                        0.0
                    };
                    
                    //Obtener el color con validaciones
                    let color = match intersect.impact {
                        '+' | '-' | '|' => {
                            //Usar textura para paredes azules con coordenadas válidas
                            let safe_tx = intersect.tx.clamp(0.0, 1.0);
                            framebuffer.get_texture_pixel(safe_tx, ty)
                        },
                        _ => {
                            //Usar color sólido para otros tipos de celdas
                            cell_to_color(intersect.impact)
                        }
                    };
                    
                    framebuffer.set_current_color(color);
                    framebuffer.set_pixel(i, y as u32);
                }
            }
        }
    }
}

fn draw_victory_screen(framebuffer: &mut Framebuffer) {
    //Dibujar mensaje de éxito centrado
    let center_x = framebuffer.width / 2;
    let center_y = framebuffer.height / 2;
    
    framebuffer.set_current_color(Color::new(0, 0, 0, 180)); 
    
    //Dibujar rectángulo de fondo para el texto
    let rect_width = 600;
    let rect_height = 200;
    let rect_x = center_x.saturating_sub(rect_width / 2);
    let rect_y = center_y.saturating_sub(rect_height / 2);
    
    for x in rect_x..rect_x + rect_width {
        for y in rect_y..rect_y + rect_height {
            if x < framebuffer.width && y < framebuffer.height {
                framebuffer.set_pixel(x, y);
            }
        }
    }
    
    //"SUCCESS!" 
    let success_text = "SUCCESS!";
    let char_width = 32;
    let text_width = success_text.len() as u32 * char_width;
    let start_x = center_x.saturating_sub(text_width / 2);
    
    //Dibujar borde negro del texto
    framebuffer.draw_text(success_text, start_x + 2, center_y - 58, char_width, Color::BLACK);
    framebuffer.draw_text(success_text, start_x - 2, center_y - 58, char_width, Color::BLACK);
    framebuffer.draw_text(success_text, start_x, center_y - 58, char_width, Color::BLACK);
    framebuffer.draw_text(success_text, start_x, center_y - 62, char_width, Color::BLACK);
    
    //Texto principal en blanco
    framebuffer.draw_text(success_text, start_x, center_y - 60, char_width, Color::WHITE);
    
    //"You reached the goal!" - texto secundario con borde
    let goal_text = "You reached the goal!";
    let char_width_small = 20;
    let text_width_small = goal_text.len() as u32 * char_width_small;
    let start_x_small = center_x.saturating_sub(text_width_small / 2);
    
    //Borde negro
    framebuffer.draw_text(goal_text, start_x_small + 1, center_y + 9, char_width_small, Color::BLACK);
    framebuffer.draw_text(goal_text, start_x_small - 1, center_y + 9, char_width_small, Color::BLACK);
    framebuffer.draw_text(goal_text, start_x_small, center_y + 9, char_width_small, Color::BLACK);
    framebuffer.draw_text(goal_text, start_x_small, center_y + 11, char_width_small, Color::BLACK);
    
    //Texto en blanco
    framebuffer.draw_text(goal_text, start_x_small, center_y + 10, char_width_small, Color::WHITE);
    
    //"Press R to restart" - instrucciones con borde
    let restart_text = "Press R to restart";
    let restart_width = restart_text.len() as u32 * 16;
    let start_x_restart = center_x.saturating_sub(restart_width / 2);
    
    //Borde negro
    framebuffer.draw_text(restart_text, start_x_restart + 1, center_y + 49, 16, Color::BLACK);
    framebuffer.draw_text(restart_text, start_x_restart - 1, center_y + 49, 16, Color::BLACK);
    framebuffer.draw_text(restart_text, start_x_restart, center_y + 49, 16, Color::BLACK);
    framebuffer.draw_text(restart_text, start_x_restart, center_y + 51, 16, Color::BLACK);
    
    //Texto en amarillo
    framebuffer.draw_text(restart_text, start_x_restart, center_y + 50, 16, Color::YELLOW);
}

fn main() {
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;
    
    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();
    
    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Color::new(50, 50, 100, 255));
    
    //Cargar solo la textura de paredes
    let wall_texture = Image::load_image("assets/img/bosque.jpg")
        .expect("No se pudo cargar la textura bosque.jpg");
    
    //Cargar la textura de paredes en el cache del framebuffer para acceso rápido
    framebuffer.load_texture_cache(&wall_texture);
    
    let maze = load_maze("maze.txt");
    let mut player = Player {
        pos: Vector2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0,
    };
    
    let mut mode = "2D"; //Empezamos en modo 2D
    let mut game_won = false; //Estado del juego
    
    //Variables para FPS
    let target_fps = 15.0;
    let frame_time = Duration::from_secs_f32(1.0 / target_fps);
    let mut fps_counter = 0;
    let mut fps_timer = Instant::now();
    let mut current_fps = 0.0;
    
    while !window.window_should_close() {
        let frame_start = Instant::now();
        
        //1. Limpiar framebuffer
        framebuffer.clear();
        
        if game_won {
            //Cargar y dibujar imagen de fondo de éxito
            let success_image = Image::load_image("assets/img/success_screen.jpg");
            if let Ok(mut img) = success_image {
                //Redimensionar la imagen para que cubra exactamente toda la ventana
                img.resize(framebuffer.width as i32, framebuffer.height as i32);
                
                //Transferir imagen redimensionada al framebuffer pixel por pixel
                for y in 0..framebuffer.height {
                    for x in 0..framebuffer.width {
                        let color = img.get_color(x as i32, y as i32);
                        framebuffer.color_buffer.draw_pixel(x as i32, y as i32, color);
                    }
                }
            }
            
            //Pantalla de éxito
            draw_victory_screen(&mut framebuffer);
            
            //Verificar si quiere reiniciar
            if window.is_key_pressed(KeyboardKey::KEY_R) {
                game_won = false;
                //Reiniciar posición del jugador
                player.pos = Vector2::new(150.0, 150.0);
                player.a = PI / 3.0;
                // Restaurar el color del cielo original
                framebuffer.set_background_color(Color::new(50, 50, 100, 255)); // Azul oscuro original
            }
        } else {
            //2. Procesar input del jugador
            process_events(&mut player, &window, &maze, block_size);
            
            //3. Verificar victoria
            if check_victory(&player, &maze, block_size) {
                game_won = true;
            }
            
            //4. Cambiar modo con la tecla M o gamepad
            if window.is_key_pressed(KeyboardKey::KEY_M) || check_gamepad_mode_change(&window) {
                mode = if mode == "2D" { "3D" } else { "2D" };
            }
            
            //5. Renderizar según el modo
            if mode == "2D" {
                render_maze(&mut framebuffer, &maze, block_size, &player);
            } else {
                render_world(&mut framebuffer, &maze, block_size, &player);
                //Solo mostrar minimapa en modo 3D
                draw_minimap(&mut framebuffer, &maze, &player, block_size);
            }
            
            //6. Dibujar información en pantalla
            let fps_text = format!("FPS: {:.1}", current_fps);
            framebuffer.draw_text(&fps_text, 10, 10, 16, Color::WHITE);
            
            let mode_text = format!("Mode: {} (Press M or Triangle to change)", mode);
            framebuffer.draw_text(&mode_text, 10, 30, 16, Color::WHITE);
            
            let gamepad_text = get_gamepad_info(&window);
            framebuffer.draw_text(&gamepad_text, 10, 50, 16, Color::WHITE);
        }
        
        //5. Calcular FPS
        fps_counter += 1;
        if fps_timer.elapsed() >= Duration::from_secs(1) {
            current_fps = fps_counter as f32 / fps_timer.elapsed().as_secs_f32();
            fps_counter = 0;
            fps_timer = Instant::now();
        }
        
        //6. Intercambiar buffers
        framebuffer.swap_buffers(&mut window, &raylib_thread);
        
        //7. Control de FPS - mantener 15 FPS estables
        let frame_duration = frame_start.elapsed();
        if frame_duration < frame_time {
            thread::sleep(frame_time - frame_duration);
        }
    }
}