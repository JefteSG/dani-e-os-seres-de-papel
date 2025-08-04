use macroquad::audio::{load_sound, play_sound, PlaySoundParams, Sound};
use macroquad::prelude::*;
use ::rand::prelude::*;
use ::rand::Rng;

// Constantes de volume de áudio
const BACKGROUND_MUSIC_VOLUME: f32 = 0.3;
const CARD_USE_VOLUME: f32 = 0.2;
const ENEMY_ATTACK_BASIC_VOLUME: f32 = 0.2;

/// Tipos de carta
#[derive(Debug, Clone)]
pub enum CardType {
    Attack_basic(u32),
    Attack_strong(u32),
    Defense(u32), 
    Poison(u32),
    Burn(u32),
    Heal(f32),   // Valor percentual (ex: 0.25 = 25% da vida máxima)
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
            Card::new("Ataque", CardType::Attack_basic(8), "assets/cards/attack_basic.png"),
            Card::new("Ataque Forte", CardType::Attack_strong(15), "assets/cards/attack_strong.png"),
            Card::new("Defesa", CardType::Defense(8), "assets/cards/defense.png"),
            Card::new("Veneno", CardType::Poison(4), "assets/cards/poison.png"),
            Card::new("Cura", CardType::Heal(0.25), "assets/cards/heal.png"), // 25% da vida máxima
            Card::new("Queimadura", CardType::Burn(10), "assets/cards/burn.png"),
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

    /// Cria uma mão com chances específicas baseadas no inimigo
    pub fn new_with_enemy_chances(enemy_name: &str, quantity: usize) -> Self {
        let mut hand = Self { cards: vec![] };
        let chances = EnemyCardChances::for_enemy(enemy_name);
        let mut rng = thread_rng();
        
        for _ in 0..quantity {
            let roll: f32 = rng.gen(); // Número aleatório entre 0.0 e 1.0
            let card = hand.select_card_by_chance(roll, &chances);
            hand.cards.push(card);
        }
        
        hand
    }

