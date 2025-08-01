use crate::deck::{Card, Deck, Hand};
use crate::effects::StatusEffect;
use crate::entity::Entity;
use std::collections::HashMap;

pub struct Enemy {
    pub name: String,
    pub hand: Hand,
    pub health: u32,
    pub max_health: u32,
    pub attack: u32,
    pub defense: u32,
    pub status_effects: HashMap<StatusEffect, u32>,
    pub image_path: String,
}

impl Enemy {
    pub fn new(
        name: &str,
        health: u32,
        max_health: u32,
        attack: u32,
        defense: u32,
        deck: &mut Deck,
        image_path: &str,
    ) -> Self {
        deck.shuffle();
        Self {
            name: name.to_string(),
            hand: Hand::new_from_deck(deck, 5),
            health: health,
            max_health: max_health,
            attack: attack,
            defense: defense,
            status_effects: HashMap::new(),
            image_path: image_path.to_string(),
        }
    }
}

impl Entity for Enemy {
    fn discart_card(&mut self, hand: &mut Hand, index: usize) -> Option<Card> {
        hand.discard(index)
    }

    fn damage(&mut self, damage: u32) {
        let damage_final = damage.saturating_sub(self.defense);
        self.health = self.health.saturating_sub(damage_final);
    }
    fn heal(&mut self, heal: u32) {
        self.health += heal;
    }

    fn attack_up(&mut self, attack: u32) {
        self.attack += attack;
    }

    fn defense_up(&mut self, defense: u32) {
        self.defense += defense;
    }

    fn status_effect(&mut self, status_effect: StatusEffect, duration: u32) {
        self.status_effects.insert(status_effect, duration);
    }

    fn apply_status_effects(&mut self) {
        let mut expired_effects = vec![];

        for (effect, duration) in self.status_effects.iter_mut() {
            match effect {
                StatusEffect::Poison => {
                    println!("{} sofre 5 de dano por envenenamento!", self.name);
                    self.health = self.health.saturating_sub(5);
                }
            }

            *duration = duration.saturating_sub(1);
            if *duration == 0 {
                expired_effects.push(*effect);
            }
        }

        for effect in expired_effects {
            println!("{:?} em {} terminou.", effect, self.name);
            self.status_effects.remove(&effect);
        }
    }
}
