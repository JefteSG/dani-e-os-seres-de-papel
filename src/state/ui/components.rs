use macroquad::prelude::*;
use crate::state::battle_state::BattleState;
use crate::deck::{Hand, CardTextureManager};
use crate::effects::StatusEffect;

// Função auxiliar para desenhar texto com emojis
pub fn draw_text_with_emoji(text: &str, x: f32, y: f32, font_size: f32, color: Color, emoji_font: Option<&Font>) {
    // Verificar se o texto contém emojis
    let has_emoji = text.chars().any(|c| {
        let code = c as u32;
        // Emojis estão nas faixas Unicode: 0x1F600-0x1F64F, 0x1F300-0x1F5FF, 0x1F680-0x1F6FF, etc.
        (code >= 0x1F600 && code <= 0x1F64F) || // Emoticons
        (code >= 0x1F300 && code <= 0x1F5FF) || // Misc Symbols and Pictographs
        (code >= 0x1F680 && code <= 0x1F6FF) || // Transport and Map Symbols
        (code >= 0x1F900 && code <= 0x1F9FF) || // Supplemental Symbols and Pictographs
        (code >= 0x2600 && code <= 0x26FF) ||   // Misc Symbols
        (code >= 0x2700 && code <= 0x27BF)      // Dingbats
    });
    
    if has_emoji && emoji_font.is_some() {
        let mut current_x = x;
        let mut normal_text = String::new();
        
        for c in text.chars() {
            let code = c as u32;
            let is_emoji = (code >= 0x1F600 && code <= 0x1F64F) || 
                          (code >= 0x1F300 && code <= 0x1F5FF) || 
                          (code >= 0x1F680 && code <= 0x1F6FF) || 
                          (code >= 0x1F900 && code <= 0x1F9FF) || 
                          (code >= 0x2600 && code <= 0x26FF) || 
                          (code >= 0x2700 && code <= 0x27BF);
            
            if is_emoji {
                if !normal_text.is_empty() {
                    draw_text(&normal_text, current_x, y, font_size, color);
                    current_x += measure_text(&normal_text, None, font_size as u16, 1.0).width;
                    normal_text.clear();
                }
                if emoji_font.is_some() {
                    if let Some(font) = emoji_font {
                        draw_text_ex(&c.to_string(), current_x, y, TextParams { 
                            font_size: font_size as u16, 
                            color, 
                            font: Some(font), 
                            ..Default::default() 
                        });
                        current_x += measure_text(&c.to_string(), Some(font), font_size as u16, 1.0).width;
                    }
                }
            } else {
                normal_text.push(c);
            }
        }
        
        if !normal_text.is_empty() {
            draw_text(&normal_text, current_x, y, font_size, color);
        }
    } else {
        draw_text(text, x, y, font_size, color);
    }
}

const CARD_HOVER_SCALE: f32 = 1.1;
const ENEMY_SHAKE_INTENSITY: f32 = 5.0;

pub fn draw_player_hand_with_animation(hand: &Hand, battle: &BattleState, card_textures: &CardTextureManager) {
    let base_card_width = 120.0;
    let base_card_height = 180.0;
    let card_spacing = 10.0;
    let start_y = screen_height() * 0.75;
    let total_width = (hand.cards.len() as f32) * (base_card_width + card_spacing) - card_spacing;
    let start_x = (screen_width() - total_width) / 2.0;
    
    for (i, card) in hand.cards.iter().enumerate() {
        let base_x = start_x + (i as f32) * (base_card_width + card_spacing);
        let base_y = start_y;
        let (mouse_x, mouse_y) = mouse_position();
        
        let mut scale = 1.0;
        let mut y_offset = 0.0;
        
        let base_is_hovered = mouse_x >= base_x
            && mouse_x <= base_x + base_card_width
            && mouse_y >= base_y
            && mouse_y <= base_y + base_card_height;
        
        if base_is_hovered {
            scale = CARD_HOVER_SCALE;
            y_offset = -10.0;
        }
        
        if let Some(selected_idx) = battle.selected_card_index {
            if selected_idx == i && battle.card_animation_timer > 0.0 {
                let bounce_progress = 1.0 - (battle.card_animation_timer / 0.3);
                let bounce_scale = 1.0 + (bounce_progress * (1.0 - bounce_progress) * 4.0 * 0.3);
                scale *= bounce_scale;
                y_offset -= bounce_progress * 20.0;
            }
        }
        
        let card_width = base_card_width * scale;
        let card_height = base_card_height * scale;
        let x = base_x - (card_width - base_card_width) / 2.0;
        let y = base_y - (card_height - base_card_height) / 2.0 + y_offset;
        
        let is_hovered = mouse_x >= x
            && mouse_x <= x + card_width
            && mouse_y >= y
            && mouse_y <= y + card_height;
        card_textures.draw_card_scaled(card, x, y, card_width, card_height);
        let number_text = format!("{}", i + 1);
        let text_x = x + card_width / 2.0 - measure_text(&number_text, None, 16, 1.0).width / 2.0;
        draw_text(&number_text, text_x, y - 5.0, 16.0, WHITE);
        let border_color = if is_hovered { YELLOW } else { WHITE };
        let border_width = if is_hovered { 3.0 } else { 2.0 };
        draw_rectangle_lines(x, y, card_width, card_height, border_width, border_color);
    }
}

