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
    
    // Efeito de slow motion se ativo
    if battle.is_final_blow {
        draw_slow_motion_effect(battle);
    }
}

fn draw_slow_motion_effect(battle: &BattleState) {
    let screen_width = screen_width();
    let screen_height = screen_height();
    
    // Calcular alpha baseado no timer (pulsação)
    let pulse = ((get_time() * 3.0).sin() * 0.3 + 0.7) as f32; // Pulsação entre 0.4 e 1.0
    let alpha = pulse * 0.3; // Alpha máximo de 0.3
    
    // Fundo vermelho semi-transparente
    draw_rectangle(
        0.0,
        0.0,
        screen_width,
        screen_height,
        Color::new(0.8f32, 0.0f32, 0.0f32, alpha),
    );
    
    // Bordas vermelhas pulsantes
    let border_thickness = 10.0;
    let border_alpha = pulse * 0.8f32;
    
    // Borda superior
    draw_rectangle(
        0.0,
        0.0,
        screen_width,
        border_thickness,
        Color::new(1.0f32, 0.0f32, 0.0f32, border_alpha),
    );
    
    // Borda inferior
    draw_rectangle(
        0.0,
        screen_height - border_thickness,
        screen_width,
        border_thickness,
        Color::new(1.0f32, 0.0f32, 0.0f32, border_alpha),
    );
    
    // Borda esquerda
    draw_rectangle(
        0.0,
        0.0,
        border_thickness,
        screen_height,
        Color::new(1.0f32, 0.0f32, 0.0f32, border_alpha),
    );
    
    // Borda direita
    draw_rectangle(
        screen_width - border_thickness,
        0.0,
        border_thickness,
        screen_height,
        Color::new(1.0f32, 0.0f32, 0.0f32, border_alpha),
    );
    
    // Texto "GOLPE FINAL!" no centro
    let text = "💀 GOLPE FINAL! 💀";
    let text_size = 48.0;
    let text_dims = measure_text(text, None, text_size as u16, 1.0);
    
    // Sombra do texto
    draw_text(
        text,
        (screen_width - text_dims.width) / 2.0 + 2.0,
        (screen_height - text_dims.height) / 2.0 + 2.0,
        text_size,
        Color::new(0.0f32, 0.0f32, 0.0f32, 0.8f32),
    );
    
    // Texto principal
    draw_text(
        text,
        (screen_width - text_dims.width) / 2.0,
        (screen_height - text_dims.height) / 2.0,
        text_size,
        Color::new(1.0f32, 0.0f32, 0.0f32, pulse),
    );
    
    // Timer no canto
    let timer_text = format!("{:.1}s", battle.slow_motion_timer);
    let timer_size = 24.0;
    draw_text(
        &timer_text,
        screen_width - 100.0,
        50.0,
        timer_size,
        Color::new(1.0f32, 1.0f32, 1.0f32, pulse),
    );
}