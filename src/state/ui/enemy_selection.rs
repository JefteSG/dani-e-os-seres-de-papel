use macroquad::prelude::*;
use crate::state::game_state::EnemyInfo;
use crate::state::ui::components::draw_text_with_emoji;
use crate::player::Player;

pub fn draw_enemy_selection(enemies: &Vec<EnemyInfo>, selected_index: usize, emoji_font: Option<&Font>, player: Option<&Player>, show_instructions: bool) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    let title = "🏆 ESCOLHA SEU OPONENTE 🏆";
    let title_size = 36.0;
    let title_dims = measure_text(title, None, title_size as u16, 1.0);
    draw_text_with_emoji(
        title,
        (screen_width - title_dims.width) / 2.0,
        screen_height * 0.05,
        title_size,
        GOLD,
        emoji_font,
    );

    if let Some(player) = player {
        let player_info = format!("👤 {} - Nível {}", player.name, player.level);
        
        let info_size = 18.0;   
        let stats_size = 16.0;
        
        let info_dims = measure_text(&player_info, None, info_size as u16, 1.0);
        let info_x = (screen_width - info_dims.width) / 2.0;
        draw_text_with_emoji(&player_info, info_x, screen_height * 0.12, info_size, WHITE, emoji_font);
        
        let stats_text = format!("ATK: {}   DEF: {}   HP: {}/{}", 
            player.attack, player.defense, player.health, player.max_health);
        let stats_dims = measure_text(&stats_text, None, stats_size as u16, 1.0);
        let stats_x = (screen_width - stats_dims.width) / 2.0;
        
        let emoji_size = stats_size;
        let emoji_spacing = 20.0;
        
        if let Some(font) = emoji_font {
            draw_text_ex("⚔️", stats_x - emoji_spacing, screen_height * 0.14, TextParams { 
                font_size: emoji_size as u16, 
                color: LIME, 
                font: Some(font), 
                ..Default::default() 
            });
        }
        
        let atk_text = format!("ATK: {}", player.attack);
        draw_text(&atk_text, stats_x, screen_height * 0.14, stats_size, LIME);
        
        let separator_x = stats_x + measure_text(&atk_text, None, stats_size as u16, 1.0).width + 10.0;
        
        if let Some(font) = emoji_font {
            draw_text_ex("🛡️", separator_x + 5.0, screen_height * 0.14, TextParams { 
                font_size: emoji_size as u16, 
                color: LIME, 
                font: Some(font), 
                ..Default::default() 
            });
        }
        
        let def_text = format!("DEF: {}", player.defense);
        let def_x = separator_x + 5.0 + emoji_spacing;
        draw_text(&def_text, def_x, screen_height * 0.14, stats_size, LIME);
        
        let separator2_x = def_x + measure_text(&def_text, None, stats_size as u16, 1.0).width + 10.0;
        
        if let Some(font) = emoji_font {
            draw_text_ex("💚", separator2_x + 5.0, screen_height * 0.14, TextParams { 
                font_size: emoji_size as u16, 
                color: LIME, 
                font: Some(font), 
                ..Default::default() 
            });
        }
        
        let hp_text = format!("HP: {}/{}", player.health, player.max_health);
        let hp_x = separator2_x + 5.0 + emoji_spacing;
        draw_text(&hp_text, hp_x, screen_height * 0.14, stats_size, LIME);
    }

    if show_instructions {
        let instructions = "Use A/D para navegar. Enter para selecionar. I para informações.";
        let inst_size = 18.0;
        let inst_dims = measure_text(instructions, None, inst_size as u16, 1.0);
        draw_text(
            instructions,
            (screen_width - inst_dims.width) / 2.0,
            screen_height * 0.22,
            inst_size,
            LIGHTGRAY,
        );

        let controls = "ESC: Menu Principal | Q: Sair do Jogo | Shift+R: Resetar Progresso";
        let controls_size = 14.0;
        let controls_dims = measure_text(controls, None, controls_size as u16, 1.0);
        draw_text(
            controls,
            (screen_width - controls_dims.width) / 2.0,
            screen_height * 0.25,
            controls_size,
            DARKGRAY,
        );
    } else {
        let help_text = "ℹ️ Pressione I para mostrar informações";
        let help_size = 16.0;
        let help_dims = measure_text(help_text, None, help_size as u16, 1.0);
        let help_x = (screen_width - help_dims.width) / 2.0;
        draw_text_with_emoji(help_text, help_x, screen_height * 0.22, help_size, LIGHTGRAY, emoji_font);
    }

    let card_width = 200.0;
    let card_height = 280.0;
    let card_spacing = 30.0;
    let total_width = (enemies.len() as f32) * (card_width + card_spacing) - card_spacing;
    let start_x = (screen_width - total_width) / 2.0;
    let start_y = screen_height * 0.35;

    for (i, enemy) in enemies.iter().enumerate() {
        let x = start_x + (i as f32) * (card_width + card_spacing);
        let y = start_y;

        let (bg_color, border_color, text_color) = if !enemy.is_unlocked {
            (Color::new(0.2, 0.2, 0.2, 0.8), GRAY, DARKGRAY)
        } else if enemy.is_defeated {
            (Color::new(0.1, 0.4, 0.1, 0.9), GREEN, LIME)
        } else if i == selected_index {
            (Color::new(0.3, 0.3, 0.6, 0.9), YELLOW, WHITE)
        } else {
            (Color::new(0.2, 0.2, 0.4, 0.9), WHITE, LIGHTGRAY)
        };

        draw_rectangle(x, y, card_width, card_height, bg_color);
        
        let border_width = if i == selected_index { 4.0 } else { 2.0 };
        draw_rectangle_lines(x, y, card_width, card_height, border_width, border_color);

        let img_height = card_height * 0.6;
        let img_y = y + 20.0;
        draw_rectangle(x + 20.0, img_y, card_width - 40.0, img_height - 40.0, Color::new(0.1, 0.1, 0.1, 0.8));
        draw_rectangle_lines(x + 20.0, img_y, card_width - 40.0, img_height - 40.0, 1.0, GRAY);

        let default_emoji = "👹".to_string();
        let emoji = enemy.emoji.as_ref().unwrap_or(&default_emoji);
        let emoji_size = 48.0;
        let emoji_x = x + (card_width - measure_text(emoji, None, emoji_size as u16, 1.0).width) / 2.0;
        let emoji_y = img_y + (img_height - 40.0) / 2.0;
        draw_text_with_emoji(emoji, emoji_x, emoji_y, emoji_size, WHITE, emoji_font);

        let name_size = 20.0;
        let name_y = y + img_height + 10.0;
        let name_dims = measure_text(&enemy.name, None, name_size as u16, 1.0);
        let name_x = x + (card_width - name_dims.width) / 2.0;
        draw_text(&enemy.name, name_x, name_y, name_size, text_color);

        let stats_text = format!("HP: {} | ATK: {} | DEF: {}", enemy.health, enemy.attack, enemy.defense);
        let stats_size = 14.0;
        let stats_dims = measure_text(&stats_text, None, stats_size as u16, 1.0);
        let stats_x = x + (card_width - stats_dims.width) / 2.0;
        let stats_y = name_y + 25.0;
        draw_text(&stats_text, stats_x, stats_y, stats_size, text_color);

        let level_text = format!("Nível {}", enemy.level);
        let level_size = 16.0;
        let level_dims = measure_text(&level_text, None, level_size as u16, 1.0);
        let level_x = x + (card_width - level_dims.width) / 2.0;
        let level_y = stats_y + 20.0;
        draw_text(&level_text, level_x, level_y, level_size, ORANGE);

        let status_text = if !enemy.is_unlocked {
            "🔒 BLOQUEADO"
        } else if enemy.is_defeated {
            "✅ DERROTADO"
        } else {
            "⚡ DISPONÍVEL"
        };
        let status_size = 16.0;
        let status_color = if !enemy.is_unlocked {
            RED
        } else if enemy.is_defeated {
            GREEN
        } else {
            YELLOW
        };
        let status_dims = measure_text(status_text, None, status_size as u16, 1.0);
        let status_x = x + (card_width - status_dims.width) / 2.0;
        let status_y = level_y + 25.0;
        draw_text_with_emoji(status_text, status_x, status_y, status_size, status_color, emoji_font);

        if i == selected_index {
            let glow_size = 8.0;
            for offset in 1..=3 {
                let alpha = 0.3 - (offset as f32 * 0.1);
                let glow_color = Color::new(1.0, 1.0, 0.0, alpha);
                draw_rectangle_lines(
                    x - (offset as f32 * glow_size / 3.0),
                    y - (offset as f32 * glow_size / 3.0),
                    card_width + (offset as f32 * glow_size * 2.0 / 3.0),
                    card_height + (offset as f32 * glow_size * 2.0 / 3.0),
                    2.0,
                    glow_color,
                );
            }
        }
    }

    if show_instructions {
        let progress_title = "🎯 SISTEMA DE PROGRESSÃO 🎯";
        let progress_title_size = 20.0;
        let progress_title_dims = measure_text(progress_title, None, progress_title_size as u16, 1.0);
        draw_text_with_emoji(
            progress_title,
            (screen_width - progress_title_dims.width) / 2.0,
            screen_height * 0.75,
            progress_title_size,
            GOLD,
            emoji_font,
        );

        let progress_text = "Derrote inimigos para ganhar experiência e subir de nível!";
        let progress_size = 16.0;
        let progress_dims = measure_text(progress_text, None, progress_size as u16, 1.0);
        draw_text(
            progress_text,
            (screen_width - progress_dims.width) / 2.0,
            screen_height * 0.78,
            progress_size,
            ORANGE,
        );

        let benefits_text = "Cada nível aumenta sua vida máxima: +25 HP (ataque e defesa vêm das cartas)";
        let benefits_size = 14.0;
        let benefits_dims = measure_text(benefits_text, None, benefits_size as u16, 1.0);
        draw_text(
            benefits_text,
            (screen_width - benefits_dims.width) / 2.0,
            screen_height * 0.81,
            benefits_size,
            LIME,
        );

        let unlock_text = "Derrote os inimigos em ordem para desbloquear os próximos!";
        let unlock_size = 14.0;
        let unlock_dims = measure_text(unlock_text, None, unlock_size as u16, 1.0);
        draw_text(
            unlock_text,
            (screen_width - unlock_dims.width) / 2.0,
            screen_height * 0.84,
            unlock_size,
            SKYBLUE,
        );
    }

    if selected_index < enemies.len() {
        let selected_enemy = &enemies[selected_index];
        if selected_enemy.is_unlocked {
            let detail_text = if selected_enemy.is_defeated {
                format!("Você já derrotou {}! Pode enfrentar novamente para treinar.", selected_enemy.name)
            } else {
                format!("Pronto para enfrentar {}? Esta será uma batalha épica!", selected_enemy.name)
            };
            let detail_size = 18.0;
            let detail_dims = measure_text(&detail_text, None, detail_size as u16, 1.0);
            draw_text(
                &detail_text,
                (screen_width - detail_dims.width) / 2.0,
                screen_height * 0.92,
                detail_size,
                LIME,
            );
        } else {
            let locked_text = format!("Derrote {} primeiro para desbloquear este oponente!", 
                if selected_index > 0 { &enemies[selected_index - 1].name } else { "o inimigo anterior" });
            let locked_size = 18.0;
            let locked_dims = measure_text(&locked_text, None, locked_size as u16, 1.0);
            draw_text(
                &locked_text,
                (screen_width - locked_dims.width) / 2.0,
                screen_height * 0.92,
                locked_size,
                RED,
            );
        }
    }
}

pub fn get_clicked_enemy_index(mouse_x: f32, mouse_y: f32, enemies: &Vec<EnemyInfo>) -> Option<usize> {
    let screen_width = screen_width();
    let screen_height = screen_height();
    
    let card_width = 200.0;
    let card_height = 280.0;
    let card_spacing = 30.0;
    let total_width = (enemies.len() as f32) * (card_width + card_spacing) - card_spacing;
    let start_x = (screen_width - total_width) / 2.0;
    let start_y = screen_height * 0.28;

    for (i, _enemy) in enemies.iter().enumerate() {
        let x = start_x + (i as f32) * (card_width + card_spacing);
        let y = start_y;

        if mouse_x >= x && mouse_x <= x + card_width && mouse_y >= y && mouse_y <= y + card_height {
            return Some(i);
        }
    }

    None
}