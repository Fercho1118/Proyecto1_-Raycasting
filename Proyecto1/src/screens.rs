use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::game_state::{GameManager, Difficulty, GameState};
use crate::maze::load_maze;
use crate::player::Player;
use crate::audio::AudioManager;
use std::f32::consts::PI;

pub fn draw_welcome_screen(framebuffer: &mut Framebuffer, game_manager: &GameManager) {
    //Cargar y dibujar imagen de fondo de bienvenida
    let welcome_image = Image::load_image("assets/img/welcome_screen.jpg");
    if let Ok(mut img) = welcome_image {
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
    
    //Título del juego
    let center_y = framebuffer.height / 2;
    
    framebuffer.set_current_color(Color::new(0, 0, 0, 200)); 
    
    let menu_width = 500;
    let menu_height = 400;
    //Posicionar el menú en el tercio izquierdo de la pantalla
    let menu_x = framebuffer.width / 6; 
    let menu_y = center_y.saturating_sub(menu_height / 2);
    
    for x in menu_x..menu_x + menu_width {
        for y in menu_y..menu_y + menu_height {
            if x < framebuffer.width && y < framebuffer.height {
                framebuffer.set_pixel(x, y);
            }
        }
    }
    
    //Título "Naruto Maze" 
    let title_text = "Naruto Maze";
    let title_char_width = 28;
    let title_width = title_text.len() as u32 * title_char_width;
    let title_x = menu_x + (menu_width - title_width) / 2; 
    
    //Borde dorado para el título
    framebuffer.draw_text(title_text, title_x + 2, center_y - 148, title_char_width, Color::new(255, 215, 0, 255));
    framebuffer.draw_text(title_text, title_x - 2, center_y - 148, title_char_width, Color::new(255, 215, 0, 255));
    framebuffer.draw_text(title_text, title_x, center_y - 150, title_char_width, Color::new(255, 215, 0, 255));
    framebuffer.draw_text(title_text, title_x, center_y - 146, title_char_width, Color::new(255, 215, 0, 255));
    
    //Título principal en blanco
    framebuffer.draw_text(title_text, title_x, center_y - 147, title_char_width, Color::WHITE);
    
    //Subtítulo "Select Difficulty"
    let subtitle_text = "Select Difficulty";
    let subtitle_char_width = 20;
    let subtitle_width = subtitle_text.len() as u32 * subtitle_char_width;
    let subtitle_x = menu_x + (menu_width.saturating_sub(subtitle_width)) / 2;
    
    //Borde negro
    framebuffer.draw_text(subtitle_text, subtitle_x + 1, center_y - 79, subtitle_char_width, Color::BLACK);
    framebuffer.draw_text(subtitle_text, subtitle_x - 1, center_y - 79, subtitle_char_width, Color::BLACK);
    framebuffer.draw_text(subtitle_text, subtitle_x, center_y - 81, subtitle_char_width, Color::BLACK);
    framebuffer.draw_text(subtitle_text, subtitle_x, center_y - 77, subtitle_char_width, Color::BLACK);
    
    //Subtítulo en gris claro
    framebuffer.draw_text(subtitle_text, subtitle_x, center_y - 79, subtitle_char_width, Color::LIGHTGRAY);
    
    //Opciones de dificultad
    let options = [
        Difficulty::Easy,
        Difficulty::Medium,
        Difficulty::Difficult,
    ];
    
    for (i, difficulty) in options.iter().enumerate() {
        let option_text = difficulty.get_name();
        let option_char_width = 24;
        let option_width = option_text.len() as u32 * option_char_width;
        let option_x = menu_x + (menu_width.saturating_sub(option_width)) / 2;
        let option_y = center_y - 30 + (i as u32 * 40);
        
        //Color según si está seleccionado
        let (text_color, border_color) = if i == game_manager.selected_option {
            (Color::YELLOW, Color::ORANGE) //Opción seleccionada
        } else {
            (Color::WHITE, Color::BLACK) //Opción normal
        };
        
        //Borde
        framebuffer.draw_text(option_text, option_x + 1, option_y + 1, option_char_width, border_color);
        framebuffer.draw_text(option_text, option_x - 1, option_y + 1, option_char_width, border_color);
        framebuffer.draw_text(option_text, option_x + 1, option_y - 1, option_char_width, border_color);
        framebuffer.draw_text(option_text, option_x - 1, option_y - 1, option_char_width, border_color);
        
        //Texto principal
        framebuffer.draw_text(option_text, option_x, option_y, option_char_width, text_color);
        
        //Indicador de selección
        if i == game_manager.selected_option {
            let arrow = ">";
            framebuffer.draw_text(arrow, option_x - 40, option_y, option_char_width, Color::YELLOW);
        }
    }
    
    //Instrucciones
    let instructions1 = "Use UP/DOWN or Joystick to navigate";
    let instructions2 = "Press ENTER or A to select";
    let instructions3 = "Press ESC or Back/B to exit";
    let instr_char_width = 14;
    
    let instr1_width = instructions1.len() as u32 * instr_char_width;
    let instr1_x = menu_x + (menu_width.saturating_sub(instr1_width)) / 2;
    
    let instr2_width = instructions2.len() as u32 * instr_char_width;
    let instr2_x = menu_x + (menu_width.saturating_sub(instr2_width)) / 2;
    
    let instr3_width = instructions3.len() as u32 * instr_char_width;
    let instr3_x = menu_x + (menu_width.saturating_sub(instr3_width)) / 2;
    
    //Bordes negros para las instrucciones
    framebuffer.draw_text(instructions1, instr1_x + 1, center_y + 151, instr_char_width, Color::BLACK);
    framebuffer.draw_text(instructions1, instr1_x - 1, center_y + 151, instr_char_width, Color::BLACK);
    framebuffer.draw_text(instructions1, instr1_x, center_y + 149, instr_char_width, Color::BLACK);
    framebuffer.draw_text(instructions1, instr1_x, center_y + 153, instr_char_width, Color::BLACK);
    
    framebuffer.draw_text(instructions2, instr2_x + 1, center_y + 171, instr_char_width, Color::BLACK);
    framebuffer.draw_text(instructions2, instr2_x - 1, center_y + 171, instr_char_width, Color::BLACK);
    framebuffer.draw_text(instructions2, instr2_x, center_y + 169, instr_char_width, Color::BLACK);
    framebuffer.draw_text(instructions2, instr2_x, center_y + 173, instr_char_width, Color::BLACK);
    
    framebuffer.draw_text(instructions3, instr3_x + 1, center_y + 191, instr_char_width, Color::BLACK);
    framebuffer.draw_text(instructions3, instr3_x - 1, center_y + 191, instr_char_width, Color::BLACK);
    framebuffer.draw_text(instructions3, instr3_x, center_y + 189, instr_char_width, Color::BLACK);
    framebuffer.draw_text(instructions3, instr3_x, center_y + 193, instr_char_width, Color::BLACK);
    
    //Instrucciones en cyan
    framebuffer.draw_text(instructions1, instr1_x, center_y + 150, instr_char_width, Color::SKYBLUE);
    framebuffer.draw_text(instructions2, instr2_x, center_y + 170, instr_char_width, Color::SKYBLUE);
    framebuffer.draw_text(instructions3, instr3_x, center_y + 190, instr_char_width, Color::SKYBLUE);
}

pub fn draw_victory_screen(framebuffer: &mut Framebuffer) {
    //Usar la imagen de fondo en lugar de color verde
    //(La imagen ya estará cargada en el framebuffer desde el main)
    
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
    
    //"SUCCESS!" - texto principal con borde para mejor contraste
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
    
    //"Press M for Menu or R to restart" - instrucciones con borde
    let restart_text = "Press M for Menu or R to restart";
    let restart_width = restart_text.len() as u32 * 16;
    let start_x_restart = center_x.saturating_sub(restart_width / 2);
    
    //Borde negro
    framebuffer.draw_text(restart_text, start_x_restart + 1, center_y + 49, 16, Color::BLACK);
    framebuffer.draw_text(restart_text, start_x_restart - 1, center_y + 49, 16, Color::BLACK);
    framebuffer.draw_text(restart_text, start_x_restart, center_y + 49, 16, Color::BLACK);
    framebuffer.draw_text(restart_text, start_x_restart, center_y + 51, 16, Color::BLACK);
    
    //Texto en amarillo
    framebuffer.draw_text(restart_text, start_x_restart, center_y + 50, 16, Color::YELLOW);
    
    //"Start/Select for Menu/Restart" - instrucciones de gamepad
    let gamepad_text = "Start/Select for Menu/Restart";
    let gamepad_width = gamepad_text.len() as u32 * 14;
    let start_x_gamepad = center_x.saturating_sub(gamepad_width / 2);
    
    //Borde negro
    framebuffer.draw_text(gamepad_text, start_x_gamepad + 1, center_y + 69, 14, Color::BLACK);
    framebuffer.draw_text(gamepad_text, start_x_gamepad - 1, center_y + 69, 14, Color::BLACK);
    framebuffer.draw_text(gamepad_text, start_x_gamepad, center_y + 69, 14, Color::BLACK);
    framebuffer.draw_text(gamepad_text, start_x_gamepad, center_y + 71, 14, Color::BLACK);
    
    //Texto en cian
    framebuffer.draw_text(gamepad_text, start_x_gamepad, center_y + 70, 14, Color::SKYBLUE);
}

#[derive(PartialEq)]
pub enum VictoryAction {
    None,
    BackToMenu,
    RestartLevel,
}

pub fn handle_victory_input(
    game_manager: &mut GameManager,
    window: &RaylibHandle,
    audio_manager: &AudioManager,
) -> VictoryAction {
    //Manejar input de victoria
    if window.is_key_pressed(KeyboardKey::KEY_M) || 
       (window.is_gamepad_available(0) && window.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT)) {
        audio_manager.play_menu_sound();
        game_manager.reset_to_welcome();
        VictoryAction::BackToMenu
    } else if window.is_key_pressed(KeyboardKey::KEY_R) ||
             (window.is_gamepad_available(0) && window.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT)) {
        audio_manager.play_start_sound();
        //Señalar que se debe reiniciar el nivel
        game_manager.state = GameState::Playing;
        VictoryAction::RestartLevel
    } else {
        VictoryAction::None
    }
}

