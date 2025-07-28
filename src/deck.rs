use rand::prelude::SliceRandom;
use rand::rng;

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
}

impl Card {
    pub fn new(name: &str, card_type: CardType) -> Self {
        Self {
            name: name.to_string(),
            card_type,
        }
    }
}

/// Baralho do jogador
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = vec![
            Card::new("Ataque", CardType::Attack(10)),
            Card::new("Defesa", CardType::Defense(8)),
            Card::new("Veneno", CardType::Poison(4)),
            Card::new("Cura", CardType::Heal(6)),
        ];

        // Duplicar cartas para um baralho maior
        cards = cards.into_iter().cycle().take(20).collect();

        Self { cards }
    }

    /// misturar as cartas
    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

/// Mão do jogador
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Self { cards: vec![] }
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