pub fn draw_enemy_with_animation(battle: &BattleState, card_textures: &CardTextureManager, enemy_image: &str) {
    let enemy_width = 225.0; 
    let enemy_height = 300.0; 
    let screen_width = screen_width();
    let screen_height = screen_height();
    let mut x = (screen_width - enemy_width) / 2.0;
    let y = (screen_height - enemy_height) / 2.0 - 95.0;
    if battle.enemy_shake_timer > 0.0 {
        use ::rand::Rng;
        let mut rng = ::rand::thread_rng();
        let shake_x = rng.gen_range(-ENEMY_SHAKE_INTENSITY..ENEMY_SHAKE_INTENSITY);
        x += shake_x;
    }
    card_textures.draw_enemy(x, y, enemy_width, enemy_height, enemy_image);
}

pub fn draw_health_bar(x: f32, y: f32, width: f32, height: f32, current: u32, max: u32, fill_color: Color, bg_color: Color) {
    draw_rectangle(x, y, width, height, bg_color);
    let health_ratio = current as f32 / max as f32;
    let fill_width = width * health_ratio;
    draw_rectangle(x, y, fill_width, height, fill_color);
    draw_rectangle_lines(x, y, width, height, 2.0, BLACK);
    let health_text = format!("{}/{}", current, max);
    let text_size = 16.0;
    let text_dims = measure_text(&health_text, None, text_size as u16, 1.0);
    let text_x = x + (width - text_dims.width) / 2.5;
    let text_y = y + (height + text_size) / 2.5;
    draw_text(&health_text, text_x, text_y, text_size, WHITE);
}

pub fn draw_enemy_info(battle: &BattleState, margin: f32, font_size: f32, emoji_font: Option<&Font>) {
    let line_height = 25.0;
    draw_text_with_emoji(
        &format!("👾 {}", battle.enemy.name),
        margin,
        margin + line_height,
        font_size + 2.0,
        RED,
        emoji_font,
    );
    draw_health_bar(
        margin,
        margin + line_height + 10.0,
        300.0,
        18.0,
        battle.enemy.health,
        battle.enemy.max_health,
        RED,
        Color::new(0.3, 0.0, 0.0, 1.0),
    );
    let enemy_stats = format!("ATK {}   DEF {}", battle.enemy.attack, battle.enemy.defense);
    draw_text(
        &enemy_stats,
        margin,
        margin + line_height + 40.0,
        font_size - 2.0,
        LIGHTGRAY,
    );
    if let Some(poison_duration) = battle.enemy.status_effects.get(&StatusEffect::Poison) {
        let pulse = (get_time() * 4.0).sin() * 0.4 + 0.6;
        let poison_color = Color::new(0.6, 1.0, 0.6, pulse as f32);
        draw_text_with_emoji(
            &format!("☠️ ENVENENADO ({})", poison_duration),
            margin + 150.0,
            margin + line_height + 35.0,
            font_size - 2.0,
            poison_color,
            emoji_font,
        );
    }
    if let Some(burn_duration) = battle.enemy.status_effects.get(&StatusEffect::Burn) {
        let pulse = (get_time() * 4.0).sin() * 0.4 + 0.6;
        let burn_color = Color::new(1.0, 0.3, 0.3, pulse as f32);
        draw_text_with_emoji(
            &format!("🔥 QUEIMADO ({})", burn_duration),
            margin + 150.0,
            margin + line_height + 50.0,
            font_size - 2.0,
            burn_color,
            emoji_font,
        );
    }
}

