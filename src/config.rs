/// Configurações globais do jogo
pub mod config {
    // Configurações de tempo
    pub const PLAYER_TURN_COOLDOWN: f32 = 1.0;
    pub const ENEMY_TURN_COOLDOWN: f32 = 1.0;
    pub const ENEMY_SHAKE_DURATION: f32 = 0.3;
    pub const FINAL_BLOW_DURATION: f32 = 1.0;
    
    // Configurações de tela
    pub const MIN_WINDOW_WIDTH: f32 = 800.0;
    pub const MIN_WINDOW_HEIGHT: f32 = 600.0;
    pub const DEFAULT_WINDOW_WIDTH: i32 = 1024;
    pub const DEFAULT_WINDOW_HEIGHT: i32 = 768;
    
    // Configurações de audio
    pub const DEFAULT_MUSIC_VOLUME: f32 = 0.5;
    pub const DEFAULT_SFX_VOLUME: f32 = 0.5;
    pub const BACKGROUND_MUSIC_VOLUME: f32 = 0.3;
    pub const CARD_USE_VOLUME: f32 = 0.2;
    pub const ENEMY_ATTACK_VOLUME: f32 = 0.2;
    
    // Configurações de gameplay
    pub const MAX_TURNS: u32 = 50;
    pub const INITIAL_HAND_SIZE: usize = 5;
    pub const MAX_BATTLE_LOG_LINES: usize = 8;
    pub const MAX_PLAYER_NAME_LENGTH: usize = 20;
    
    // Configurações de experiência
    pub const BASE_EXP_GAIN: u32 = 50;
    pub const EXP_MULTIPLIER_PER_LEVEL: u32 = 25;
    pub const HEALTH_INCREASE_PERCENT: f32 = 0.1;
    
    // Configurações de cartas
    pub const DECK_SIZE: usize = 40;
    pub const STATUS_EFFECT_DURATION_POISON: u32 = 4;
    pub const STATUS_EFFECT_DURATION_BURN: u32 = 3;
    
    // UI Constants
    pub const CARD_WIDTH: f32 = 120.0;
    pub const CARD_HEIGHT: f32 = 180.0;
    pub const CARD_SPACING: f32 = 10.0;
}