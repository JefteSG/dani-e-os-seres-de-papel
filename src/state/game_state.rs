use crate::state::battle_state::BattleState;
use crate::state::menu::MenuSelection;
use crate::deck::{CardTextureManager, CardType, Deck, Hand};
use crate::enemy::Enemy;
use crate::player::Player;
use crate::effects::StatusEffect;
use crate::entity::Entity;
use crate::gameturn::GameTurn;
use crate::state::damage_particle::DamageParticle;
use macroquad::prelude::*;
use ::rand::random;
use ::rand::thread_rng;
use ::rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;



const PLAYER_TURN_COOLDOWN: f32 = 1.5;
const ENEMY_TURN_COOLDOWN: f32 = 1.0;
const ENEMY_SHAKE_DURATION: f32 = 0.3;

pub enum AppState {
    Menu,
    EnemySelection,
    Battle(BattleState),
    GameOver,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EnemyInfo {
    pub id: u32,
    pub name: String,
    pub base_health: u32,     // HP base (não muda)
    pub base_attack: u32,     // ATK base (não muda)
    pub base_defense: u32,    // DEF base (não muda)
    pub health: u32,          // HP atual (calculado)
    pub max_health: u32,      // HP máximo (calculado)
    pub attack: u32,          // ATK atual (calculado)
    pub defense: u32,         // DEF atual (calculado)
    pub level: u32,           // Nível do inimigo (aumenta a cada derrota)
    pub times_defeated: u32,  // Quantas vezes foi derrotado
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
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerSaveData {
    pub name: String,
    pub health: u32,
    pub max_health: u32,
    pub attack: u32,
    pub defense: u32,
}

pub struct GameState {
    pub app_state: AppState,
    pub selection: MenuSelection,
    pub winner: Option<String>,
    pub card_textures: CardTextureManager,
    pub enemies: Vec<EnemyInfo>,
    pub selected_enemy_index: usize,
    pub persistent_player: Option<Player>, // Para manter status entre batalhas
    pub player_name: String,              // Nome do jogador
    pub is_editing_name: bool,            // Se está editando o nome
    pub emoji_font: Option<Font>,         // Fonte para emojis
}

impl GameState {
    pub fn new() -> Self {
        let enemies = vec![
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
                is_unlocked: true,  // Primeiro inimigo sempre desbloqueado
                is_defeated: false,
                emoji: Some("💀".to_string()),
                image: Some("assets/enemies/goblin.png".to_string()),
            },
            EnemyInfo {
                id: 2,
                name: "Zumbi Guerreiro".to_string(),
                base_health: 140,
                base_attack: 35,
                base_defense: 16,
                health: 120,
                max_health: 120,
                attack: 25,
                defense: 16,
                level: 1,
                times_defeated: 0,
                is_unlocked: false, // Precisa derrotar o anterior
                is_defeated: false,
                emoji: Some("🧟".to_string()),
                image: Some("assets/enemies/orc.png".to_string()),
            },
            EnemyInfo {
                id: 3,
                name: "Dragão Sombrio".to_string(),
                base_health: 180,
                base_attack: 18,
                base_defense: 12,
                health: 180,
                max_health: 180,
                attack: 18,
                defense: 12,
                level: 3,
                times_defeated: 0,
                is_unlocked: false, // Precisa derrotar o anterior
                is_defeated: false,
                emoji: Some("🐲".to_string()),
                image: Some("assets/enemies/dragon.png".to_string()),
            },
            EnemyInfo {
                id: 4,
                name: "Alquimista".to_string(),
                base_health: 180,
                base_attack: 18,
                base_defense: 12,
                health: 180,
                max_health: 180,
                attack: 18,
                defense: 12,
                level: 1,
                times_defeated: 0,
                is_unlocked: false, // Precisa derrotar o anterior
                is_defeated: false,
                emoji: Some("🧪".to_string()),
                image: Some("assets/enemies/alchemist.png".to_string()),
            },
        ];

        let mut game_state = Self {
            app_state: AppState::Menu,
            selection: MenuSelection::Start,
            winner: None,
            card_textures: CardTextureManager::new(),
            enemies,
            selected_enemy_index: 0,
            persistent_player: None,
            player_name: "Jogador".to_string(), // Nome padrão
            is_editing_name: false,
            emoji_font: None,
        };
        
        // Carregar progresso salvo
        game_state.load_progress();
        
        game_state
    }

    pub async fn load_card_textures(&mut self) {
        self.card_textures.load_all_textures().await;
    }

    pub fn update(&mut self) {
        // Calcule o índice do clique antes do bloco mutável para evitar borrow duplo
        let mut clicked_card_index = None;
        if let AppState::Battle(battle) = &self.app_state {
            if battle.turn.player_turn() && !battle.waiting_for_cooldown {
                let (mouse_x, mouse_y) = mouse_position();
                clicked_card_index = self.get_clicked_card_index(mouse_x, mouse_y, &battle.player.hand);
            }
        }
        match &mut self.app_state {
            AppState::Menu => {
                // Lógica de edição de nome
                if self.is_editing_name {
                    // Processar input de texto
                    if is_key_pressed(KeyCode::Enter) {
                        self.is_editing_name = false;
                        if self.player_name.trim().is_empty() {
                            self.player_name = "Jogador".to_string();
                        }
                        println!("Nome do jogador: {}", self.player_name);
                            // Salvar o nome do jogador
                        self.save_progress();
                    }
                    if is_key_pressed(KeyCode::Escape) {
                        self.is_editing_name = false;
                        self.player_name = "Jogador".to_string(); // Resetar para padrão
                    }
                    if is_key_pressed(KeyCode::Backspace) && !self.player_name.is_empty() {
                        self.player_name.pop();
                    }
                    
                    // Capturar caracteres digitados
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
                        if is_key_pressed(key_code) && self.player_name.len() < 20 {
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
                    // Navegação normal do menu
                    if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                        self.selection = MenuSelection::Start;
                    }
                    if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                        self.selection = MenuSelection::Quit;
                    }
                    if is_key_pressed(KeyCode::Tab) {
                        self.is_editing_name = true;
                    }
                    if is_key_pressed(KeyCode::Enter) {
                        self.execute_menu_selection();
                    }
                }
                
                // Clique do mouse (funciona sempre)
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mouse_x, mouse_y) = mouse_position();
                    
                    // Verificar clique no campo de nome
                    if crate::state::ui::menu::is_name_field_clicked(mouse_x, mouse_y) {
                        self.is_editing_name = !self.is_editing_name;
                    }
                    // Verificar clique nas opções do menu (apenas se não estiver editando nome)
                    else if !self.is_editing_name {
                        if let Some(clicked_option) = crate::state::ui::menu::get_clicked_menu_option(mouse_x, mouse_y) {
                            self.selection = clicked_option;
                            self.execute_menu_selection();
                        }
                    }
                }
                
                // Hover do mouse (apenas se não estiver editando nome)
                if !self.is_editing_name {
                    let (mouse_x, mouse_y) = mouse_position();
                    if let Some(hovered_option) = crate::state::ui::menu::get_clicked_menu_option(mouse_x, mouse_y) {
                        self.selection = hovered_option;
                    }
                }
            }
            AppState::EnemySelection => {
                // Navegação com teclado
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
                
                // Seleção com Enter ou clique
                if is_key_pressed(KeyCode::Enter) {
                    self.start_battle_with_selected_enemy();
                }
                
                // Voltar ao menu com ESC
                if is_key_pressed(KeyCode::Escape) {
                    self.app_state = AppState::Menu;
                }
                
                // Sair do jogo com Q
                if is_key_pressed(KeyCode::Q) {
                    std::process::exit(0);
                }
                
                // Reset do progresso com R (segurando Shift)
                if is_key_pressed(KeyCode::R) && (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)) {
                    self.reset_progress();
                }
                
