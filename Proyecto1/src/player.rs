use raylib::prelude::*;
use std::f32::consts::PI;
use crate::maze::Maze;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32,
}

pub fn get_gamepad_info(rl: &RaylibHandle) -> String {
    if rl.is_gamepad_available(0) {
        match rl.get_gamepad_name(0) {
            Some(name) => format!("Gamepad: {}", name),
            None => "Gamepad: Unknown".to_string(),
        }
    } else {
        "No gamepad detected".to_string()
    }
}

pub fn check_gamepad_mode_change(rl: &RaylibHandle) -> bool {
    if rl.is_gamepad_available(0) {
        //Usar el botón Triangle (Y en Xbox, Triangle en PS) para cambiar modo
        rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP)
    } else {
        false
    }
}

fn is_valid_position(pos: Vector2, maze: &Maze, block_size: usize) -> bool {
    let x = pos.x as usize / block_size;
    let y = pos.y as usize / block_size;
    
    //Verificar límites del laberinto
    if y >= maze.len() || x >= maze[0].len() {
        return false;
    }
    
    //Verificar si la posición es un espacio vacío o la meta
    maze[y][x] == ' ' || maze[y][x] == 'g'
}

pub fn check_victory(player: &Player, maze: &Maze, block_size: usize) -> bool {
    let x = player.pos.x as usize / block_size;
    let y = player.pos.y as usize / block_size;
    
    //Verificar límites del laberinto
    if y >= maze.len() || x >= maze[0].len() {
        return false;
    }
    
    //Verificar si el jugador está en la meta (celda 'g')
    maze[y][x] == 'g'
}

pub fn process_events(player: &mut Player, rl: &RaylibHandle, maze: &Maze, block_size: usize) {
    const MOVE_SPEED: f32 = 5.0;
    const ROTATION_SPEED: f32 = PI / 50.0;
    const MOUSE_SENSITIVITY: f32 = 0.003;
    const GAMEPAD_SENSITIVITY: f32 = 0.05;
    const GAMEPAD_DEADZONE: f32 = 0.1;
    
    //Verificar si hay un gamepad conectado
    let gamepad_available = rl.is_gamepad_available(0);
    
    //Rotación con mouse (solo horizontal)
    let mouse_delta = rl.get_mouse_delta();
    player.a += mouse_delta.x * MOUSE_SENSITIVITY; 
    
    //Rotación con teclado
    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        player.a -= ROTATION_SPEED;  //Girar a la izquierda
    }
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.a += ROTATION_SPEED;  //Girar a la derecha
    }
    
    //Rotación con gamepad (stick derecho)
    if gamepad_available {
        let right_stick_x = rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_RIGHT_X);
        if right_stick_x.abs() > GAMEPAD_DEADZONE {
            player.a += right_stick_x * GAMEPAD_SENSITIVITY;
        }
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
    
    //Movimiento con gamepad (stick izquierdo)
    if gamepad_available {
        let left_stick_y = rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_Y);
        if left_stick_y.abs() > GAMEPAD_DEADZONE {
            let new_pos = Vector2::new(
                player.pos.x + MOVE_SPEED * (-left_stick_y) * player.a.cos(),
                player.pos.y + MOVE_SPEED * (-left_stick_y) * player.a.sin(),
            );
            
            if is_valid_position(new_pos, maze, block_size) {
                player.pos = new_pos;
            }
        }
    }
    
    //Movimiento lateral
    if rl.is_key_down(KeyboardKey::KEY_A) {
        let new_pos = Vector2::new(
            player.pos.x + MOVE_SPEED * (player.a - PI/2.0).cos(),  //Movimiento a la izquierda
            player.pos.y + MOVE_SPEED * (player.a - PI/2.0).sin(),
        );
        
        if is_valid_position(new_pos, maze, block_size) {
            player.pos = new_pos;
        }
    }
    
    if rl.is_key_down(KeyboardKey::KEY_D) {
        let new_pos = Vector2::new(
            player.pos.x + MOVE_SPEED * (player.a + PI/2.0).cos(),  //Movimiento a la derecha
            player.pos.y + MOVE_SPEED * (player.a + PI/2.0).sin(),
        );
        
        if is_valid_position(new_pos, maze, block_size) {
            player.pos = new_pos;
        }
    }
    
    //Movimiento lateral con gamepad (stick izquierdo horizontal)
    if gamepad_available {
        let left_stick_x = rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_X);
        if left_stick_x.abs() > GAMEPAD_DEADZONE {
            let angle = if left_stick_x < 0.0 {
                player.a - PI/2.0  //Movimiento a la izquierda 
            } else {
                player.a + PI/2.0  //Movimiento a la derecha
            };
            
            let new_pos = Vector2::new(
                player.pos.x + MOVE_SPEED * left_stick_x.abs() * angle.cos(),
                player.pos.y + MOVE_SPEED * left_stick_x.abs() * angle.sin(),
            );
            
            if is_valid_position(new_pos, maze, block_size) {
                player.pos = new_pos;
            }
        }
    }
}