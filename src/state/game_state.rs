use crate::state::battle_state::BattleState;
use crate::state::menu::MenuSelection;
use crate::deck::{CardTextureManager, CardType, Deck, Hand};
use crate::enemy::Enemy;
use crate::player::Player;
use crate::effects::StatusEffect;
use crate::entity::Entity;
use crate::gameturn::GameTurn;
use crate::state::damage_particle::DamageParticle;
use crate::config::config::*;
use macroquad::prelude::*;

use ::rand::thread_rng;
use ::rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub enum AppState {
    Menu,
    EnemySelection,
    Battle(BattleState),
    GameOver,
    SoundSettings,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EnemyInfo {
    pub id: u32,
    pub name: String,
    pub base_health: u32,     
    pub base_attack: u32,     
    pub base_defense: u32,    
    pub health: u32,          
    pub max_health: u32,      
    pub attack: u32,          
    pub defense: u32,         
    pub level: u32,           
    pub times_defeated: u32,  
    pub is_unlocked: bool,
    pub is_defeated: bool,
    pub emoji: Option<String>,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub enemies: Vec<EnemyInfo>,
    pub persistent_player: Option<PlayerSaveData>,
    pub player_name: String,
    pub sound_settings: SoundSettings,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerSaveData {
    pub name: String,
    pub health: u32,
    pub max_health: u32,
    pub attack: u32,
    pub defense: u32,
    pub level: u32,
    pub experience: u32,
    pub experience_to_next_level: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SoundSettings {
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub music_enabled: bool,
    pub sfx_enabled: bool,
}

pub struct GameState {
    pub app_state: AppState,
    pub selection: MenuSelection,
    pub winner: Option<String>,
    pub card_textures: CardTextureManager,
    pub enemies: Vec<EnemyInfo>,
    pub selected_enemy_index: usize,
    pub persistent_player: Option<Player>, 
    pub player_name: String,              
    pub is_editing_name: bool,            
    pub emoji_font: Option<Font>,         
    pub window_too_small: bool,           
    pub show_player_info: bool,           
    pub show_instructions: bool,          
    pub music_volume: f32,                
    pub sfx_volume: f32,                  
    pub music_enabled: bool,              
    pub sfx_enabled: bool,                
    pub music_started: bool,              

}

fn get_default_enemies() -> Vec<EnemyInfo> {
    vec![
        EnemyInfo {
            id: 1,
            name: "Esqueleto Bombado".to_string(),
            base_health: 80,
            base_attack: 8,
            base_defense: 5,
            health: 80,
            max_health: 80,
            attack: 8,
            defense: 5,
            level: 1,
            times_defeated: 0,
            is_unlocked: true, 
            is_defeated: false,
            emoji: Some("💀".to_string()),
            image: Some("assets/enemies/skeleton.png".to_string()),
        },
        EnemyInfo {
            id: 2,
            name: "Zumbi Influencer".to_string(),
            base_health: 140,
            base_attack: 35,
            base_defense: 16,
            health: 140,
            max_health: 140,
            attack: 25,
            defense: 16,
            level: 1,
            times_defeated: 0,
            is_unlocked: false, 
            is_defeated: false,
            emoji: Some("🧟".to_string()),
            image: Some("assets/enemies/zombie.png".to_string()),
        },
        EnemyInfo {
            id: 3,
            name: "Dragoberto".to_string(),
            base_health: 220,
            base_attack: 32,
            base_defense: 30,
            health: 220,
            max_health: 220,
            attack: 32,
            defense: 30,
            level: 3,
            times_defeated: 0,
            is_unlocked: false, 
            is_defeated: false,
            emoji: Some("🐲".to_string()),
            image: Some("assets/enemies/dragon.png".to_string()),
        },
        EnemyInfo {
            id: 4,
            name: "Psicopapão".to_string(),
            base_health: 520,
            base_attack: 66,
            base_defense: 12,
            health: 520,
            max_health: 520,
            attack: 66,
            defense: 12,
            level: 1,
            times_defeated: 0,
            is_unlocked: false, 
            is_defeated: false,
            emoji: Some("👾".to_string()),
            image: Some("assets/enemies/devourer.png".to_string()),
        },
    ]
}

impl GameState {
    pub fn new() -> Self {
        let enemies = get_default_enemies();

        let mut game_state = Self {
            app_state: AppState::Menu,
            selection: MenuSelection::Start,
            winner: None,
            card_textures: CardTextureManager::new(),
            enemies,
            selected_enemy_index: 0,
            persistent_player: None,
            player_name: "Player".to_string(),
            is_editing_name: false,
            emoji_font: None,
            window_too_small: false,
            show_player_info: true,
            show_instructions: false,
            music_volume: DEFAULT_MUSIC_VOLUME,
            sfx_volume: DEFAULT_SFX_VOLUME,
            music_enabled: true,
            sfx_enabled: true,
            music_started: false,
        };
        
        game_state.load_progress();
        
        game_state
    }

    pub async fn load_card_textures(&mut self) {
        self.card_textures.load_all_textures().await;
    }

    pub fn update(&mut self) {
        let current_width = screen_width();
        let current_height = screen_height();
        
        self.window_too_small = current_width < MIN_WINDOW_WIDTH || current_height < MIN_WINDOW_HEIGHT;
        
        if self.window_too_small {
            return;
        }
        
        let mut clicked_card_index = None;
        if let AppState::Battle(battle) = &self.app_state {
            if battle.turn.player_turn() && !battle.waiting_for_cooldown {
                let (mouse_x, mouse_y) = mouse_position();
                clicked_card_index = self.get_clicked_card_index(mouse_x, mouse_y, &battle.player.hand);
            }
        }
        if !self.music_started && self.music_enabled {
            self.play_music_with_current_settings();
            self.music_started = true;
        }

        match &mut self.app_state {
            AppState::Menu => {
                if self.is_editing_name {
                    if is_key_pressed(KeyCode::Enter) {
                        self.is_editing_name = false;
                        if self.player_name.trim().is_empty() {
                            self.player_name = "Jogador".to_string();
                        }
                        self.save_progress();
                    }
                    if is_key_pressed(KeyCode::Escape) {
                        self.is_editing_name = false;
                        self.player_name = "Player".to_string();
                    }
                    if is_key_pressed(KeyCode::Backspace) && !self.player_name.is_empty() {
                        self.player_name.pop();
                    }
                    
                    for key_code in [
                        KeyCode::A, KeyCode::B, KeyCode::C, KeyCode::D, KeyCode::E,
                        KeyCode::F, KeyCode::G, KeyCode::H, KeyCode::I, KeyCode::J,
                        KeyCode::K, KeyCode::L, KeyCode::M, KeyCode::N, KeyCode::O,
                        KeyCode::P, KeyCode::Q, KeyCode::R, KeyCode::S, KeyCode::T,
                        KeyCode::U, KeyCode::V, KeyCode::W, KeyCode::X, KeyCode::Y,
                        KeyCode::Z, KeyCode::Space,
                        KeyCode::Key0, KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4,
                        KeyCode::Key5, KeyCode::Key6, KeyCode::Key7, KeyCode::Key8, KeyCode::Key9,
                    ] {
                        if is_key_pressed(key_code) && self.player_name.len() < MAX_PLAYER_NAME_LENGTH {
                            let ch = match key_code {
                                KeyCode::A => 'A', KeyCode::B => 'B', KeyCode::C => 'C', KeyCode::D => 'D', KeyCode::E => 'E',
                                KeyCode::F => 'F', KeyCode::G => 'G', KeyCode::H => 'H', KeyCode::I => 'I', KeyCode::J => 'J',
                                KeyCode::K => 'K', KeyCode::L => 'L', KeyCode::M => 'M', KeyCode::N => 'N', KeyCode::O => 'O',
                                KeyCode::P => 'P', KeyCode::Q => 'Q', KeyCode::R => 'R', KeyCode::S => 'S', KeyCode::T => 'T',
                                KeyCode::U => 'U', KeyCode::V => 'V', KeyCode::W => 'W', KeyCode::X => 'X', KeyCode::Y => 'Y',
                                KeyCode::Z => 'Z', KeyCode::Space => ' ',
                                KeyCode::Key0 => '0', KeyCode::Key1 => '1', KeyCode::Key2 => '2', KeyCode::Key3 => '3', KeyCode::Key4 => '4',
                                KeyCode::Key5 => '5', KeyCode::Key6 => '6', KeyCode::Key7 => '7', KeyCode::Key8 => '8', KeyCode::Key9 => '9',
                                _ => continue,
                            };
                            self.player_name.push(ch);
                            
                        }
                    }
                } else {
                    if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                        self.selection = match self.selection {
                            MenuSelection::Start => MenuSelection::Start,
                            MenuSelection::SoundSettings => MenuSelection::Start,
                            MenuSelection::Quit => MenuSelection::SoundSettings,
                        };
                    }
                    if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                        self.selection = match self.selection {
                            MenuSelection::Start => MenuSelection::SoundSettings,
                            MenuSelection::SoundSettings => MenuSelection::Quit,
                            MenuSelection::Quit => MenuSelection::Quit,
                        };
                    }
                    if is_key_pressed(KeyCode::Tab) {
                        self.is_editing_name = true;
                    }
                    if is_key_pressed(KeyCode::Enter) {
                        self.execute_menu_selection();
                    }
                }

                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mouse_x, mouse_y) = mouse_position();
                    
                    if crate::state::ui::menu::is_name_field_clicked(mouse_x, mouse_y) {
                        self.is_editing_name = !self.is_editing_name;
                    }
                    else if !self.is_editing_name {
                        if let Some(clicked_option) = crate::state::ui::menu::get_clicked_menu_option(mouse_x, mouse_y) {
                            self.selection = clicked_option;
                            self.execute_menu_selection();
                        }
                    }
                }
                
                if !self.is_editing_name {
                    let (mouse_x, mouse_y) = mouse_position();
                    if let Some(hovered_option) = crate::state::ui::menu::get_clicked_menu_option(mouse_x, mouse_y) {
                        self.selection = hovered_option;
                    }
                }
            }
            AppState::EnemySelection => {
                if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
                    if self.selected_enemy_index > 0 {
                        self.selected_enemy_index -= 1;
                    }
                }
                if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
                    if self.selected_enemy_index < self.enemies.len() - 1 {
                        self.selected_enemy_index += 1;
                    }
                }
                
                if is_key_pressed(KeyCode::I) {
                    self.show_instructions = !self.show_instructions;
                }
                
                if is_key_pressed(KeyCode::Enter) {
                    self.start_battle_with_selected_enemy();
                }
                
                if is_key_pressed(KeyCode::Escape) {
                    self.app_state = AppState::Menu;
                }
                
                if is_key_pressed(KeyCode::Q) {
                    std::process::exit(0);
                }
                
                if is_key_pressed(KeyCode::R) && (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)) {
                    self.reset_progress();
                }
                
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mouse_x, mouse_y) = mouse_position();
                    if let Some(clicked_enemy) = self.get_clicked_enemy_index(mouse_x, mouse_y) {
                        self.selected_enemy_index = clicked_enemy;
                        self.start_battle_with_selected_enemy();
                    }
                }
            }
            AppState::Battle(battle) => {
                
                let max_visible_lines = MAX_BATTLE_LOG_LINES;
                if battle.battle_log.len() > max_visible_lines {
                    if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                        battle.log_scroll_offset = (battle.log_scroll_offset - 1.0).max(0.0);
                    }
                    if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                        let max_scroll = (battle.battle_log.len() - max_visible_lines) as f32;
                        battle.log_scroll_offset = (battle.log_scroll_offset + 1.0).min(max_scroll);
                    }
                    if is_key_pressed(KeyCode::Home) {
                        battle.log_scroll_offset = 0.0;
                    }
                    if is_key_pressed(KeyCode::End) {
                        battle.log_scroll_offset = (battle.battle_log.len() - max_visible_lines) as f32;
                    }
                } else {
                    battle.log_scroll_offset = 0.0;
                }
                if battle.turn.turn_over() {
                    battle.current_message = "⏰ TIME'S UP! Turns have ended!".to_string();
                    battle.add_battle_end_log("Empate");
                    self.winner = Some("Empate".to_string());
                    self.on_battle_end("Playable turns have ended!");
                    self.app_state = AppState::GameOver;
                    return;
                }
                if battle.turn.player_turn() {
                    if battle.is_final_blow {
                        let frame_time = get_frame_time();
                        battle.slow_motion_timer -= frame_time;
                        
                        for particle in &mut battle.damage_particles {
                            particle.update(frame_time);
                        }
                        battle.damage_particles.retain(|p| p.is_alive());
                        
                        if battle.slow_motion_timer <= 0.0 || battle.slow_motion_timer > 2.0 {
                            battle.slow_motion_timer = 0.0;
                            battle.is_final_blow = false;
                            
                            if battle.enemy.health <= 0 {
                                battle.add_battle_end_log("Jogador");
                                self.winner = Some("Jogador".to_string());
                                self.on_battle_end("Player");
                                self.app_state = AppState::GameOver;
                                return;
                            } else {
                                battle.add_battle_end_log("Inimigo");
                                self.winner = Some("Inimigo".to_string());
                                self.on_battle_end("Enemy");
                                self.app_state = AppState::GameOver;
                                return;
                            }
                        }
                        return;
                    }
                    
                    if battle.player.hand.cards.is_empty() && battle.deck.cards.is_empty() {
                        battle.current_message = "🃏 Sem cartas! Passando turno...".to_string();
                        battle.turn_cooldown = PLAYER_TURN_COOLDOWN;
                        battle.waiting_for_cooldown = true;
                    } else if !battle.waiting_for_cooldown {
                        let mut card_played = None;
                        for i in 0..battle.player.hand.cards.len().min(5) {
                            let key = match i {
                                0 => KeyCode::Key1,
                                1 => KeyCode::Key2,
                                2 => KeyCode::Key3,
                                3 => KeyCode::Key4,
                                4 => KeyCode::Key5,
                                _ => continue,
                            };
                            if is_key_pressed(key) {
                                if let Some(card) = battle.player.hand.discard(i) {
                                    battle.selected_card_index = Some(i);
                                    card_played = Some(card);
                                    break;
                                }
                            }
                        }
                        if is_mouse_button_pressed(MouseButton::Left) {
                            if let Some(index) = clicked_card_index {
                                if let Some(card) = battle.player.hand.discard(index) {
                                    battle.selected_card_index = Some(index);
                                    card_played = Some(card);
                                }
                            }
                        }
                        if let Some(card) = card_played {
                            self.card_textures.play_card_use_sound();
                            battle.card_animation_timer = 0.3;
                            
                            battle.add_card_log("Player", &card.name);
                            
                            match card.card_type {
                                CardType::AttackBasic(damage) => {
                                    battle.player.attack_up(damage);
                                    battle.current_message = format!(
                                        "Você usou {} e aumentou o ataque em {}!",
                                        card.name, damage
                                    );
                                }
                                CardType::AttackStrong(damage) => {
                                    battle.player.attack_up(damage);
                                    battle.current_message =
                                        format!("Você usou {} e aumentou o ataque em {}!",
                                        card.name, damage
                                    );
                                }
                                CardType::Defense(defense) => {
                                    battle.player.defense_up(defense);
                                    battle.current_message = format!(
                                        "Você usou {} e aumentou a defesa em {}!",
                                        card.name, defense
                                    );
                                }
                                CardType::Poison(_) => {
                                    battle.enemy.status_effect(StatusEffect::Poison, STATUS_EFFECT_DURATION_POISON);
                                    battle.current_message =
                                        format!("Você usou {} e envenenou o inimigo!", card.name);
                                }
                                CardType::Heal(heal_percent) => {
                                    let heal_amount = (battle.player.max_health as f32 * heal_percent) as u32;
                                    battle.player.heal(heal_amount);
                                    battle.current_message =
                                        format!("Você usou {} e se curou em {}!", card.name, heal_amount);
                                    battle.add_heal_log("Jogador", heal_amount);
                                        
                                }
                                CardType::Burn(_) => {
                                    battle.enemy.status_effect(StatusEffect::Burn, STATUS_EFFECT_DURATION_BURN);
                                    battle.current_message =
                                        format!("Você usou {} e queimou! 🔥", card.name);
                                }
                            }
                            if !battle.deck.cards.is_empty() {
                                battle.player.hand.draw_from_deck(&mut battle.deck, 1);
                            }
                            let damage_dealt = battle.player.attack;
                            let enemy_health_before = battle.enemy.health;
                            battle.enemy.damage(damage_dealt);
                            let actual_damage = enemy_health_before - battle.enemy.health;
                            
                            let enemy_name = battle.enemy.name.clone();
                            battle.add_damage_log("Player", &enemy_name, damage_dealt, actual_damage);
                            
                            battle.enemy.apply_status_effects();
                            
                            if battle.enemy.health <= 0 {
                                battle.is_final_blow = true;
                                battle.slow_motion_timer = 1.0;
                                battle.current_message = "💀 GOLPE FINAL! 💀".to_string();
                                battle.waiting_for_cooldown = true;
                            } else {
                                battle.enemy_shake_timer = ENEMY_SHAKE_DURATION;
                                let enemy_x = screen_width() / 2.0;
                                let enemy_y = screen_height() / 2.0 - 50.0;
                                battle.damage_particles.push(DamageParticle::new(
                                    enemy_x,
                                    enemy_y,
                                    damage_dealt,
                                ));
                                battle.turn_cooldown = PLAYER_TURN_COOLDOWN;
                                battle.waiting_for_cooldown = true;
                            }
                        }
                    } else {
                        battle.turn_cooldown -= get_frame_time();
                        battle.enemy_shake_timer =
                            (battle.enemy_shake_timer - get_frame_time()).max(0.0);
                        battle.card_animation_timer =
                            (battle.card_animation_timer - get_frame_time()).max(0.0);
                        for particle in &mut battle.damage_particles {
                            particle.update(get_frame_time());
                        }
                        battle.damage_particles.retain(|p| p.is_alive());
                        if battle.turn_cooldown <= 0.0 {
                            battle.waiting_for_cooldown = false;
                            battle.turn.next_turn();
                            
                            let enemy_name = battle.enemy.name.clone();
                            battle.add_turn_log(&enemy_name);
                        }
                    }
                } else {
                    if battle.is_final_blow {
                        let frame_time = get_frame_time();
                        battle.slow_motion_timer -= frame_time;
                        
                        for particle in &mut battle.damage_particles {
                            particle.update(frame_time);
                        }
                        battle.damage_particles.retain(|p| p.is_alive());
                        
                        if battle.slow_motion_timer <= 0.0 || battle.slow_motion_timer > 2.0 {
                            battle.slow_motion_timer = 0.0;
                            battle.is_final_blow = false;
                            
                            if battle.enemy.health <= 0 {
                                battle.add_battle_end_log("Player");
                                self.winner = Some("Player".to_string());
                                self.on_battle_end("Player");
                                self.app_state = AppState::GameOver;
                                return;
                            } else {
                                battle.add_battle_end_log("Enemy");
                                self.winner = Some("Enemy".to_string());
                                self.on_battle_end("Enemy");
                                self.app_state = AppState::GameOver;
                                return;
                            }
                        }
                        return;
                    }
                    
                    if !battle.waiting_for_cooldown {
                        if let Some(card) = battle.deck.cards.choose(&mut thread_rng()).cloned() {
                            let enemy_name = battle.enemy.name.to_lowercase();
                            let sound_key = if enemy_name.contains("esqueleto") || enemy_name.contains("skeleton") {
                                "skeleton"
                            } else if enemy_name.contains("zumbi") || enemy_name.contains("zombie") {
                                "zombie"
                            } else if enemy_name.contains("drag") || enemy_name.contains("dragon") {
                                "dragon"
                            } else if enemy_name.contains("devourer") || enemy_name.contains("psicopapão") {
                                "devourer"
                            } else {
                                "skeleton"
                            };
                            self.card_textures.play_enemy_sound(sound_key);
                            match card.card_type {
                                CardType::AttackBasic(attack) => {
                                    battle.enemy.attack_up(attack);
                                    battle.current_message =
                                        format!("Enemy increased attack by {}!", attack);
                                }
                                CardType::AttackStrong(attack) => {
                                    battle.enemy.attack_up(attack);
                                    battle.current_message =
                                        format!("Enemy increased attack by {}!", attack);
                                }
                                CardType::Defense(defense) => {
                                    battle.enemy.defense_up(defense);
                                    battle.current_message =
                                        format!("Enemy increased defense by {}!", defense);
                                }
                                CardType::Poison(_) => {
                                    battle.player.status_effect(StatusEffect::Poison, STATUS_EFFECT_DURATION_POISON);
                                    battle.current_message = "Enemy applied poison!".to_string();
                                }
                                CardType::Heal(heal_percent) => {   
                                    let heal_amount = (battle.enemy.max_health as f32 * heal_percent) as u32;
                                    battle.enemy.heal(heal_amount);
                                    battle.current_message = format!("Enemy healed {}!", heal_amount);
                                }
                                CardType::Burn(_) => {
                                    battle.player.status_effect(StatusEffect::Burn, STATUS_EFFECT_DURATION_BURN);
                                    battle.current_message =
                                        format!("Enemy burned! 🔥");
                                }
                            }
                            let damage_dealt = battle.enemy.attack;
                            let player_health_before = battle.player.health;
                            battle.player.damage(damage_dealt);
                            let actual_damage = player_health_before - battle.player.health;
                            
                            let enemy_name = battle.enemy.name.clone();
                            battle.add_damage_log(&enemy_name, "Player", damage_dealt, actual_damage);
                            
                            battle.player.apply_status_effects();
                            
                            if battle.player.health <= 0 {
                                battle.is_final_blow = true;
                                battle.slow_motion_timer = 1.0;
                                battle.current_message = "💀 GOLPE FINAL! 💀".to_string();
                                battle.waiting_for_cooldown = true;
                            } else {
                                let player_x = screen_width() / 2.0;
                                let player_y = screen_height() * 0.8;
                                battle.damage_particles.push(DamageParticle::new(
                                    player_x,
                                    player_y,
                                    damage_dealt,
                                ));
                                battle.turn_cooldown = ENEMY_TURN_COOLDOWN;
                                battle.waiting_for_cooldown = true;
                            }
                        } else {
                            battle.current_message =
                                "🃏 Enemy deck empty! Turn skipped.".to_string();
                            battle.turn_cooldown = ENEMY_TURN_COOLDOWN;
                            battle.waiting_for_cooldown = true;
                        }
                    } else {

                        battle.turn_cooldown -= get_frame_time();
                        battle.enemy_shake_timer =
                            (battle.enemy_shake_timer - get_frame_time()).max(0.0);
                        battle.card_animation_timer =
                            (battle.card_animation_timer - get_frame_time()).max(0.0);
                        for particle in &mut battle.damage_particles {
                            particle.update(get_frame_time());
                        }
                        battle.damage_particles.retain(|p| p.is_alive());
                        if battle.turn_cooldown <= 0.0 {
                            battle.waiting_for_cooldown = false;
                            battle.turn.next_turn();
                            
                            battle.add_turn_log("Player");
                        }
                    }
                }
            }

            AppState::SoundSettings => {
                if is_key_pressed(KeyCode::Space) {
                    self.music_enabled = !self.music_enabled;
                    if self.music_started {
                        self.play_music_with_current_settings();
                    }
                }
                if is_key_pressed(KeyCode::S) {
                    self.sfx_enabled = !self.sfx_enabled;
                }
                
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mouse_x, mouse_y) = mouse_position();
                    let screen_width = screen_width();
                    let screen_height = screen_height();
                    let bar_width = 300.0;
                    let bar_height = 20.0;
                    let bar_x = (screen_width - bar_width) / 2.0;
                    let music_bar_y = screen_height * 0.4;
                    let sfx_bar_y = screen_height * 0.6;
                    
                    if mouse_x >= bar_x && mouse_x <= bar_x + bar_width &&
                       mouse_y >= music_bar_y && mouse_y <= music_bar_y + bar_height {
                        let relative_x = (mouse_x - bar_x) / bar_width;
                        self.music_volume = relative_x.max(0.0).min(1.0);
                        if self.music_enabled && self.music_started {
                            self.play_music_with_current_settings();
                        }
                    }
                    
                    if mouse_x >= bar_x && mouse_x <= bar_x + bar_width &&
                       mouse_y >= sfx_bar_y && mouse_y <= sfx_bar_y + bar_height {
                        let relative_x = (mouse_x - bar_x) / bar_width;
                        self.sfx_volume = relative_x.max(0.0).min(1.0);
                    }
                }
                
                if is_key_pressed(KeyCode::Escape) {
                    self.app_state = AppState::Menu;
                    self.save_progress();
                }
            }
            AppState::GameOver => {
                                    if is_key_pressed(KeyCode::Escape) {
                        self.app_state = AppState::EnemySelection;
                        self.winner = None;
                    }
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mouse_x, mouse_y) = mouse_position();
                    let screen_width = screen_width();
                    let screen_height = screen_height();
                    let panel_width = 400.0;
                    let panel_height = 300.0;
                    let panel_x = (screen_width - panel_width) / 2.0;
                    let panel_y = (screen_height - panel_height) / 2.0;
                    let button_width = 200.0;
                    let button_height = 40.0;
                    let button_x = panel_x + (panel_width - button_width) / 2.0;
                    let button_y = panel_y + 200.0;
                    let clicked_button = mouse_x >= button_x
                        && mouse_x <= button_x + button_width
                        && mouse_y >= button_y
                        && mouse_y <= button_y + button_height;
                    if clicked_button {
                        self.app_state = AppState::EnemySelection;
                        self.winner = None;
                    }
                }
            }
        }
    }

    pub fn draw(&self) {
        self.card_textures.draw_background();
        
        if self.window_too_small {
            self.draw_window_size_warning();
        }
        
        match &self.app_state {
            AppState::Menu => {
                crate::state::ui::menu::draw_menu(&self.selection, &self.player_name, self.is_editing_name, self.emoji_font.as_ref());
            }
            AppState::EnemySelection => {
                crate::state::ui::enemy_selection::draw_enemy_selection(&self.enemies, self.selected_enemy_index, self.emoji_font.as_ref(), self.persistent_player.as_ref(), self.show_instructions);
            }
            AppState::Battle(battle) => {
                crate::state::ui::battle::draw_battle(battle, &self.card_textures, self.emoji_font.as_ref(), &self.enemies[self.selected_enemy_index].image.as_ref().unwrap());
            }

            AppState::SoundSettings => {
                crate::state::ui::sound_settings::draw_sound_settings(
                    self.music_volume,
                    self.sfx_volume,
                    self.music_enabled,
                    self.sfx_enabled,
                    self.emoji_font.as_ref(),
                );
            }
            AppState::GameOver => {
                crate::state::ui::game_over::draw_game_over(&self.winner);
            }
        }
    }



    /// Desenha um aviso quando a janela está muito pequena
    fn draw_window_size_warning(&self) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        
        draw_rectangle(0.0, 0.0, screen_width, screen_height, Color::new(0.0, 0.0, 0.0, 0.8));
        
        let warning_text = "🚫 JANELA BLOQUEADA 🚫";
        let subtitle_text = "Redimensione a janela para pelo menos 800x600";
        let instruction_text = "Use as bordas da janela para redimensionar";
        let block_text = "JOGO BLOQUEADO - Redimensione para continuar";
        
        let warning_size = 24.0;
        let subtitle_size = 18.0;
        let instruction_size = 16.0;
        let block_size = 20.0;
        
        let warning_dims = measure_text(warning_text, None, warning_size as u16, 1.0);
        let subtitle_dims = measure_text(subtitle_text, None, subtitle_size as u16, 1.0);
        let instruction_dims = measure_text(instruction_text, None, instruction_size as u16, 1.0);
        let block_dims = measure_text(block_text, None, block_size as u16, 1.0);
        
        let warning_x = (screen_width - warning_dims.width) / 2.0;
        let subtitle_x = (screen_width - subtitle_dims.width) / 2.0;
        let instruction_x = (screen_width - instruction_dims.width) / 2.0;
        let block_x = (screen_width - block_dims.width) / 2.0;
        
        let center_y = screen_height / 2.0;
        
        // Desenhar textos
        draw_text(warning_text, warning_x, center_y - 60.0, warning_size, RED);
        draw_text(block_text, block_x, center_y - 20.0, block_size, ORANGE);
        draw_text(subtitle_text, subtitle_x, center_y + 20.0, subtitle_size, YELLOW);
        draw_text(instruction_text, instruction_x, center_y + 50.0, instruction_size, WHITE);
        
        // Mostrar tamanho atual da janela
        let size_text = format!("Tamanho atual: {}x{}", screen_width as i32, screen_height as i32);
        let size_dims = measure_text(&size_text, None, instruction_size as u16, 1.0);
        let size_x = (screen_width - size_dims.width) / 2.0;
        draw_text(&size_text, size_x, center_y + 80.0, instruction_size, LIGHTGRAY);
        
        // Mostrar tamanho mínimo necessário
        let min_text = "Tamanho mínimo: 800x600";
        let min_dims = measure_text(&min_text, None, instruction_size as u16, 1.0);
        let min_x = (screen_width - min_dims.width) / 2.0;
        draw_text(&min_text, min_x, center_y + 110.0, instruction_size, LIGHTGRAY);
    }

    fn execute_menu_selection(&mut self) {
        match self.selection {
            MenuSelection::Start => {
                self.app_state = AppState::EnemySelection;
            }
            MenuSelection::SoundSettings => {
                self.app_state = AppState::SoundSettings;
            }
            MenuSelection::Quit => {
                std::process::exit(0);
            }
        }
    }



    fn get_clicked_card_index(&self, mouse_x: f32, mouse_y: f32, hand: &Hand) -> Option<usize> {
        let start_y = screen_height() * 0.75;
        let total_width = (hand.cards.len() as f32) * (CARD_WIDTH + CARD_SPACING) - CARD_SPACING;
        let start_x = (screen_width() - total_width) / 2.0;
        for (i, _card) in hand.cards.iter().enumerate() {
            let base_x = start_x + (i as f32) * (CARD_WIDTH + CARD_SPACING);
            let base_y = start_y;
            let is_hovered = mouse_x >= base_x && mouse_x <= base_x + CARD_WIDTH && mouse_y >= base_y && mouse_y <= base_y + CARD_HEIGHT;
            if is_hovered {
                return Some(i);
            }
        }
        None
    }

    fn start_battle_with_selected_enemy(&mut self) {
        let selected_enemy = &self.enemies[self.selected_enemy_index];
        
        if !selected_enemy.is_unlocked {
            return;
        }

        let mut deck = Deck::new();
        
        let player = if let Some(ref persistent_player) = self.persistent_player {
            persistent_player.clone()
        } else {
            Player::new(&self.player_name, &mut deck)
        };

        // testando se é melhor resetar a mão do jogador
        // player.hand = Hand::new_from_deck(&mut deck, 5);

        let enemy = Enemy::new(
            &selected_enemy.name,
            selected_enemy.health,
            selected_enemy.max_health,
            selected_enemy.attack,  
            selected_enemy.defense, 
            &mut deck,
            selected_enemy.image.as_ref().unwrap(),
        );

        let mut battle_state = BattleState {
            player,
            enemy,
            deck,
            turn: GameTurn::new(MAX_TURNS),
            current_message: "The battle has begun!".to_string(),
            music_started: false,
            turn_cooldown: 0.0,
            waiting_for_cooldown: false,
            selected_card_index: None,
            card_animation_timer: 0.0,
            enemy_shake_timer: 0.0,
            damage_particles: Vec::new(),
            battle_log: Vec::new(),
            log_scroll_offset: 0.0,
            slow_motion_timer: 0.0,
            is_final_blow: false,
        };
        
        battle_state.add_battle_start_log(&selected_enemy.name);
        
        self.app_state = AppState::Battle(battle_state);
    }

    fn get_clicked_enemy_index(&self, mouse_x: f32, mouse_y: f32) -> Option<usize> {
        crate::state::ui::enemy_selection::get_clicked_enemy_index(mouse_x, mouse_y, &self.enemies)
    }

    pub fn on_battle_end(&mut self, winner: &str) {
        if winner == "Player" {
            if let AppState::Battle(battle) = &mut self.app_state {
                let player = &mut battle.player;
            
                let enemy = &mut self.enemies[self.selected_enemy_index];
                enemy.is_defeated = true;
                enemy.times_defeated += 1;
            
                let exp_gained = enemy.level * EXP_MULTIPLIER_PER_LEVEL + BASE_EXP_GAIN;
                let leveled_up = player.gain_experience(exp_gained);
                
                if leveled_up {
                } else {
                }
            
                let max_health_increase: u32 = (player.max_health as f32 * HEALTH_INCREASE_PERCENT) as u32;
                player.max_health = player.max_health.saturating_add(max_health_increase);
            
                player.health = player.max_health;
                let updated_player = player.clone();
            
                // Sai do escopo da batalha
                let _ = battle;
                self.scale_enemy(self.selected_enemy_index);
            
                if self.selected_enemy_index + 1 < self.enemies.len() {
                    self.enemies[self.selected_enemy_index + 1].is_unlocked = true;
                }
            
                self.persistent_player = Some(updated_player);
            }            
        }
        
        
        self.save_progress();
    }

    fn scale_enemy(&mut self, enemy_index: usize) {
        let enemy = &mut self.enemies[enemy_index];
        
        enemy.level += 1;
        
        let scaling_factor = 1.0 + (enemy.times_defeated as f32 * 0.25);
        let level_bonus = (enemy.level - 1) as f32 * 0.20;
        let total_multiplier = scaling_factor + level_bonus;
        
        enemy.health = (enemy.base_health as f32 * total_multiplier) as u32;
        enemy.max_health = enemy.health;
        enemy.attack = (enemy.base_attack as f32 * total_multiplier) as u32;
        enemy.defense = (enemy.base_defense as f32 * total_multiplier) as u32;
        
        enemy.health = enemy.health.max(enemy.base_health);
        enemy.max_health = enemy.max_health.max(enemy.base_health);
        enemy.attack = enemy.attack.max(enemy.base_attack);
        enemy.defense = enemy.defense.max(enemy.base_defense);
        
    }

    const SAVE_FILE: &'static str = "save_game.json";

    pub fn save_progress(&self) {
        let player_save_data = self.persistent_player.as_ref().map(|player| PlayerSaveData {
            name: player.name.clone(),
            health: player.max_health,
            max_health: player.max_health,
            attack: player.attack,
            defense: player.defense,
            level: player.level,
            experience: player.experience,
            experience_to_next_level: player.experience_to_next_level,
        });

        let sound_settings = SoundSettings {
            music_volume: self.music_volume,
            sfx_volume: self.sfx_volume,
            music_enabled: self.music_enabled,
            sfx_enabled: self.sfx_enabled,
        };

        let save_data = SaveData {
            enemies: self.enemies.clone(),
            persistent_player: player_save_data,
            player_name: self.player_name.clone(),
            sound_settings,
        };

        if let Ok(json) = serde_json::to_string_pretty(&save_data) {
            if let Err(_e) = fs::write(Self::SAVE_FILE, json) {
            } else {
            }
        }
    }

    pub fn load_progress(&mut self) {
        if Path::new(Self::SAVE_FILE).exists() {
            if let Ok(json) = fs::read_to_string(Self::SAVE_FILE) {
                if let Ok(save_data) = serde_json::from_str::<SaveData>(&json) {
                    self.enemies = save_data.enemies;
                            let default_enemies = get_default_enemies();
        for (index, enemy) in self.enemies.iter_mut().enumerate() {
            if index < default_enemies.len() && enemy.image.as_ref().unwrap() != default_enemies[index].image.as_ref().unwrap() {
                enemy.image = default_enemies[index].image.clone();
                enemy.emoji = default_enemies[index].emoji.clone();
                enemy.name = default_enemies[index].name.clone();
                enemy.base_health = default_enemies[index].base_health;
                enemy.base_attack = default_enemies[index].base_attack;
                enemy.base_defense = default_enemies[index].base_defense;
                enemy.health = default_enemies[index].health;
                enemy.max_health = default_enemies[index].max_health;
            }
        }
                    self.player_name = save_data.player_name;
                    
                    if let Some(player_data) = save_data.persistent_player {
                        let mut deck = Deck::new();
                        let mut player = Player::new(&player_data.name, &mut deck);
                        player.health = player_data.max_health;
                        player.max_health = player_data.max_health;
                        player.attack = player_data.attack;
                        player.defense = player_data.defense;
                        player.level = player_data.level;
                        player.experience = player_data.experience;
                        player.experience_to_next_level = player_data.experience_to_next_level;
                        self.persistent_player = Some(player);
                    }
                    
                    self.music_volume = save_data.sound_settings.music_volume;
                    self.sfx_volume = save_data.sound_settings.sfx_volume;
                    self.music_enabled = save_data.sound_settings.music_enabled;
                    self.sfx_enabled = save_data.sound_settings.sfx_enabled;
                    
                } else {
                }
            }
        }
    }

    pub fn reset_progress(&mut self) {
        self.enemies = get_default_enemies();
        
        self.persistent_player = None;
        
    }

    pub fn play_music_with_current_settings(&mut self) {
        if self.music_enabled {
            self.card_textures.play_background_music_with_volume(self.music_volume);
        } else {
            self.card_textures.stop_background_music();
        }
    }
}