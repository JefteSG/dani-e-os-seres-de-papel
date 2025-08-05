use crate::player::Player;
use crate::enemy::Enemy;
use crate::deck::Deck;
use crate::gameturn::GameTurn;

use crate::state::damage_particle::DamageParticle;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct BattleLogEntry {
    pub message: String,
    pub color: Color,
    pub timestamp: f32,
}

pub struct BattleState {
    pub player: Player,
    pub enemy: Enemy,
    pub deck: Deck,
    pub turn: GameTurn,
    pub current_message: String,
    pub music_started: bool,
    pub turn_cooldown: f32,
    pub waiting_for_cooldown: bool,
    pub selected_card_index: Option<usize>,
    pub card_animation_timer: f32,
    pub enemy_shake_timer: f32,
    pub damage_particles: Vec<DamageParticle>,
    pub battle_log: Vec<BattleLogEntry>,
    pub log_scroll_offset: f32,
    pub slow_motion_timer: f32,
    pub is_final_blow: bool,
}

impl BattleState {
    pub fn add_log(&mut self, message: String, color: Color) {
        let entry = BattleLogEntry {
            message,
            color,
            timestamp: get_time() as f32,
        };
        
        self.battle_log.push(entry);
        
        if self.battle_log.len() > 50 {
            self.battle_log.remove(0);
        }
        
        self.scroll_to_bottom();
    }
    
    pub fn add_damage_log(&mut self, attacker: &str, target: &str, damage: u32, actual_damage: u32) {
        let message = if actual_damage < damage {
            format!("{} atacou {} por {} de dano ({} bloqueado pela defesa)", 
                attacker, target, actual_damage, damage - actual_damage)
        } else {
            format!("{} causou {} de dano em {}", attacker, actual_damage, target)
        };
        self.add_log(message, if attacker == "Jogador" { LIME } else { ORANGE });
    }
    
    pub fn add_heal_log(&mut self, target: &str, heal: u32) {
        let message = format!("{} se curou em {} pontos de vida", target, heal);
        self.add_log(message, GREEN);
    }
    
    pub fn add_status_log(&mut self, target: &str, status: &str, duration: u32) {
        let message = format!("{} foi afetado por {} por {} turnos", target, status, duration);
        self.add_log(message, PURPLE);
    }
    
    pub fn add_card_log(&mut self, player: &str, card_name: &str) {
        let message = format!("{} usou carta: {}", player, card_name);
        self.add_log(message, YELLOW);
    }
    
    pub fn add_turn_log(&mut self, player: &str) {
        let message = format!("--- Turno de {} ---", player);
        self.add_log(message, WHITE);
    }
    
    pub fn add_battle_start_log(&mut self, enemy_name: &str) {
        self.add_log("BATALHA INICIADA!".to_string(), RED);
        self.add_log(format!("Enfrentando: {}", enemy_name), ORANGE);
        self.add_log("Boa sorte, guerreiro!".to_string(), LIME);
    }
    
    pub fn add_battle_end_log(&mut self, winner: &str) {
        if winner == "Jogador" {
            self.add_log("🎉 VITÓRIA! 🎉".to_string(), GOLD);
            self.add_log("Você derrotou o inimigo!".to_string(), LIME);
        } else {
            self.add_log("💀 DERROTA 💀".to_string(), RED);
            self.add_log("Você foi derrotado...".to_string(), GRAY);
        }
    }
    
    fn scroll_to_bottom(&mut self) {
        let max_visible_lines = 8;
        if self.battle_log.len() > max_visible_lines {
            self.log_scroll_offset = (self.battle_log.len() - max_visible_lines) as f32;
        } else {
            self.log_scroll_offset = 0.0;
        }
    }
}