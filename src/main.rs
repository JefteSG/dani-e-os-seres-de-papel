#[cfg(windows)]
extern crate winapi;

use macroquad::prelude::*;

mod config;
mod deck;
mod effects;
mod enemy;
mod entity;
mod error;
mod gameturn;
mod player;
mod state;

use state::game_state::GameState;

/// Esconde a janela do console no Windows para uma experiência de jogo mais limpa
#[cfg(windows)]
fn hide_console() {
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};
    
    unsafe {
        let window = GetConsoleWindow();
        if !window.is_null() {
            ShowWindow(window, SW_HIDE);
        }
    }
}

/// Configuração da janela do jogo
fn window_conf() -> Conf {
    Conf {
        window_title: "Dani e os Seres de Papel".to_owned(),
        window_width: config::config::DEFAULT_WINDOW_WIDTH,
        window_height: config::config::DEFAULT_WINDOW_HEIGHT,
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
            println!("⚠️ Error loading primary emoji font, trying alternative...");
            load_ttf_font("assets/Noto_Color_Emoji/NotoColorEmoji-Regular.ttf").await
                .unwrap_or_else(|e| {
                    eprintln!("❌ Error loading alternative emoji font: {:?}", e);
                    eprintln!("⚠️ Continuing without emoji support");
                    // Retorna fonte padrão caso ambas falhem
                    panic!("Could not load any emoji font - game requires font support");
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
