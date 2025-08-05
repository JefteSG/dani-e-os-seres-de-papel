use crate::deck::{Card, Hand};
use crate::effects::StatusEffect;

pub trait Entity {
    fn discart_card(&mut self, hand: &mut Hand, index: usize) -> Option<Card>;
    fn damage(&mut self, damage: u32);
    fn defense_up(&mut self, defense: u32);
    fn attack_up(&mut self, attack: u32);
    fn heal(&mut self, heal: u32);
    fn status_effect(&mut self, status_effect: StatusEffect, duration: u32);
    fn apply_status_effects(&mut self);
}
