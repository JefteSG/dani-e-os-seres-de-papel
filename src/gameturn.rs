// o turno é dividido em 2 partes: o turno do jogador e o turno do inimigo
// o jogador começa primeiro
// depois o inimigo, por hora o inimigo vai atacar apenas o jogador

// use crate::enemy::Enemy;
// use crate::player::Player;
// use crate::deck::Deck;

pub struct GameTurn {
    pub turn: u32,
    pub max_turn: u32,
}

impl GameTurn {
    pub fn new(max_turn: u32) -> Self {
        Self { turn: 0, max_turn }
    }
    pub fn next_turn(&mut self) {
        self.turn += 1;
    }

    pub fn reset_turn(&mut self) {
        self.turn = 0;
    }

    pub fn turn_over(&self) -> bool {
        self.turn >= self.max_turn
    }

    pub fn player_turn(&self) -> bool {
        self.turn % 2 == 0
    }

    pub fn enemy_turn(&self) -> bool {
        self.turn % 2 == 1
    }
}

// fn main() {
//     // iniciar um player
//     // iniciar um inimigo
//     // iniciar um deck
//     // iniciar um game turn
//     let mut player = Player::new("Jogador");
//     let mut enemy = Enemy::new("Inimigo", 100, 10, 5, 100);
//     let mut deck = Deck::new();
//     let mut game_turn = GameTurn::new(5);

//     loop {
//         if game_turn.turn_over() {
//             break;
//         }
//         if game_turn.player_turn() {
//             println!("Turno do jogador");
//             println!("Vida do jogador: {}", player.health);
//             println!("Vida do inimigo: {}", enemy.health);
//             println!("Cartas na mão: {}", deck.cards.len());
//             println("Escolha uma carta:");
//             println("1 - Ataque");
//             println("2 - Defesa");
//             println("3 - Veneno");
//             println("4 - Cura");
//             let mut choice = String::new();
//             std::io::stdin().read_line(&mut choice).expect("Failed to read line");
//             let choice = choice.trim().parse().expect("Failed to parse choice");
//             match choice {
//                 1 => player.attack_up(5),
//                 2 => player.defense_up(5),
//                 3 => player.damage(5),
//                 4 => player.heal(5),
//                 _ => println!("Opção inválida"),
//             }
//         }
//         if game_turn.enemy_turn() {
//             enemy.attack_up(5);
//         }
//         game_turn.next_turn();
//     }

// }
