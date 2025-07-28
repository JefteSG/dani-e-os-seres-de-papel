use bracket_lib::prelude::*;
mod deck;
mod effects;
mod enemy;
mod entity;
mod gameturn;
mod player;
mod state;

fn main() -> BError {
    let context = BTermBuilder::simple80x50() // Tela com 80 colunas e 50 linhas
        .with_title("🃏 Dani e os Seres de Papel 🐉")
        .build()?; // Usa o operador ? para lidar com erros

    let gs = state::GameState::new(); // Inicializa o estado do jogo

    main_loop(context, gs) // Inicia o loop do jogo
}
