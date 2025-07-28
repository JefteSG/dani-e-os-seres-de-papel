use crate::deck::{CardType, Deck};
use crate::enemy::Enemy;
use crate::gameturn::GameTurn;
use crate::player::Player;
use crate::{effects::StatusEffect, entity::Entity};
use bracket_lib::prelude::*;
use bracket_lib::terminal::GameState as BracketGameState;
use rand::prelude::*;

pub enum MenuSelection {
    Start,
    Quit,
}

pub enum AppState {
    Menu,
    Battle(BattleState),
    GameOver,
}

pub struct GameState {
    pub app_state: AppState,
    selection: MenuSelection,
    pub winner: Option<String>,
}

pub struct BattleState {
    pub player: Player,
    pub enemy: Enemy,
    pub deck: Deck,
    pub turn: GameTurn,
    pub current_message: String,
    pub waiting_input: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            app_state: AppState::Menu,
            selection: MenuSelection::Start,
            winner: None,
        }
    }
}

impl BracketGameState for GameState {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        match &mut self.app_state {
            AppState::Menu => {
                ctx.print_color_centered(3, YELLOW, BLACK, "🃏 Dani e os Seres de Papel 🐉");
                ctx.print_color_centered(
                    5,
                    WHITE,
                    BLACK,
                    "Use ↑ e ↓ para navegar. Pressione Enter para selecionar.",
                );

                // Destaque visual para opções
                match self.selection {
                    MenuSelection::Start => {
                        ctx.print_color_centered(8, GREEN, BLACK, ">> Iniciar <<");
                        ctx.print_color_centered(10, GRAY, BLACK, "Sair");
                    }
                    MenuSelection::Quit => {
                        ctx.print_color_centered(8, GRAY, BLACK, "Iniciar");
                        ctx.print_color_centered(10, RED, BLACK, ">> Sair <<");
                    }
                }

                if let Some(key) = ctx.key {
                    match key {
                        VirtualKeyCode::Up | VirtualKeyCode::W => {
                            self.selection = MenuSelection::Start;
                        }
                        VirtualKeyCode::Down | VirtualKeyCode::S => {
                            self.selection = MenuSelection::Quit;
                        }
                        VirtualKeyCode::Return => match self.selection {
                            MenuSelection::Start => {
                                let battle = BattleState {
                                    player: Player::new("Jogador"),
                                    enemy: Enemy::new("Inimigo", 100, 10, 5, 10),
                                    deck: Deck::new(),
                                    turn: GameTurn::new(20),
                                    current_message: "Seu turno começou!".to_string(),
                                    waiting_input: true,
                                };
                                self.app_state = AppState::Battle(battle);
                            }
                            MenuSelection::Quit => ctx.quit(),
                        },
                        _ => {}
                    }
                }
            }

            AppState::Battle(battle) => {
                // HUD
                ctx.print(
                    1,
                    1,
                    format!(
                        "🧙 Player: ♥ {} 🗡 {} 🛡 {}",
                        battle.player.health, battle.player.attack, battle.player.defense
                    ),
                );
                ctx.print(
                    1,
                    2,
                    format!(
                        "👾 Inimigo: ♥ {} 🗡 {} 🛡 {}",
                        battle.enemy.health, battle.enemy.attack, battle.enemy.defense
                    ),
                );

                ctx.print_color(1, 4, YELLOW, BLACK, &battle.current_message);

                if battle.turn.turn_over() {
                    self.app_state = AppState::GameOver;
                    return;
                }

                if battle.turn.player_turn() {
                    if battle.waiting_input {
                        ctx.print_color_centered(
                            6,
                            WHITE,
                            BLACK,
                            "1 - Aumentar Ataque | 2 - Aumentar Defesa | 3 - Envenenar | 4 - Curar",
                        );

                        if let Some(key) = ctx.key {
                            match key {
                                VirtualKeyCode::Key1 => {
                                    battle.player.attack_up(5);
                                    battle.current_message =
                                        format!("Você aumentou o ataque em 5!");
                                }
                                VirtualKeyCode::Key2 => {
                                    battle.player.defense_up(5);
                                    battle.current_message =
                                        format!("Você aumentou a defesa em 5!");
                                }
                                VirtualKeyCode::Key3 => {
                                    battle.enemy.status_effect(StatusEffect::Poison, 2);
                                    battle.current_message = format!("Você envenenou o inimigo!");
                                }
                                VirtualKeyCode::Key4 => {
                                    battle.player.heal(10);
                                    battle.current_message = format!("Você se curou em 10!");
                                }
                                _ => return, // Espera tecla válida
                            }

                            battle.enemy.damage(battle.player.attack);
                            battle.enemy.apply_status_effects();
                            battle.waiting_input = false;
                            battle.turn.next_turn();
                            if battle.enemy.health <= 0 {
                                self.winner = Some("Jogador".to_string());
                                self.app_state = AppState::GameOver;
                                return;
                            }
                        }
                    } else {
                        ctx.print_centered(
                            6,
                            "Pressione qualquer tecla para jogar a próxima rodada...",
                        );
                        if ctx.key.is_some() {
                            battle.waiting_input = true;
                        }
                    }
                } else {
                    // Turno do inimigo
                    if let Some(card) = battle.deck.cards.choose(&mut rand::rng()).cloned() {
                        match card.card_type {
                            CardType::Attack(a) => {
                                battle.enemy.attack_up(a);
                                battle.current_message =
                                    format!("Inimigo aumentou ataque em {}!", a);
                            }
                            CardType::Defense(d) => {
                                battle.enemy.defense_up(d);
                                battle.current_message =
                                    format!("Inimigo aumentou defesa em {}!", d);
                            }
                            CardType::Poison(_) => {
                                battle.player.status_effect(StatusEffect::Poison, 2);
                                battle.current_message = "Inimigo aplicou veneno!".to_string();
                            }
                            CardType::Heal(h) => {
                                battle.enemy.heal(h);
                                battle.current_message = format!("Inimigo curou {}!", h);
                            }
                        }

                        battle.player.damage(battle.enemy.attack);
                        battle.enemy.apply_status_effects();
                    } else {
                        battle.current_message = "Deck inimigo vazio. Turno pulado.".to_string();
                    }

                    battle.turn.next_turn();
                    if battle.player.health <= 0 {
                        self.winner = Some("Inimigo".to_string());
                        self.app_state = AppState::GameOver;
                        return;
                    }
                }
            }

            AppState::GameOver => {
                ctx.print_color_centered(10, RED, BLACK, "🎮 Fim do jogo!");

                if let Some(ref winner) = self.winner {
                    ctx.print_color_centered(12, YELLOW, BLACK, format!("🏆 {} venceu!", winner));
                }

                ctx.print_color_centered(
                    14,
                    WHITE,
                    BLACK,
                    "Pressione ESC para voltar ao menu principal.",
                );

                if let Some(key) = ctx.key {
                    // voltar ao menu principal
                    if key == VirtualKeyCode::Escape {
                        self.app_state = AppState::Menu;
                    }
                }
            }
        }
    }
}
