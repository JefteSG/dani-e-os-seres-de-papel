# Dani e os Seres de Papel 🃏✨

Um jogo de cartas estratégico desenvolvido em Rust usando a engine Macroquad.

## 📦 Tecnologias Utilizadas
- **Linguagem**: Rust (edição 2024)
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

## 📥 Download e Instalação

### 🪟 Windows

#### **Opção 1: Download da Release (Recomendado)**
1. **📥 Baixe**: Vá para [Releases](https://github.com/JefteSG/dani-e-os-seres-de-papel/releases)
2. **📦 Download**: Clique em `dani-seres-do-papel-windows.zip`
3. **📂 Extraia**: Extraia o arquivo ZIP
4. **🎮 Execute**: Clique duas vezes em `dani_seres_do_papel.exe`

⚠️ **Problema com Antivírus?** Se o Windows Defender bloquear o arquivo, veja [WINDOWS_ANTIVIRUS_FIX.md](WINDOWS_ANTIVIRUS_FIX.md) para soluções.

#### **Opção 2: Compilação Local**
```bash
# Instale o Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone o repositório
git clone https://github.com/JefteSG/dani-e-os-seres-de-papel.git
cd dani-e-os-seres-de-papel

# Compile para Windows
cargo build --release --target x86_64-pc-windows-msvc

# Execute
./target/x86_64-pc-windows-msvc/release/dani_seres_do_papel.exe
```

### 🐧 Linux

#### **Opção 1: Download da Release (Recomendado)**
1. **📥 Baixe**: Vá para [Releases](https://github.com/JefteSG/dani-e-os-seres-de-papel/releases)
2. **📦 Download**: Clique em `dani-seres-do-papel-linux.tar.gz`
3. **📂 Extraia**: 
   ```bash
   tar -xzf dani-seres-do-papel-linux.tar.gz
   ```
4. **🎮 Execute**: 
   ```bash
   cd dani-seres-do-papel
   chmod +x dani_seres_do_papel
   ./dani_seres_do_papel
   ```

#### **Opção 2: Compilação Local**
```bash
# Instale dependências do sistema
sudo apt-get update
sudo apt-get install -y libasound2-dev libx11-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev

# Instale o Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone o repositório
git clone https://github.com/JefteSG/dani-e-os-seres-de-papel.git
cd dani-e-os-seres-de-papel

# Compile
cargo build --release

# Execute
./target/release/dani_seres_do_papel
```

### 🍎 macOS

#### **Compilação Local**
```bash
# Instale o Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone o repositório
git clone https://github.com/JefteSG/dani-e-os-seres-de-papel.git
cd dani-e-os-seres-de-papel

# Compile
cargo build --release

# Execute
./target/release/dani_seres_do_papel
```

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

## 🚀 Como Executar (Desenvolvimento)

### Linux/macOS
1. Certifique-se de ter o Rust instalado
2. Clone este repositório
3. Execute:
   ```bash
   cargo run --release
   ```

### Windows
Para compilar para Windows a partir do Linux/macOS:

1. **Instale o Cross** (ferramenta de compilação cruzada):
   ```bash
   cargo install cross
   ```

2. **Certifique-se de que o Docker está rodando**

3. **Compile para Windows**:
   ```bash
   # Usando o script automático (recomendado)
   ./build-windows.sh
   
   # Ou manualmente
   cross build --target x86_64-pc-windows-gnu --release
   ```

4. **O executável será criado em**: `target/x86_64-pc-windows-gnu/release/dani_seres_do_papel.exe`

**Nota**: O target MSVC não é suportado pelo Cross e requer Visual Studio. Recomendamos usar o target GNU que é mais estável e compatível.

📖 **Documentação completa**: Veja [BUILD_WINDOWS.md](BUILD_WINDOWS.md) para instruções detalhadas.

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
