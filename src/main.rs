#[cfg(windows)]
extern crate winapi;

use macroquad::prelude::*;

mod deck;
mod effects;
mod enemy;
mod entity;
mod gameturn;
mod player;
mod state;

use state::game_state::GameState;

#[cfg(windows)]
fn hide_console() {
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};
    use std::ptr;
    
    unsafe {
        let window = GetConsoleWindow();
        if !window.is_null() {
            ShowWindow(window, SW_HIDE);
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Dani e os Seres de Papel".to_owned(),
        window_width: 1024,
        window_height: 768,
        window_resizable: true, 
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    #[cfg(windows)]
    hide_console();
    
    let mut game_state = GameState::new();
    
    let font = match load_ttf_font("assets/Noto_Emoji/NotoEmoji-VariableFont_wght.ttf").await {
        Ok(font) => {
            println!("✅ Emoji font loaded successfully!");
            font
        }
        Err(_) => {
            println!("⚠️ Error loading emoji font, using default font");
            load_ttf_font("assets/Noto_Color_Emoji/NotoColorEmoji-Regular.ttf").await.unwrap_or_else(|_| {
                println!("Error loading alternative emoji font");
                panic!("Could not load any emoji font");
            })
        }
    };

    game_state.emoji_font = Some(font);
    game_state.load_card_textures().await;

    loop {
        game_state.update();
        game_state.draw();
        next_frame().await;
    }
}