    /// Seleciona uma carta baseada nas chances
    fn select_card_by_chance(&self, roll: f32, chances: &EnemyCardChances) -> Card {
        let card_options = [
            (
                chances.attack_basic,
                "Ataque Básico",
                CardType::Attack_basic(15),
                "assets/cards/attack.png",
            ),
            (
                chances.attack_strong,
                "Ataque Forte",
                CardType::Attack_strong(25),
                "assets/cards/attack.png",
            ),
            (
                chances.defense,
                "Defesa",
                CardType::Defense(10),
                "assets/cards/defense.png",
            ),
            (
                chances.poison,
                "Veneno",
                CardType::Poison(5),
                "assets/cards/poison.png",
            ),
            (
                chances.burn,
                "Queimadura",
                CardType::Burn(5),
                "assets/cards/burn.png",
            ),
        ];
    
        let mut cumulative = 0.0;
        for (chance, name, card_type, asset) in card_options {
            cumulative += chance;
            if roll < cumulative {
                return Card::new(name, card_type, asset);
            }
        }
    
        // Fallback
        Card::new("Cura", CardType::Heal(0.25), "assets/cards/heal.png")
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

/// Estrutura para definir chances de cartas por inimigo
pub struct EnemyCardChances {
    pub attack_basic: f32,    // Chance de carta de ataque básico (0.0 a 1.0)
    pub attack_strong: f32,   // Chance de carta de ataque forte
    pub defense: f32,         // Chance de carta de defesa
    pub poison: f32,          // Chance de carta de veneno
    pub burn: f32,            // Chance de carta de queimadura
    pub heal: f32,            // Chance de carta de cura
}

impl EnemyCardChances {
    /// Retorna as chances específicas para cada inimigo
    pub fn for_enemy(enemy_name: &str) -> Self {
        match enemy_name.to_lowercase().as_str() {
            "dragão" | "dragon" => Self {
                attack_basic: 0.10,  
                attack_strong: 0.05,
                defense: 0.10,
                poison: 0.10,
                burn: 0.50,
                heal: 0.05,   
            },
            "devorador" | "devourer" => Self {
                attack_basic: 0.30,   // 30% chance
                attack_strong: 0.35,  // 35% chance (mais ataque)
                defense: 0.05,        // 5% chance
                poison: 0.10,         // 10% chance
                burn: 0.10,           // 10% chance
                heal: 0.10,           // 10% chance
            },
            "zumbi" | "zombie" => Self {
                attack_basic: 0.20,   // 20% chance
                attack_strong: 0.15,  // 15% chance
                defense: 0.10,        // 10% chance
                poison: 0.35,         // 35% chance (mais poison)
                burn: 0.10,           // 10% chance
                heal: 0.10,           // 10% chance
            },
            "esqueleto" | "skeleton" => Self {
                attack_basic: 0.25,   // 25% chance
                attack_strong: 0.20,  // 20% chance
                defense: 0.15,        // 15% chance
                poison: 0.15,         // 15% chance
                burn: 0.15,           // 15% chance
                heal: 0.10,           // 10% chance
            },
            _ => Self {
                attack_basic: 0.10,   // Distribuição padrão
                attack_strong: 0.05,
                defense: 0.10,
                poison: 0.10,
                burn: 0.50,
                heal: 0.05,
            },
        }
    }
}

/// Gerenciador de texturas das cartas e áudio
pub struct CardTextureManager {
    pub attack_basic_texture: Option<Texture2D>,
    pub attack_strong_texture: Option<Texture2D>,
    pub defense_texture: Option<Texture2D>,
    pub poison_texture: Option<Texture2D>,
    pub heal_texture: Option<Texture2D>,
    pub burn_texture: Option<Texture2D>,
    pub card_back_texture: Option<Texture2D>, // Para cartas viradas
    pub enemy_textures: std::collections::HashMap<String, Texture2D>, // Cache de texturas de inimigos
    pub background_texture: Option<Texture2D>, // Background do jogo

    // Sistema de áudio
    pub background_music: Option<Sound>,
    pub card_use_sound: Option<Sound>,
    pub enemy_attack_basic_sound: Option<Sound>,
    pub enemy_sounds: std::collections::HashMap<String, Sound>, // Sons específicos por inimigo
}

impl CardTextureManager {
    pub fn new() -> Self {
        Self {
            attack_basic_texture: None,
            attack_strong_texture: None,
            defense_texture: None,
            poison_texture: None,
            heal_texture: None,
            burn_texture: None,
            card_back_texture: None,
            enemy_textures: std::collections::HashMap::new(),
            background_texture: None,
            background_music: None,
            card_use_sound: None,
            enemy_attack_basic_sound: None,
            enemy_sounds: std::collections::HashMap::new(),
        }
    }

    /// Carrega todas as texturas das cartas e áudios
    pub async fn load_all_textures(&mut self) {
        self.attack_basic_texture = load_texture("assets/cards/attack_basic.png").await.ok();
        self.attack_strong_texture = load_texture("assets/cards/attack_strong.png").await.ok();
        self.defense_texture = load_texture("assets/cards/defense.png").await.ok();
        self.poison_texture = load_texture("assets/cards/poison.png").await.ok();
        self.heal_texture = load_texture("assets/cards/heal.png").await.ok();
        self.card_back_texture = load_texture("assets/cards/card_back.png").await.ok();
        self.background_texture = load_texture("assets/background.png").await.ok();
        self.burn_texture = load_texture("assets/cards/burn.png").await.ok();
        // Carregar texturas de inimigos
        self.load_enemy_texture("assets/enemies/skeleton.png").await;
        self.load_enemy_texture("assets/enemies/zombie.png").await;
        self.load_enemy_texture("assets/enemies/dragon.png").await;
        self.load_enemy_texture("assets/enemies/devourer.png").await;

        // Carregar áudios (silenciosamente falha se arquivos não existem)
        self.background_music = load_sound("assets/audio/music/background.ogg").await.ok();
        self.card_use_sound = load_sound("assets/audio/sfx/card_use.wav").await.ok();
        self.enemy_attack_basic_sound = load_sound("assets/audio/sfx/enemy_attack_basic.wav").await.ok();
        
        // Carregar sons específicos por inimigo
        self.load_enemy_sound("skeleton", "assets/audio/sfx/skeleton_attack.wav").await;
        self.load_enemy_sound("zombie", "assets/audio/sfx/zombie_attack.wav").await;
        self.load_enemy_sound("dragon", "assets/audio/sfx/dragon_attack.wav").await;
        self.load_enemy_sound("devourer", "assets/audio/sfx/devourer_attack.wav").await;
    }

    /// Carrega uma textura de inimigo específica
    pub async fn load_enemy_texture(&mut self, image_path: &str) {
        if let Ok(texture) = load_texture(image_path).await {
            self.enemy_textures.insert(image_path.to_string(), texture);
        }
    }

    /// Carrega um som específico de inimigo
    pub async fn load_enemy_sound(&mut self, enemy_name: &str, sound_path: &str) {
        if let Ok(sound) = load_sound(sound_path).await {
            self.enemy_sounds.insert(enemy_name.to_string(), sound);
        }
    }

    /// Pega a textura baseada no tipo de carta
    pub fn get_texture_for_card(&self, card: &Card) -> Option<&Texture2D> {
        match card.card_type {
            CardType::Attack_basic(_) => self.attack_basic_texture.as_ref(),
            CardType::Attack_strong(_) => self.attack_strong_texture.as_ref(),
            CardType::Defense(_) => self.defense_texture.as_ref(),
            CardType::Poison(_) => self.poison_texture.as_ref(),
            CardType::Heal(_) => self.heal_texture.as_ref(),
            CardType::Burn(_) => self.burn_texture.as_ref(),
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
                CardType::Attack_basic(_) => RED,
                CardType::Attack_strong(_) => RED,
                CardType::Defense(_) => BLUE,
                CardType::Poison(_) => GREEN,
                CardType::Heal(_) => YELLOW,
                CardType::Burn(_) => RED,
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
            // Calcular proporção para manter aspect ratio
            let texture_width = texture.width();
            let texture_height = texture.height();
            let aspect_ratio = texture_width / texture_height;
            let target_aspect_ratio = width / height;
            
            let (final_width, final_height, final_x, final_y) = if aspect_ratio > target_aspect_ratio {
                // Imagem é mais larga, ajustar pela largura
                let new_width = width;
                let new_height = width / aspect_ratio;
                let new_y = y + (height - new_height) / 2.0;
                (new_width, new_height, x, new_y)
            } else {
                // Imagem é mais alta, ajustar pela altura
                let new_height = height;
                let new_width = height * aspect_ratio;
                let new_x = x + (width - new_width) / 2.0;
                (new_width, new_height, new_x, y)
            };
            
            draw_texture_ex(
                texture,
                final_x,
                final_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(final_width, final_height)),
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
    pub fn play_enemy_attack_basic_sound(&self) {
        if let Some(sound) = &self.enemy_attack_basic_sound {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume: ENEMY_ATTACK_BASIC_VOLUME,
                },
            );
        }
    }

    /// Toca o som específico de um inimigo
    pub fn play_enemy_sound(&self, enemy_name: &str) {
        if let Some(sound) = self.enemy_sounds.get(enemy_name) {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume: ENEMY_ATTACK_BASIC_VOLUME,
                },
            );
        } else {
            // Fallback para o som básico se não encontrar o som específico
            self.play_enemy_attack_basic_sound();
        }
    }

    /// Desenha o background do jogo
    pub fn draw_background(&self) {
        if let Some(texture) = &self.background_texture {
            let screen_width = screen_width();
            let screen_height = screen_height();
            
            // Calcular proporção para cobrir toda a tela
            let texture_width = texture.width();
            let texture_height = texture.height();
            let aspect_ratio = texture_width / texture_height;
            let screen_aspect_ratio = screen_width / screen_height;
            
            let (final_width, final_height, final_x, final_y) = if aspect_ratio > screen_aspect_ratio {
                // Imagem é mais larga, ajustar pela altura
                let new_height = screen_height;
                let new_width = screen_height * aspect_ratio;
                let new_x = (screen_width - new_width) / 2.0;
                (new_width, new_height, new_x, 0.0)
            } else {
                // Imagem é mais alta, ajustar pela largura
                let new_width = screen_width;
                let new_height = screen_width / aspect_ratio;
                let new_y = (screen_height - new_height) / 2.0;
                (new_width, new_height, 0.0, new_y)
            };
            
            draw_texture_ex(
                texture,
                final_x,
                final_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(final_width, final_height)),
                    ..Default::default()
                },
            );
        }
    }
}