                // Clique do mouse (será implementado na UI)
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mouse_x, mouse_y) = mouse_position();
                    if let Some(clicked_enemy) = self.get_clicked_enemy_index(mouse_x, mouse_y) {
                        self.selected_enemy_index = clicked_enemy;
                        self.start_battle_with_selected_enemy();
                    }
                }
            }
            AppState::Battle(battle) => {
                if !battle.music_started {
                    self.card_textures.play_background_music();
                    battle.music_started = true;
                }
                if battle.turn.turn_over() {
                    self.app_state = AppState::GameOver;
                    return;
                }
                if battle.turn.player_turn() {
                    if !battle.waiting_for_cooldown {
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
                            
                            // Adicionar log de uso de carta
                            battle.add_card_log("Jogador", &card.name);
                            
                            match card.card_type {
                                CardType::Attack(damage) => {
                                    battle.player.attack_up(damage);
                                    battle.current_message = format!(
                                        "Você usou {} e aumentou o ataque em {}!",
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
                                    battle.enemy.status_effect(StatusEffect::Poison, 2);
                                    battle.current_message =
                                        format!("Você usou {} e envenenou o inimigo!", card.name);
                                }
                                CardType::Heal(heal) => {
                                    battle.player.heal(heal);
                                    battle.current_message =
                                        format!("Você usou {} e se curou em {}!", card.name, heal);
                                }
                            }
                            if !battle.deck.cards.is_empty() {
                                battle.player.hand.draw_from_deck(&mut battle.deck, 1);
                            }
                            let damage_dealt = battle.player.attack;
                            let enemy_health_before = battle.enemy.health;
                            battle.enemy.damage(damage_dealt);
                            let actual_damage = enemy_health_before - battle.enemy.health;
                            
                            // Adicionar log de dano
                            let enemy_name = battle.enemy.name.clone();
                            battle.add_damage_log("Jogador", &enemy_name, damage_dealt, actual_damage);
                            
                            battle.enemy.apply_status_effects();
                            if battle.enemy.health > 0 {
                                battle.enemy_shake_timer = ENEMY_SHAKE_DURATION;
                                let enemy_x = screen_width() / 2.0;
                                let enemy_y = screen_height() / 2.0 - 50.0;
                                battle.damage_particles.push(DamageParticle::new(
                                    enemy_x,
                                    enemy_y,
                                    damage_dealt,
                                ));
                            }
                            battle.turn_cooldown = PLAYER_TURN_COOLDOWN;
                            battle.waiting_for_cooldown = true;
                            if battle.enemy.health <= 0 {
                                battle.add_battle_end_log("Jogador");
                                self.winner = Some("Jogador".to_string());
                                self.on_battle_end("Jogador");
                                self.app_state = AppState::GameOver;
                                return;
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
                            
                            // Adicionar log de mudança de turno (jogador -> inimigo)
                            let enemy_name = battle.enemy.name.clone();
                            battle.add_turn_log(&enemy_name);
                        }
                    }
                } else {
                    if !battle.waiting_for_cooldown {
                        if let Some(card) = battle.deck.cards.choose(&mut thread_rng()).cloned() {
                            self.card_textures.play_enemy_attack_sound();
                            match card.card_type {
                                CardType::Attack(attack) => {
                                    battle.enemy.attack_up(attack);
                                    battle.current_message =
                                        format!("Inimigo aumentou ataque em {}!", attack);
                                }
                                CardType::Defense(defense) => {
                                    battle.enemy.defense_up(defense);
                                    battle.current_message =
                                        format!("Inimigo aumentou defesa em {}!", defense);
                                }
                                CardType::Poison(_) => {
                                    battle.player.status_effect(StatusEffect::Poison, 2);
                                    battle.current_message = "Inimigo aplicou veneno!".to_string();
                                }
                                CardType::Heal(heal) => {
                                    battle.enemy.heal(heal);
                                    battle.current_message = format!("Inimigo curou {}!", heal);
                                }
                            }
                            let damage_dealt = battle.enemy.attack;
                            let player_health_before = battle.player.health;
                            battle.player.damage(damage_dealt);
                            let actual_damage = player_health_before - battle.player.health;
                            
                            // Adicionar log de dano
                            let enemy_name = battle.enemy.name.clone();
                            battle.add_damage_log(&enemy_name, "Jogador", damage_dealt, actual_damage);
                            
                            battle.player.apply_status_effects();
                            let player_x = screen_width() / 2.0;
                            let player_y = screen_height() * 0.8;
                            battle.damage_particles.push(DamageParticle::new(
                                player_x,
                                player_y,
                                damage_dealt,
                            ));
                            battle.turn_cooldown = ENEMY_TURN_COOLDOWN;
                            battle.waiting_for_cooldown = true;
                            if battle.player.health <= 0 {
                                battle.add_battle_end_log("Inimigo");
                                self.winner = Some("Inimigo".to_string());
                                self.on_battle_end("Inimigo");
                                self.app_state = AppState::GameOver;
                                return;
                            }
                        } else {
                            battle.current_message =
                                "Deck inimigo vazio. Turno pulado.".to_string();
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
                            
                            // Adicionar log de mudança de turno (inimigo -> jogador)
                            battle.add_turn_log("Jogador");
                        }
                    }
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
        clear_background(BLACK);
        match &self.app_state {
            AppState::Menu => {
                crate::state::ui::menu::draw_menu(&self.selection, &self.player_name, self.is_editing_name, self.emoji_font.as_ref());
            }
            AppState::EnemySelection => {
                crate::state::ui::enemy_selection::draw_enemy_selection(&self.enemies, self.selected_enemy_index, self.emoji_font.as_ref());
            }
            AppState::Battle(battle) => {
                crate::state::ui::battle::draw_battle(battle, &self.card_textures, self.emoji_font.as_ref(), &self.enemies[self.selected_enemy_index].image.as_ref().unwrap());
            }
            AppState::GameOver => {
                crate::state::ui::game_over::draw_game_over(&self.winner);
            }
        }
    }

    fn execute_menu_selection(&mut self) {
        match self.selection {
            MenuSelection::Start => {
                self.app_state = AppState::EnemySelection;
            }
            MenuSelection::Quit => {
                std::process::exit(0);
            }
        }
    }



    fn get_clicked_card_index(&self, mouse_x: f32, mouse_y: f32, hand: &Hand) -> Option<usize> {
        let card_width = 120.0;
        let card_height = 180.0;
        let card_spacing = 10.0;
        let start_y = screen_height() * 0.75; // Corrigido: mesma posição que draw_player_hand_with_animation
        let total_width = (hand.cards.len() as f32) * (card_width + card_spacing) - card_spacing;
        let start_x = (screen_width() - total_width) / 2.0;
        for (i, _card) in hand.cards.iter().enumerate() {
            let base_x = start_x + (i as f32) * (card_width + card_spacing);
            let base_y = start_y;
            let is_hovered = mouse_x >= base_x && mouse_x <= base_x + card_width && mouse_y >= base_y && mouse_y <= base_y + card_height;
            if is_hovered {
                return Some(i);
            }
        }
        None
    }

    fn start_battle_with_selected_enemy(&mut self) {
        let selected_enemy = &self.enemies[self.selected_enemy_index];
        
        // Verificar se o inimigo está desbloqueado
        if !selected_enemy.is_unlocked {
            return; // Não fazer nada se o inimigo não estiver desbloqueado
        }

        let mut deck = Deck::new();
        
        // Usar jogador persistente ou criar novo
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
            selected_enemy.attack,  // Corrigido: attack vem antes
            selected_enemy.defense, // Corrigido: defense vem depois  
            &mut deck,
            selected_enemy.image.as_ref().unwrap(),
        );

        let mut battle_state = BattleState {
            player,
            enemy,
            deck,
            turn: GameTurn::new(20),
            current_message: "A batalha começou!".to_string(),
            music_started: false,
            turn_cooldown: 0.0,
            waiting_for_cooldown: false,
            selected_card_index: None,
            card_animation_timer: 0.0,
            enemy_shake_timer: 0.0,
            damage_particles: Vec::new(),
            battle_log: Vec::new(),
            log_scroll_offset: 0.0,
        };
        
        // Adicionar log inicial da batalha
        battle_state.add_battle_start_log(&selected_enemy.name);
        
        self.app_state = AppState::Battle(battle_state);
    }

    fn get_clicked_enemy_index(&self, mouse_x: f32, mouse_y: f32) -> Option<usize> {
        crate::state::ui::enemy_selection::get_clicked_enemy_index(mouse_x, mouse_y, &self.enemies)
    }

    pub fn on_battle_end(&mut self, winner: &str) {
        if winner == "Jogador" {
            // Marcar inimigo atual como derrotado e aumentar contador
            let enemy = &mut self.enemies[self.selected_enemy_index];
            enemy.is_defeated = true;
            enemy.times_defeated += 1;
            
            // Escalar o inimigo (ficar mais forte)
            self.scale_enemy(self.selected_enemy_index);
            
            // Desbloquear próximo inimigo se existir
            if self.selected_enemy_index + 1 < self.enemies.len() {
                self.enemies[self.selected_enemy_index + 1].is_unlocked = true;
            }
            
            // Salvar estado do jogador para próximas batalhas
            if let AppState::Battle(battle) = &self.app_state {
                self.persistent_player = Some(battle.player.clone());
            }
        } else {
            // Se perdeu, resetar jogador persistente
            self.persistent_player = None;
        }
        
        // Salvar progresso automaticamente
        self.save_progress();
    }

    fn scale_enemy(&mut self, enemy_index: usize) {
        let enemy = &mut self.enemies[enemy_index];
        
        // Aumentar nível
        enemy.level += 1;
        
        // Fórmulas de escalabilidade (balanceadas)
        let scaling_factor = 1.0 + (enemy.times_defeated as f32 * 0.25); // +25% por vitória
        let level_bonus = (enemy.level - 1) as f32 * 0.20; // +20% por nível
        let total_multiplier = scaling_factor + level_bonus;
        
        // Aplicar escalabilidade aos stats
        enemy.health = (enemy.base_health as f32 * total_multiplier) as u32;
        enemy.max_health = enemy.health;
        enemy.attack = (enemy.base_attack as f32 * total_multiplier) as u32;
        enemy.defense = (enemy.base_defense as f32 * total_multiplier) as u32;
        
        // Garantir valores mínimos
        enemy.health = enemy.health.max(enemy.base_health);
        enemy.max_health = enemy.max_health.max(enemy.base_health);
        enemy.attack = enemy.attack.max(enemy.base_attack);
        enemy.defense = enemy.defense.max(enemy.base_defense);
        
        println!("🔥 {} subiu para nível {}! Stats aumentados!", enemy.name, enemy.level);
        println!("   HP: {} | ATK: {} | DEF: {}", enemy.health, enemy.attack, enemy.defense);
    }

    const SAVE_FILE: &'static str = "save_game.json";

    pub fn save_progress(&self) {
        let player_save_data = self.persistent_player.as_ref().map(|player| PlayerSaveData {
            name: player.name.clone(),
            health: player.health,
            max_health: player.max_health,
            attack: player.attack,
            defense: player.defense,
        });

        let save_data = SaveData {
            enemies: self.enemies.clone(),
            persistent_player: player_save_data,
            player_name: self.player_name.clone(),
        };

        if let Ok(json) = serde_json::to_string_pretty(&save_data) {
            if let Err(e) = fs::write(Self::SAVE_FILE, json) {
                println!("Erro ao salvar progresso: {}", e);
            } else {
                println!("Progresso salvo com sucesso!");
            }
        }
    }

    pub fn load_progress(&mut self) {
        if Path::new(Self::SAVE_FILE).exists() {
            if let Ok(json) = fs::read_to_string(Self::SAVE_FILE) {
                if let Ok(save_data) = serde_json::from_str::<SaveData>(&json) {
                    self.enemies = save_data.enemies;
                    self.player_name = save_data.player_name;
                    
                    // Converter PlayerSaveData de volta para Player
                    if let Some(player_data) = save_data.persistent_player {
                        let mut deck = Deck::new();
                        let mut player = Player::new(&player_data.name, &mut deck);
                        player.health = player_data.health;
                        player.max_health = player_data.max_health;
                        player.attack = player_data.attack;
                        player.defense = player_data.defense;
                        self.persistent_player = Some(player);
                    }
                    
                    println!("Progresso carregado com sucesso!");
                } else {
                    println!("Erro ao carregar save: arquivo corrompido");
                }
            }
        }
    }

    pub fn reset_progress(&mut self) {
        // Resetar inimigos para estado inicial
        self.enemies = vec![
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
                image: Some("assets/enemies/goblin.png".to_string()),
            },
            EnemyInfo {
                id: 2,
                name: "Zumbi Guerreiro".to_string(),
                base_health: 120,
                base_attack:25,
                base_defense: 16,
                health: 120,
                max_health: 120,
                attack: 25,
                defense: 16,
                level: 1,
                times_defeated: 0,
                is_unlocked: false,
                is_defeated: false,
                emoji: Some("🧟".to_string()),
                image: Some("assets/enemies/orc.png".to_string()),
            },
            EnemyInfo {
                id: 3,
                name: "Dragão Sombrio".to_string(),
                base_health: 180,
                base_attack: 18,
                base_defense: 12,
                health: 180,
                max_health: 180,
                attack: 18,
                defense: 12,
                level: 3,
                times_defeated: 0,
                is_unlocked: false,
                is_defeated: false,
                emoji: Some("🐲".to_string()),
                image: Some("assets/enemies/dragon.png".to_string()),
            },
            EnemyInfo {
                id: 4,
                name: "Alquimista".to_string(),
                base_health: 180,
                base_attack: 18,
                base_defense: 12,
                health: 180,
                max_health: 180,
                attack: 18,
                defense: 12,
                level: 1,
                times_defeated: 0,
                is_unlocked: false,
                is_defeated: false,
                emoji: Some("🧪".to_string()),
                image: Some("assets/enemies/alchemist.png".to_string()),
            },
        ];
        
        // Resetar jogador persistente
        self.persistent_player = None;
        
        // Deletar arquivo de save
        if Path::new(Self::SAVE_FILE).exists() {
            if let Err(e) = fs::remove_file(Self::SAVE_FILE) {
                println!("Erro ao deletar save: {}", e);
            }
        }
        
        println!("🔄 Progresso resetado! Todos os inimigos voltaram ao nível 1.");
    }
}