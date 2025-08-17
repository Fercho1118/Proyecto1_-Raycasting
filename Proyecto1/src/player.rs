use raylib::prelude::*;
use std::f32::consts::PI;
use crate::maze::Maze;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32,
}

fn is_valid_position(pos: Vector2, maze: &Maze, block_size: usize) -> bool {
    let x = pos.x as usize / block_size;
    let y = pos.y as usize / block_size;
    
    //Verificar límites del laberinto
    if y >= maze.len() || x >= maze[0].len() {
        return false;
    }
    
    //Verificar si la posición es un espacio vacío
    maze[y][x] == ' '
}

pub fn process_events(player: &mut Player, rl: &RaylibHandle, maze: &Maze, block_size: usize) {
    const MOVE_SPEED: f32 = 5.0;
    const ROTATION_SPEED: f32 = PI / 50.0;
    const MOUSE_SENSITIVITY: f32 = 0.003;
    
    //Rotación con mouse (solo horizontal)
    let mouse_delta = rl.get_mouse_delta();
    player.a += mouse_delta.x * MOUSE_SENSITIVITY; 
    
    //Rotación con teclado
    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        player.a += ROTATION_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.a -= ROTATION_SPEED;
    }
    
    //Movimiento hacia adelante
    if rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_W) {
        let new_pos = Vector2::new(
            player.pos.x + MOVE_SPEED * player.a.cos(),
            player.pos.y + MOVE_SPEED * player.a.sin(),
        );
        
        if is_valid_position(new_pos, maze, block_size) {
            player.pos = new_pos;
        }
    }
    
    //Movimiento hacia atrás
    if rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_S) {
        let new_pos = Vector2::new(
            player.pos.x - MOVE_SPEED * player.a.cos(),
            player.pos.y - MOVE_SPEED * player.a.sin(),
        );
        
        if is_valid_position(new_pos, maze, block_size) {
            player.pos = new_pos;
        }
    }
    
    //Movimiento lateral
    if rl.is_key_down(KeyboardKey::KEY_A) {
        let new_pos = Vector2::new(
            player.pos.x + MOVE_SPEED * (player.a + PI/2.0).cos(),
            player.pos.y + MOVE_SPEED * (player.a + PI/2.0).sin(),
        );
        
        if is_valid_position(new_pos, maze, block_size) {
            player.pos = new_pos;
        }
    }
    
    if rl.is_key_down(KeyboardKey::KEY_D) {
        let new_pos = Vector2::new(
            player.pos.x + MOVE_SPEED * (player.a - PI/2.0).cos(),
            player.pos.y + MOVE_SPEED * (player.a - PI/2.0).sin(),
        );
        
        if is_valid_position(new_pos, maze, block_size) {
            player.pos = new_pos;
        }
    }
}