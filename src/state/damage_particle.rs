use macroquad::prelude::*;

#[derive(Clone)]
pub struct DamageParticle {
    pub x: f32,
    pub y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub life: f32,
    pub max_life: f32,
    pub damage: u32,
}

impl DamageParticle {
    pub fn new(x: f32, y: f32, damage: u32) -> Self {
        use ::rand::Rng;
        let mut rng = ::rand::thread_rng();
        Self {
            x,
            y,
            velocity_x: rng.gen_range(-50.0..50.0),
            velocity_y: rng.gen_range(-100.0..-50.0),
            life: 2.0,
            max_life: 2.0,
            damage,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.x += self.velocity_x * dt;
        self.y += self.velocity_y * dt;
        self.velocity_y += 200.0 * dt; // Gravidade
        self.life -= dt;
    }

    pub fn is_alive(&self) -> bool {
        self.life > 0.0
    }

    pub fn draw(&self) {
        let alpha = (self.life / self.max_life).min(1.0);
        let color = Color::new(1.0, 0.3, 0.3, alpha);
        let damage_text = format!("-{}", self.damage);
        draw_text(&damage_text, self.x, self.y, 20.0, color);
    }
}