pub fn render_victory_screen(framebuffer: &mut Framebuffer) {
    //Cargar imagen de fondo de victoria
    let success_image = Image::load_image("assets/img/success_screen.jpg");
    if let Ok(mut img) = success_image {
        img.resize(framebuffer.width as i32, framebuffer.height as i32);
        for y in 0..framebuffer.height {
            for x in 0..framebuffer.width {
                let color = img.get_color(x as i32, y as i32);
                framebuffer.color_buffer.draw_pixel(x as i32, y as i32, color);
            }
        }
    }
    
    //Dibujar pantalla de victoria
    draw_victory_screen(framebuffer);
}

pub fn handle_welcome_input(game_manager: &mut GameManager, window: &RaylibHandle, audio_manager: &AudioManager) {
    //Navegación con teclado
    if window.is_key_pressed(KeyboardKey::KEY_UP) {
        audio_manager.play_up_down_sound();
        game_manager.selected_option = if game_manager.selected_option > 0 {
            game_manager.selected_option - 1
        } else {
            2 //Volver al final
        };
    }
    
    if window.is_key_pressed(KeyboardKey::KEY_DOWN) {
        audio_manager.play_up_down_sound();
        game_manager.selected_option = (game_manager.selected_option + 1) % 3;
    }
    
    //Navegación con gamepad
    if window.is_gamepad_available(0) {
        let left_stick_y = window.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_Y);
        static mut LAST_STICK_INPUT: f32 = 0.0;
        
        unsafe {
            if left_stick_y < -0.5 && LAST_STICK_INPUT >= -0.5 {
                audio_manager.play_up_down_sound();
                game_manager.selected_option = if game_manager.selected_option > 0 {
                    game_manager.selected_option - 1
                } else {
                    2
                };
            } else if left_stick_y > 0.5 && LAST_STICK_INPUT <= 0.5 {
                audio_manager.play_up_down_sound();
                game_manager.selected_option = (game_manager.selected_option + 1) % 3;
            }
            LAST_STICK_INPUT = left_stick_y;
        }
    }
    
    //Selección con Enter o botón A del gamepad
    if window.is_key_pressed(KeyboardKey::KEY_ENTER) || 
       (window.is_gamepad_available(0) && window.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN)) {
        audio_manager.play_start_sound();
        let difficulty = match game_manager.selected_option {
            0 => Difficulty::Easy,
            1 => Difficulty::Medium,
            2 => Difficulty::Difficult,
            _ => Difficulty::Easy,
        };
        game_manager.start_game(difficulty);
    }
    
    //Controles adicionales para el menú con gamepad
    if window.is_gamepad_available(0) {
        //Botón Circle/B para salir del juego
        if window.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT) {
            std::process::exit(0);
        }
    }
}
