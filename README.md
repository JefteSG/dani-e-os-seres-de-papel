# Dani e os Seres de Papel 🃏✨

Um jogo de cartas estratégico desenvolvido em Rust usando a engine Macroquad.

## 📦 Tecnologias Utilizadas
- **Linguagem**: Rust (edição 2021)
- **Engine**: Macroquad (com suporte a áudio)
- **Dependências**:
  - `rand` para geração de números aleatórios

## 🎮 Sobre o Jogo
"Dani e os Seres de Papel" é um jogo de turnos onde o jogador enfrenta inimigos usando um deck de cartas estratégicas. O jogo possui:

- **Sistema de cooldown** entre turnos para criar ritmo estratégico
- **Diferentes tipos de cartas**: ataque, defesa, veneno e cura
- **Inimigos variados** com comportamentos distintos
- **Sistema de partículas** para feedback visual
- **Persistência de progresso** via arquivo `save_game.json`

## ⚙️ Sistema de Cooldown
O jogo utiliza um sistema de espera entre turnos para melhorar a jogabilidade:

| Turno       | Tempo Padrão | Descrição                     |
|-------------|--------------|-------------------------------|
| **Jogador** | 1.0 segundos | Após usar uma carta           |
| **Inimigo** | 1.0 segundo  | Após o inimigo realizar ação  |

Os tempos podem ser ajustados no código fonte conforme necessidade.

## 🎵 Assets
O projeto inclui:
- **Efeitos sonoros** para cartas e ações
- **Música de fundo** em formato .ogg
- **Arte de cartas** e inimigos em PNG
- **Fontes de emoji** para elementos visuais

## 🚀 Como Executar
1. Certifique-se de ter o Rust instalado
2. Clone este repositório
3. Execute:
   ```bash
   cargo run --release
   ```

## 📂 Estrutura do Projeto
```
src/
├── main.rs            # Ponto de entrada
├── deck.rs            # Sistema de cartas
├── effects.rs         # Efeitos de jogo
├── enemy.rs           # Lógica dos inimigos
├── entity.rs          # Entidades do jogo
├── gameturn.rs        # Sistema de turnos
├── player.rs          # Lógica do jogador
└── state/             # Máquina de estados do jogo
```

## ⚖️ Balanceamento
O jogo foi projetado para permitir fácil ajuste de:
- Tempos de cooldown
- Dano das cartas
- Comportamento dos inimigos
- Velocidade do jogo

## 📜 Licença
Este projeto utiliza fontes sob licença Open Font License (OFL).

---

Desenvolvido com ❤️ usando Rust e Macroquad
