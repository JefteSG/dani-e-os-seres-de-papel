use crate::deck::{Card, Deck, Hand};
use crate::effects::StatusEffect;
use crate::entity::Entity;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub hand: Hand,
    pub health: u32,
    pub max_health: u32,
    pub attack: u32,
    pub defense: u32,
    pub status_effects: HashMap<StatusEffect, u32>,
}

impl Player {
    pub fn new(name: &str, deck: &mut Deck) -> Player {
        deck.shuffle();
        Player {
            name: name.to_string(),
            hand: Hand::new_from_deck(deck, 5),
            health: 100,
            max_health: 100,
            attack: 10,
            defense: 8,
            status_effects: HashMap::new(),
        }
    }
}

impl Entity for Player {
    fn discart_card(&mut self, hand: &mut Hand, index: usize) -> Option<Card> {
        hand.discard(index)
    }

    fn damage(&mut self, damage: u32) {
        let damage_final = damage.saturating_sub(self.defense);
        self.health = self.health.saturating_sub(damage_final);
    }

    fn defense_up(&mut self, defense: u32) {
        self.defense += defense;
    }

    fn attack_up(&mut self, attack: u32) {
        self.attack += attack;
    }

    fn heal(&mut self, heal: u32) {
        // só pode curar até a vida máxima
        self.health = self.health.saturating_add(heal);
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }

    fn status_effect(&mut self, status_effect: StatusEffect, duration: u32) {
        // Permitir acumulação de status effects
        if let Some(existing_duration) = self.status_effects.get_mut(&status_effect) {
            *existing_duration += duration; // Acumular duração
        } else {
            self.status_effects.insert(status_effect, duration);
        }
    }

    fn apply_status_effects(&mut self) {
        let mut expired_effects = vec![];

        for (effect, duration) in self.status_effects.iter_mut() {
            match effect {
                StatusEffect::Poison => {
                    println!("{} sofre 5 de dano por envenenamento!", self.name);
                    self.health = self.health.saturating_sub(5);
                }
                StatusEffect::Burn => {
                    // queimadura respeita defesa porem da o dobro de dano
                    let damage_final = 2 * self.attack.saturating_sub(self.defense);
                    self.health = self.health.saturating_sub(damage_final);
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
    // fn is_dead(&mut self){
    //     self.
    // }
}
