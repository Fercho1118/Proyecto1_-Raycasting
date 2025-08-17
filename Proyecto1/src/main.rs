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
use player::{Player, process_events};
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
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
        
        let distance_to_wall = intersect.distance;
        let distance_to_projection_plane = 70.0;
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;
        
        let stake_top = ((hh - (stake_height / 2.0)) as usize).max(0);
        let stake_bottom = ((hh + (stake_height / 2.0)) as usize).min(framebuffer.height as usize);
        
        //Color basado en el tipo de pared
        let color = cell_to_color(intersect.impact);
        framebuffer.set_current_color(color);
        
        for y in stake_top..stake_bottom {
            if i < framebuffer.width && y < framebuffer.height as usize {
                framebuffer.set_pixel(i, y as u32);
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
        .title("Raycaster Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();
    
    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Color::new(50, 50, 100, 255));
    
    let maze = load_maze("maze.txt");
    let mut player = Player {
        pos: Vector2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0,
    };
    
    let mut mode = "2D"; //Empezamos en modo 2D
    
    while !window.window_should_close() {
        //1. Limpiar framebuffer
        framebuffer.clear();
        
        //2. Procesar input del jugador
        process_events(&mut player, &window, &maze, block_size);
        
        //3. Cambiar modo con la tecla M
        if window.is_key_pressed(KeyboardKey::KEY_M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }
        
        //4. Renderizar según el modo
        if mode == "2D" {
            render_maze(&mut framebuffer, &maze, block_size, &player);
        } else {
            render_world(&mut framebuffer, &maze, block_size, &player);
        }
        
        //5. Intercambiar buffers
        framebuffer.swap_buffers(&mut window, &raylib_thread);
        thread::sleep(Duration::from_millis(16));
    }
}