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
    pub level: u32,
    pub experience: u32,
    pub experience_to_next_level: u32,
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
            level: 1,
            experience: 0,
            experience_to_next_level: 100,
            status_effects: HashMap::new(),
        }
    }

    pub fn gain_experience(&mut self, exp: u32) -> bool {
        self.experience += exp;
        
        if self.experience >= self.experience_to_next_level {
            self.level_up();
            return true;
        }
        false
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        
        self.experience_to_next_level = self.level * 100 + (self.level - 1) * 50;
        
        let health_bonus = 25;
        
        self.max_health += health_bonus;
        self.health = self.max_health;
        
        println!("🎉 {} leveled up to level {}!", self.name, self.level);
        println!("   HP: {} | ATK: {} | DEF: {}", self.max_health, self.attack, self.defense);
        println!("   💚 +{} max HP (attack and defense come from cards)", health_bonus);
    }

    pub fn get_experience_progress(&self) -> f32 {
        if self.experience_to_next_level == 0 {
            return 1.0;
        }
        (self.experience as f32) / (self.experience_to_next_level as f32)
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
        self.health = self.health.saturating_add(heal);
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }

    fn status_effect(&mut self, status_effect: StatusEffect, duration: u32) {
        if let Some(existing_duration) = self.status_effects.get_mut(&status_effect) {
            *existing_duration += duration;
        } else {
            self.status_effects.insert(status_effect, duration);
        }
    }

    fn apply_status_effects(&mut self) {
        let mut expired_effects = vec![];

        for (effect, duration) in self.status_effects.iter_mut() {
            match effect {
                StatusEffect::Poison => {
                    println!("{} takes 5 damage from poison!", self.name);
                    self.health = self.health.saturating_sub(5);
                }
                StatusEffect::Burn => {
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
            println!("{:?} on {} ended.", effect, self.name);
            self.status_effects.remove(&effect);
        }
    }
}
