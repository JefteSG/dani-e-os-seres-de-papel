use ::rand::prelude::SliceRandom;
use ::rand::thread_rng;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams, Sound};
use macroquad::prelude::*;

// Constantes de volume de áudio
const BACKGROUND_MUSIC_VOLUME: f32 = 0.3;
const CARD_USE_VOLUME: f32 = 0.2;
const ENEMY_ATTACK_VOLUME: f32 = 0.2;

/// Tipos de carta
#[derive(Debug, Clone)]
pub enum CardType {
    Attack(u32),  // dano
    Defense(u32), // defesa
    Poison(u32),  // veneno
    Heal(u32),    // cura
}

/// Carta individual
#[derive(Debug, Clone)]
pub struct Card {
    pub name: String,
    pub card_type: CardType,
    pub image_path: String, // Caminho para a imagem
}

impl Card {
    pub fn new(name: &str, card_type: CardType, image_path: &str) -> Self {
        Self {
            name: name.to_string(),
            card_type,
            image_path: image_path.to_string(),
        }
    }

    /// Carrega a textura da carta (deve ser chamado no contexto async)
    pub async fn load_texture(&self) -> Result<Texture2D, macroquad::Error> {
        load_texture(&self.image_path).await
    }
}

/// Baralho do jogador
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = vec![
            Card::new("Ataque", CardType::Attack(10), "assets/cards/attack.png"),
            Card::new("Defesa", CardType::Defense(8), "assets/cards/defense.png"),
            Card::new("Veneno", CardType::Poison(4), "assets/cards/poison.png"),
            Card::new("Cura", CardType::Heal(10), "assets/cards/heal.png"),
        ];
        // Duplicar cartas para um baralho maior
        cards = cards.into_iter().cycle().take(40).collect();
        Self { cards }
    }

    /// misturar as cartas
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

/// Mão do jogador
#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Self { cards: vec![] }
    }

    pub fn new_from_deck(deck: &mut Deck, quantity: usize) -> Self {
        let mut hand = Self { cards: vec![] };
        hand.draw_from_deck(deck, quantity);
        hand
    }

    pub fn display(&self) {
        for (i, card) in self.cards.iter().enumerate() {
            println!("{}: {:?} - {}", i, card.card_type, card.name);
        }
    }

    pub fn draw_from_deck(&mut self, deck: &mut Deck, quantity: usize) {
        for _ in 0..quantity {
            if let Some(card) = deck.draw() {
                self.cards.push(card);
            }
        }
    }

    pub fn discard(&mut self, index: usize) -> Option<Card> {
        if index < self.cards.len() {
            Some(self.cards.remove(index))
        } else {
            None
        }
    }

    pub fn play(&self, index: usize) -> Option<&Card> {
        self.cards.get(index)
    }
}

/// Gerenciador de texturas das cartas e áudio
pub struct CardTextureManager {
    pub attack_texture: Option<Texture2D>,
    pub defense_texture: Option<Texture2D>,
    pub poison_texture: Option<Texture2D>,
    pub heal_texture: Option<Texture2D>,
    pub card_back_texture: Option<Texture2D>, // Para cartas viradas
    pub enemy_textures: std::collections::HashMap<String, Texture2D>, // Cache de texturas de inimigos

    // Sistema de áudio
    pub background_music: Option<Sound>,
    pub card_use_sound: Option<Sound>,
    pub enemy_attack_sound: Option<Sound>,
}

impl CardTextureManager {
    pub fn new() -> Self {
        Self {
            attack_texture: None,
            defense_texture: None,
            poison_texture: None,
            heal_texture: None,
            card_back_texture: None,
            enemy_textures: std::collections::HashMap::new(),
            background_music: None,
            card_use_sound: None,
            enemy_attack_sound: None,
        }
    }

    /// Carrega todas as texturas das cartas e áudios
    pub async fn load_all_textures(&mut self) {
        self.attack_texture = load_texture("assets/cards/attack.png").await.ok();
        self.defense_texture = load_texture("assets/cards/defense.png").await.ok();
        self.poison_texture = load_texture("assets/cards/poison.png").await.ok();
        self.heal_texture = load_texture("assets/cards/heal.png").await.ok();
        self.card_back_texture = load_texture("assets/cards/card_back.png").await.ok();

        // Carregar texturas de inimigos
        self.load_enemy_texture("assets/enemies/goblin.png").await;
        self.load_enemy_texture("assets/enemies/orc.png").await;
        self.load_enemy_texture("assets/enemies/dragon.png").await;
        self.load_enemy_texture("assets/enemies/alchemist.png").await;

        // Carregar áudios (silenciosamente falha se arquivos não existem)
        self.background_music = load_sound("assets/audio/music/background.ogg").await.ok();
        self.card_use_sound = load_sound("assets/audio/sfx/card_use.wav").await.ok();
        self.enemy_attack_sound = load_sound("assets/audio/sfx/enemy_attack.wav").await.ok();
    }

