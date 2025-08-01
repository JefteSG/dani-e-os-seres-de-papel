use macroquad::prelude::*;

mod deck;
mod effects;
mod enemy;
mod entity;
mod gameturn;
mod player;
mod state;

use state::game_state::GameState;

fn window_conf() -> Conf {
    Conf {
        window_title: "Dani e os Seres de Papel".to_owned(),
        window_width: 1024,
        window_height: 768,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::new();
    
    // Carregar fonte com suporte a emojis
    let font = match load_ttf_font("assets/Noto_Emoji/NotoEmoji-VariableFont_wght.ttf").await {
        Ok(font) => {
            println!("✅ Fonte de emoji carregada com sucesso!");
            font
        }
        Err(_) => {
            println!("⚠️ Erro ao carregar fonte de emoji, usando fonte padrão");
            load_ttf_font("assets/Noto_Color_Emoji/NotoColorEmoji-Regular.ttf").await.unwrap_or_else(|_| {
                println!("❌ Erro ao carregar fonte de emoji alternativa");
                panic!("Não foi possível carregar nenhuma fonte de emoji");
            })
        }
    };
    // Armazenar a fonte para uso posterior
    // O macroquad usa a fonte padrão do sistema, mas podemos usar a fonte carregada
    // especificamente em cada chamada de draw_text se necessário
    game_state.emoji_font = Some(font);

    // Carregar texturas das cartas
    game_state.load_card_textures().await;

    loop {
        // Update game logic
        game_state.update();

        // Draw everything
        game_state.draw();

        // Wait for next frame
        next_frame().await;
    }
}