pub fn draw_player_info(battle: &BattleState, font_size: f32, emoji_font: Option<&Font>) {
    let info_y = screen_height() * 0.8;
    let margin = 50.0;
    draw_text_with_emoji(&format!("🧙 {}", battle.player.name), margin, info_y, font_size + 2.0, BLUE, emoji_font);
    draw_health_bar(
        margin,
        info_y + 10.0,
        300.0,
        18.0,
        battle.player.health,
        battle.player.max_health,
        GREEN,
        Color::new(0.0, 0.3, 0.0, 1.0),
    );
    let player_stats = format!("ATK {}   DEF {}", battle.player.attack, battle.player.defense);
    draw_text(
        &player_stats,
        margin,
        info_y + 40.0,
        font_size - 2.0,
        LIGHTGRAY,
    );
    if let Some(poison_duration) = battle.player.status_effects.get(&StatusEffect::Poison) {
        let pulse = (get_time() * 4.0).sin() * 0.4 + 0.6;
        let poison_color = Color::new(0.6, 1.0, 0.6, pulse as f32);
        draw_text_with_emoji(
            &format!("☠️ POISONED ({})", poison_duration),
            margin + 150.0,
            info_y + 35.0,
            font_size - 2.0,
            poison_color,
            emoji_font,
        );
    }
    if let Some(burn_duration) = battle.player.status_effects.get(&StatusEffect::Burn) {
        let pulse = (get_time() * 4.0).sin() * 0.4 + 0.6;
        let burn_color = Color::new(1.0, 0.3, 0.3, pulse as f32);
        draw_text_with_emoji(
            &format!("🔥 BURNED ({})", burn_duration),
            margin + 150.0,
            info_y + 50.0,
            font_size - 2.0,
            burn_color,
            emoji_font,
        );
    }
}

pub fn draw_player_info_above_cards(battle: &BattleState, font_size: f32, emoji_font: Option<&Font>) {
    let info_y = screen_height() * 0.55;
    let margin = 50.0;
    
    draw_text_with_emoji(&format!("🧙 {}", battle.player.name), margin, info_y, font_size + 2.0, BLUE, emoji_font);
    
    draw_health_bar(
        margin,
        info_y + 10.0,
        300.0,
        18.0,
        battle.player.health,
        battle.player.max_health,
        GREEN,
        Color::new(0.0, 0.3, 0.0, 1.0),
    );
    
    let player_stats = format!("ATK {}   DEF {}", battle.player.attack, battle.player.defense);
    draw_text(
        &player_stats,
        margin,
        info_y + 40.0,
        font_size - 2.0,
        LIGHTGRAY,
    );
    
    if let Some(poison_duration) = battle.player.status_effects.get(&StatusEffect::Poison) {
        let pulse = (get_time() * 4.0).sin() * 0.4 + 0.6;
        let poison_color = Color::new(0.6, 1.0, 0.6, pulse as f32);
        draw_text_with_emoji(
            &format!("☠️ ENVENENADO ({})", poison_duration),
            margin + 150.0,
            info_y + 35.0,
            font_size - 2.0,
            poison_color,
            emoji_font,
        );
    }
    if let Some(burn_duration) = battle.player.status_effects.get(&StatusEffect::Burn) {
        let pulse = (get_time() * 4.0).sin() * 0.4 + 0.6;
        let burn_color = Color::new(1.0, 0.3, 0.3, pulse as f32);
        draw_text_with_emoji(
            &format!("🔥 QUEIMADO ({})", burn_duration),
            margin + 150.0,
            info_y + 50.0,
            font_size - 2.0,
            burn_color,
            emoji_font,
        );
    }
}