    /// Carrega uma textura de inimigo específica
    pub async fn load_enemy_texture(&mut self, image_path: &str) {
        if let Ok(texture) = load_texture(image_path).await {
            self.enemy_textures.insert(image_path.to_string(), texture);
        }
    }

    /// Pega a textura baseada no tipo de carta
    pub fn get_texture_for_card(&self, card: &Card) -> Option<&Texture2D> {
        match card.card_type {
            CardType::Attack(_) => self.attack_texture.as_ref(),
            CardType::Defense(_) => self.defense_texture.as_ref(),
            CardType::Poison(_) => self.poison_texture.as_ref(),
            CardType::Heal(_) => self.heal_texture.as_ref(),
        }
    }

    /// Desenha uma carta na posição especificada
    pub fn draw_card(&self, card: &Card, x: f32, y: f32, width: f32, height: f32) {
        if let Some(texture) = self.get_texture_for_card(card) {
            draw_texture_ex(
                texture,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(width, height)),
                    ..Default::default()
                },
            );
        } else {
            // Fallback: desenha um retângulo colorido se a imagem não carregar
            let color = match card.card_type {
                CardType::Attack(_) => RED,
                CardType::Defense(_) => BLUE,
                CardType::Poison(_) => GREEN,
                CardType::Heal(_) => YELLOW,
            };
            draw_rectangle(x, y, width, height, color);

            // Desenha o texto por cima
            let text_size = 16.0;
            let text_x =
                x + width / 2.0 - measure_text(&card.name, None, text_size as u16, 1.0).width / 2.0;
            let text_y = y + height / 2.0;
            draw_text(&card.name, text_x, text_y, text_size, BLACK);
        }
    }

    /// Desenha uma carta com escala (para animações)
    pub fn draw_card_scaled(&self, card: &Card, x: f32, y: f32, width: f32, height: f32) {
        // Por enquanto, é idêntico ao draw_card, mas permite futuras melhorias
        self.draw_card(card, x, y, width, height);
    }

    /// Desenha o verso da carta
    pub fn draw_card_back(&self, x: f32, y: f32, width: f32, height: f32) {
        if let Some(texture) = &self.card_back_texture {
            draw_texture_ex(
                texture,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(width, height)),
                    ..Default::default()
                },
            );
        } else {
            // Fallback: desenha um retângulo marrom para o verso
            draw_rectangle(x, y, width, height, BROWN);
            draw_rectangle_lines(x, y, width, height, 2.0, BLACK);
        }
    }

    /// Desenha um inimigo na posição especificada
    pub fn draw_enemy(&self, x: f32, y: f32, width: f32, height: f32, enemy_image: &str) {
        if let Some(texture) = self.enemy_textures.get(enemy_image) {
            draw_texture_ex(
                texture,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(width, height)),
                    ..Default::default()
                },
            );
        } else {
            // Fallback: desenha um retângulo vermelho para o inimigo
            draw_rectangle(x, y, width, height, DARKGRAY);
            draw_rectangle_lines(x, y, width, height, 3.0, RED);

            // Desenha um texto indicativo
            let text = "👾";
            let text_size = 32.0;
            let text_x =
                x + width / 2.0 - measure_text(text, None, text_size as u16, 1.0).width / 2.0;
            let text_y = y + height / 2.0;
            draw_text(text, text_x, text_y, text_size, RED);
        }
    }

    /// Toca a música de fundo
    pub fn play_background_music(&self) {
        if let Some(music) = &self.background_music {
            play_sound(
                music,
                PlaySoundParams {
                    looped: true,
                    volume: BACKGROUND_MUSIC_VOLUME,
                },
            );
        }
    }

    /// Toca o som de usar carta
    pub fn play_card_use_sound(&self) {
        if let Some(sound) = &self.card_use_sound {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume: CARD_USE_VOLUME,
                },
            );
        }
    }

    /// Toca o som de ataque do inimigo
    pub fn play_enemy_attack_sound(&self) {
        if let Some(sound) = &self.enemy_attack_sound {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume: ENEMY_ATTACK_VOLUME,
                },
            );
        }
    }
}

