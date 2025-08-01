use macroquad::prelude::*;
use crate::state::battle_state::BattleState;
use crate::deck::CardTextureManager;
use super::components::*;

pub fn draw_battle(battle: &BattleState, card_textures: &CardTextureManager, emoji_font: Option<&Font>, enemy_image: &str) {
    let font_size = 22.0;
    let margin = 20.0;

    // 1. TOPO: Informações do inimigo
    draw_enemy_info(battle, margin, font_size, emoji_font);

    // 2. CENTRO-SUPERIOR: Mensagem atual
    let message_y = screen_height() * 0.15;
    draw_text(
        &battle.current_message,
        margin,
        message_y,
        font_size,
        YELLOW,
    );

    // 3. CENTRO: Inimigo com animações
    draw_enemy_with_animation(battle, card_textures, enemy_image);

    // 4. ACIMA DAS CARTAS: Informações do jogador
    draw_player_info_above_cards(battle, font_size, emoji_font);

    // 5. CENTRO-INFERIOR: Cartas da mão do jogador
    draw_player_hand_with_animation(&battle.player.hand, battle, card_textures);

    // 6. PARTE INFERIOR: Instruções
    draw_instructions(battle, font_size);

    // Desenhar partículas de dano (sobreposição)
    for particle in &battle.damage_particles {
        particle.draw();
    }
    
    // Desenhar log de batalha no canto direito
    draw_battle_log(battle, emoji_font);
}