pub fn draw_instructions(battle: &BattleState, font_size: f32) {
    let instructions_y = screen_height() * 0.68;
    if battle.turn.player_turn() {
        if !battle.waiting_for_cooldown {
            let instructions = "Choose a card (1-5 or click)";
            let screen_width = screen_width();
            let inst_dims = measure_text(instructions, None, font_size as u16, 1.0);
            draw_text(
                instructions,
                (screen_width - inst_dims.width) / 2.0,
                instructions_y,
                font_size,
                WHITE,
            );
        } else {
            let wait_text = format!("Wait... ({:.1}s)", battle.turn_cooldown);
            let screen_width = screen_width();
            let wait_dims = measure_text(&wait_text, None, font_size as u16, 1.0);
            draw_text(
                &wait_text,
                (screen_width - wait_dims.width) / 2.0,
                instructions_y,
                font_size,
                YELLOW,
            );
        }
    } else if battle.waiting_for_cooldown {
        let wait_text = format!("Enemy's turn... ({:.1}s)", battle.turn_cooldown);
        let screen_width = screen_width();
        let wait_dims = measure_text(&wait_text, None, font_size as u16, 1.0);
        draw_text(
            &wait_text,
            (screen_width - wait_dims.width) / 2.0,
            instructions_y,
            font_size,
            ORANGE,
        );
    }
}

pub fn draw_battle_log(battle: &BattleState, emoji_font: Option<&Font>) {
    let screen_width = screen_width();
    let _screen_height = screen_height();
    
    let log_width = 350.0;
    let log_height = 200.0;
    let log_x = screen_width - log_width - 20.0;
    let log_y = 20.0;

    draw_rectangle(log_x - 5.0, log_y - 5.0, log_width + 10.0, log_height + 10.0, Color::new(0.0, 0.0, 0.0, 0.8));
    draw_rectangle(log_x, log_y, log_width, log_height, Color::new(0.1, 0.1, 0.1, 0.9));
    draw_rectangle_lines(log_x, log_y, log_width, log_height, 2.0, Color::new(0.3, 0.3, 0.3, 1.0));
    
    let title = "📜 Battle Log";
    let title_size = 16.0;
    let title_dims = measure_text(title, None, title_size as u16, 1.0);
    draw_text_with_emoji(
        title,
        log_x + (log_width - title_dims.width) / 2.0,
        log_y + 20.0,
        title_size,
        GOLD,
        emoji_font,
    );
    
    let content_y = log_y + 35.0;
    let content_height = log_height - 45.0;
    let line_height = 18.0;
    let max_visible_lines = (content_height / line_height) as usize;
    
    let start_index = battle.log_scroll_offset as usize;
    let end_index = (start_index + max_visible_lines).min(battle.battle_log.len());
    
    for (i, entry) in battle.battle_log.iter().enumerate().skip(start_index).take(max_visible_lines) {
        if i >= end_index {
            break;
        }
        
        let y_pos = content_y + ((i - start_index) as f32 * line_height);
        
        if y_pos > log_y + log_height - 10.0 {
            break;
        }
        
        let mut display_text = entry.message.clone();
        let max_chars = 45;
        if display_text.len() > max_chars {
            display_text.truncate(max_chars - 3);
            display_text.push_str("...");
        }
        
        draw_text(
            &display_text,
            log_x + 10.0,
            y_pos,
            14.0,
            entry.color,
        );
    }
    
    if battle.battle_log.len() > max_visible_lines {
        let scroll_indicator_x = log_x + log_width - 15.0;
        let scroll_indicator_y = content_y;
        let scroll_indicator_height = content_height - 10.0;
        
        draw_rectangle(scroll_indicator_x, scroll_indicator_y, 8.0, scroll_indicator_height, Color::new(0.2, 0.2, 0.2, 0.8));
        
        let scroll_ratio = if battle.battle_log.len() > max_visible_lines {
            battle.log_scroll_offset / (battle.battle_log.len() - max_visible_lines) as f32
        } else {
            0.0
        };
        let indicator_height = 20.0;
        let indicator_y = scroll_indicator_y + (scroll_indicator_height - indicator_height) * scroll_ratio;
        
        draw_rectangle(scroll_indicator_x + 1.0, indicator_y, 6.0, indicator_height, Color::new(0.6, 0.6, 0.6, 1.0));
    }
    
    if battle.battle_log.len() > max_visible_lines {
        let scroll_text = "WASD Scroll | Home/End";
        let scroll_size = 10.0;
        let _scroll_dims = measure_text(scroll_text, None, scroll_size as u16, 1.0);
        draw_text(
            scroll_text,
            log_x + 10.0,
            log_y + log_height - 15.0,
            scroll_size,
            Color::new(0.7, 0.7, 0.7, 1.0),
        );
    }
}