#![allow(unused_imports)]
#![allow(dead_code)]

mod line;
mod framebuffer;
mod maze;
mod caster;
mod player;
mod game_state;
mod screens;

use line::line;
use maze::{Maze, load_maze};
use caster::{cast_ray, Intersect};
use framebuffer::Framebuffer;
use player::{Player, process_events, get_gamepad_info, check_gamepad_mode_change, check_victory};
use game_state::{GameManager, GameState, Difficulty};
use screens::{draw_welcome_screen, draw_victory_screen, handle_victory_input, render_victory_screen, handle_welcome_input, VictoryAction};
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
    if maze.is_empty() {
        return;
    }
    
    //Configuración del minimapa adaptativo
    let maze_width = maze[0].len();
    let maze_height = maze.len();
    
    //Calcular escala del minimapa para que quepa en la esquina
    let max_minimap_width = 250; //Máximo ancho del minimapa
    let max_minimap_height = 200; //Máximo alto del minimapa
    
    let scale_by_width = max_minimap_width / maze_width;
    let scale_by_height = max_minimap_height / maze_height;
    let minimap_scale = scale_by_width.min(scale_by_height).max(4); //Mínimo 4 píxeles por celda
    
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
    
    //Jugador como círculo rojo (escalar según el tamaño del minimapa)
    framebuffer.set_current_color(Color::RED);
    let radius = (minimap_scale / 3).max(2); //Radio proporcional al tamaño de celda
    for dx in -(radius as i32)..=(radius as i32) {
        for dy in -(radius as i32)..=(radius as i32) {
            if dx * dx + dy * dy <= (radius * radius) as i32 {
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

fn draw_scaled_ray(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    angle: f32,
    original_block_size: usize,
    scale_factor: f32,
    offset_x: usize,
    offset_y: usize,
) {
    let mut d = 0.0;
    let step_size = 1.0;
    
    loop {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        
        let x = player.pos.x + d * cos_a;
        let y = player.pos.y + d * sin_a;
        
        let i = (x / original_block_size as f32) as usize;
        let j = (y / original_block_size as f32) as usize;
        
        if j >= maze.len() || i >= maze[j].len() || maze[j][i] != ' ' || d > 200.0 {
            break;
        }
        
        //Dibujar punto escalado
        let scaled_x = (offset_x as f32 + x * scale_factor) as i32;
        let scaled_y = (offset_y as f32 + y * scale_factor) as i32;
        
        if scaled_x >= 0 && scaled_y >= 0 && 
           scaled_x < framebuffer.width as i32 && scaled_y < framebuffer.height as i32 {
            framebuffer.set_pixel(scaled_x as u32, scaled_y as u32);
        }
        
        d += step_size;
    }
}

fn calculate_adaptive_block_size(maze: &Maze, framebuffer: &Framebuffer) -> usize {
    if maze.is_empty() {
        return 100; //Default fallback
    }
    
    let maze_width = maze[0].len();
    let maze_height = maze.len();
    
    //Calcular el tamaño de bloque que mejor se ajuste a la pantalla
    //Dejar un margen de 50 píxeles en cada lado
    let available_width = framebuffer.width as usize - 100;
    let available_height = framebuffer.height as usize - 100;
    
    let block_size_by_width = available_width / maze_width;
    let block_size_by_height = available_height / maze_height;
    
    //Usar el menor de los dos para asegurar que el laberinto completo sea visible
    let adaptive_size = block_size_by_width.min(block_size_by_height);
    
    //Asegurar un tamaño mínimo de 20 píxeles para que sea visible
    adaptive_size.max(20)
}

pub fn render_maze(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
) {
    //Calcular el tamaño de bloque adaptativo para modo 2D
    let adaptive_block_size = calculate_adaptive_block_size(maze, framebuffer);
    
    //Calcular offset para centrar el laberinto
    let maze_width = maze[0].len() * adaptive_block_size;
    let maze_height = maze.len() * adaptive_block_size;
    let offset_x = (framebuffer.width as usize - maze_width) / 2;
    let offset_y = (framebuffer.height as usize - maze_height) / 2;
    
    //Calcular factor de escala para mantener proporciones
    let scale_factor = adaptive_block_size as f32 / block_size as f32;
    
    //Dibujar el laberinto escalado
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = offset_x + col_index * adaptive_block_size;
            let yo = offset_y + row_index * adaptive_block_size;
            draw_cell(framebuffer, xo, yo, adaptive_block_size, cell);
        }
    }
    
    //Dibujar al jugador como un círculo rojo
    framebuffer.set_current_color(Color::RED);
    
    //Escalar y centrar la posición del jugador
    let player_x = (offset_x as f32 + player.pos.x * scale_factor) as i32;
    let player_y = (offset_y as f32 + player.pos.y * scale_factor) as i32;
    let radius = (8.0 * scale_factor).max(6.0) as i32; //Asegurar mínimo 6 píxeles
    
    //Verificar que las coordenadas estén dentro del rango visible
    if player_x >= 0 && player_y >= 0 && 
       player_x < framebuffer.width as i32 && player_y < framebuffer.height as i32 {
        
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
    }
    
    //Dibujar la dirección del jugador
    framebuffer.set_current_color(Color::YELLOW);
    let direction_length = 30.0 * scale_factor;
    let end_x = player.pos.x + direction_length * player.a.cos();
    let end_y = player.pos.y + direction_length * player.a.sin();
    
    line(
        framebuffer,
        Vector2::new(offset_x as f32 + player.pos.x * scale_factor, offset_y as f32 + player.pos.y * scale_factor),
        Vector2::new(offset_x as f32 + end_x * scale_factor, offset_y as f32 + end_y * scale_factor),
    );
    
    //Dibujar algunos rayos para visualizar el FOV
    framebuffer.set_current_color(Color::WHITESMOKE);
    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / (num_rays - 1) as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        
        //Usar una versión modificada de cast_ray que dibuja líneas escaladas
        draw_scaled_ray(framebuffer, &maze, &player, a, block_size, scale_factor, offset_x, offset_y);
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

fn main() {
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;
    
    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster - Select Level")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();
    
    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Color::new(50, 50, 100, 255));
    
    //Cargar textura de paredes
    let wall_texture = Image::load_image("assets/img/bosque.jpg")
        .expect("No se pudo cargar la textura bosque.jpg");
    framebuffer.load_texture_cache(&wall_texture);
    
    //Game manager para estados
    let mut game_manager = GameManager::new();
    
    //Variables del juego
    let mut maze = load_maze("maze_easy.txt"); 
    let mut player = Player {
        pos: Vector2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0,
    };
    
    let mut mode = "2D";
    
    //Variables para FPS
    let target_fps = 15.0;
    let frame_time = Duration::from_secs_f32(1.0 / target_fps);
    let mut fps_counter = 0;
    let mut fps_timer = Instant::now();
    let mut current_fps = 0.0;
    
    while !window.window_should_close() {
        let frame_start = Instant::now();
        framebuffer.clear();
        
        match game_manager.state {
            GameState::Welcome => {
                //Manejar input del menú de bienvenida
                handle_welcome_input(&mut game_manager, &window);
                
                //Si se seleccionó un nivel, cargar el laberinto correspondiente
                if game_manager.state == GameState::Playing {
                    maze = load_maze(game_manager.current_difficulty.get_maze_file());
                    player.pos = Vector2::new(150.0, 150.0);
                    player.a = PI / 3.0;
                }
                
                //Dibujar pantalla de bienvenida
                draw_welcome_screen(&mut framebuffer, &game_manager);
            },
            
            GameState::Playing => {
                //Lógica del juego normal
                process_events(&mut player, &window, &maze, block_size);
                
                //Verificar victoria
                if check_victory(&player, &maze, block_size) {
                    game_manager.win_game();
                }
                
                //Controles adicionales con gamepad
                if window.is_gamepad_available(0) {
                    // Botón Start/Options - Ir al menú principal
                    if window.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT) {
                        game_manager.reset_to_welcome();
                    }
                    
                    //Botón Select/Share - Reset nivel actual
                    if window.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT) {
                        maze = load_maze(game_manager.current_difficulty.get_maze_file());
                        player.pos = Vector2::new(150.0, 150.0);
                        player.a = PI / 3.0;
                        framebuffer.set_background_color(Color::new(50, 50, 100, 255));
                    }
                }
                
                //Cambiar modo con la tecla M o gamepad
                if window.is_key_pressed(KeyboardKey::KEY_M) || check_gamepad_mode_change(&window) {
                    mode = if mode == "2D" { "3D" } else { "2D" };
                }
                
                //Dibujar juego según el modo
                if mode == "2D" {
                    render_maze(&mut framebuffer, &maze, block_size, &player);
                } else {
                    render_world(&mut framebuffer, &maze, block_size, &player);
                    //Solo mostrar minimapa en modo 3D
                    draw_minimap(&mut framebuffer, &maze, &player, block_size);
                }
                
                //Mostrar información
                let fps_text = format!("FPS: {:.1}", current_fps);
                framebuffer.draw_text(&fps_text, 10, 10, 16, Color::WHITE);
                
                let mode_text = format!("Mode: {} (Press M or Triangle to change)", mode);
                framebuffer.draw_text(&mode_text, 10, 30, 16, Color::WHITE);
                
                //Mover los controles a la esquina inferior izquierda para evitar superposición
                let controls_text = "Options=Menu | Share=Reset";
                let controls_y = framebuffer.height.saturating_sub(25);
                framebuffer.draw_text(&controls_text, 10, controls_y, 14, Color::LIGHTGRAY);
                
                let level_text = format!("Level: {}", game_manager.current_difficulty.get_name());
                framebuffer.draw_text(&level_text, 10, 50, 16, Color::WHITE);
                
                let gamepad_text = get_gamepad_info(&window);
                framebuffer.draw_text(&gamepad_text, 10, 70, 16, Color::WHITE);
            },
            
            GameState::Victory => {
                //Renderizar pantalla de victoria con imagen de fondo
                render_victory_screen(&mut framebuffer);
                
                //Manejar input de victoria
                let action = handle_victory_input(&mut game_manager, &window);
                match action {
                    VictoryAction::RestartLevel => {
                        //Reiniciar el mismo nivel
                        maze = load_maze(game_manager.current_difficulty.get_maze_file());
                        player.pos = Vector2::new(150.0, 150.0);
                        player.a = PI / 3.0;
                        framebuffer.set_background_color(Color::new(50, 50, 100, 255));
                    },
                    _ => {} //BackToMenu y None se manejan automáticamente
                }
            }
        }
        
        //Calcular FPS
        fps_counter += 1;
        if fps_timer.elapsed() >= Duration::from_secs(1) {
            current_fps = fps_counter as f32 / fps_timer.elapsed().as_secs_f32();
            fps_counter = 0;
            fps_timer = Instant::now();
        }
        
        //Intercambiar buffers
        framebuffer.swap_buffers(&mut window, &raylib_thread);
        
        //Control de FPS - mantener 15 FPS estables
        let frame_duration = frame_start.elapsed();
        if frame_duration < frame_time {
            thread::sleep(frame_time - frame_duration);
        }
    }
}