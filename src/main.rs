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
        window_resizable: true, 
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::new();
    
    let font = match load_ttf_font("assets/Noto_Emoji/NotoEmoji-VariableFont_wght.ttf").await {
        Ok(font) => {
            println!("✅ Fonte de emoji carregada com sucesso!");
            font
        }
        Err(_) => {
            println!("⚠️ Erro ao carregar fonte de emoji, usando fonte padrão");
            load_ttf_font("assets/Noto_Color_Emoji/NotoColorEmoji-Regular.ttf").await.unwrap_or_else(|_| {
                println!("Erro ao carregar fonte de emoji alternativa");
                panic!("Não foi possível carregar nenhuma fonte de emoji");
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
