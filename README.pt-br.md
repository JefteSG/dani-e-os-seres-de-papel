# Dani e os Seres de Papel 🃏✨

> 🇺🇸 [Read in English](README.md)

Um jogo estratégico de cartas desenvolvido em Rust usando o engine Macroquad.

## 💝 Origem do Projeto

Este projeto nasceu de duas paixões: **aprender Rust** e **se divertir criando**!

Os monstros e criaturas que você vai encontrar no jogo foram desenhados pelo meu filho **Daniel** — daí o nome especial: **"Dani e os Seres de Papel"** 🎨✨

É um projeto em família que une programação, criatividade e muito amor! 💕

## 📦 Tecnologias Utilizadas
- **Linguagem**: Rust (edição 2024)
- **Engine**: Macroquad (com suporte a áudio)
- **Dependências**:
  - `rand` para geração de números aleatórios

## 🎮 Sobre o Jogo
"Dani e os Seres de Papel" é um jogo por turnos onde o jogador enfrenta inimigos usando um baralho de cartas estratégicas. O jogo conta com:

- **Sistema de cooldown** entre turnos para criar ritmo estratégico
- **Diferentes tipos de carta**: ataque, defesa, veneno e cura
- **Inimigos variados** com comportamentos distintos
- **Sistema de partículas** para feedback visual
- **Persistência de progresso** via arquivo `save_game.json`

## 📸 Capturas de Tela

### 🏠 Menu Principal
![Menu Principal](tela_inicial.png)

### 👥 Seleção de Personagens
![Seleção de Personagens](selecao_personagens.png)

### ⚙️ Configurações
![Configurações](config_sinal.png)

### ⚔️ Tela de Batalha
![Tela de Batalha](luta.png)

### 🎥 Vídeo de Gameplay
[🎮 **Assistir Gameplay**](https://youtu.be/OClSfvBDb6o)

*Veja o jogo em ação e descubra estratégias para vencer!*

## 📥 Download e Instalação

### 🪟 Windows

#### **Opção 1: Download pelo Releases (Recomendado)**
1. **📥 Acesse**: [Releases](https://github.com/JefteSG/dani-e-os-seres-de-papel/releases)
2. **📦 Baixe**: Clique em `dani-seres-do-papel-windows.zip`
3. **📂 Extraia**: Extraia o arquivo ZIP
4. **🎮 Execute**: Dê dois cliques em `dani_seres_do_papel.exe`

⚠️ **Problema com antivírus?** Se o Windows Defender bloquear o arquivo, consulte [WINDOWS_ANTIVIRUS_FIX.md](WINDOWS_ANTIVIRUS_FIX.md) para soluções.

#### **Opção 2: Compilação Local**
```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clonar o repositório
git clone https://github.com/JefteSG/dani-e-os-seres-de-papel.git
cd dani-e-os-seres-de-papel

# Compilar para Windows
cargo build --release --target x86_64-pc-windows-msvc

# Executar
./target/x86_64-pc-windows-msvc/release/dani_seres_do_papel.exe
```

### 🐧 Linux

#### **Opção 1: Download pelo Releases (Recomendado)**
1. **📥 Acesse**: [Releases](https://github.com/JefteSG/dani-e-os-seres-de-papel/releases)
2. **📦 Baixe**: Clique em `dani-seres-do-papel-linux.tar.gz`
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
# Instalar dependências do sistema
sudo apt-get update
sudo apt-get install -y libasound2-dev libx11-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev

# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clonar o repositório
git clone https://github.com/JefteSG/dani-e-os-seres-de-papel.git
cd dani-e-os-seres-de-papel

# Compilar
cargo build --release

# Executar
./target/release/dani_seres_do_papel
```

### 🍎 macOS

#### **Compilação Local**
```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clonar o repositório
git clone https://github.com/JefteSG/dani-e-os-seres-de-papel.git
cd dani-e-os-seres-de-papel

# Compilar
cargo build --release

# Executar
./target/release/dani_seres_do_papel
```

## 🤖 CI/CD e Releases Automatizados

O projeto usa GitHub Actions para integração contínua e publicação automática de releases:

| Workflow | Gatilho | Finalidade |
|----------|---------|------------|
| **Rust Build and Test** | Push / PR na `main` | Compila e roda testes no Linux e Windows |
| **Development Build** | Push na `main` ou disparo manual | Gera pacotes de desenvolvimento para Windows e Linux (artefatos mantidos por 7 dias) |
| **Build and Release** | Tag `v*` ou disparo manual | Gera os pacotes de release e publica no GitHub Releases |

### 🚀 Publicando uma Nova Versão

Para publicar uma nova versão, crie uma tag no formato `v1.0.0`:

```bash
git tag v1.0.0
git push origin v1.0.0
```

O workflow **Build Windows MSVC and Linux** vai automaticamente:
- Compilar o executável Windows e empacotar em `dani-seres-do-papel-windows.zip`
- Compilar o executável Linux e empacotar em `dani-seres-do-papel-linux.tar.gz`
- Publicar um GitHub Release com os dois arquivos anexados

## ⚙️ Sistema de Cooldown
O jogo usa um sistema de espera entre os turnos para melhorar a jogabilidade:

| Turno       | Tempo Padrão | Descrição                          |
|-------------|--------------|-------------------------------------|
| **Jogador** | 1,0 segundo  | Após usar uma carta                 |
| **Inimigo** | 1,0 segundo  | Após o inimigo realizar uma ação    |

Os tempos podem ser ajustados no código-fonte conforme necessário.

## 🎵 Assets
O projeto inclui:
- **Efeitos sonoros** para cartas e ações
- **Música de fundo** em formato .ogg
- **Arte de cartas e inimigos** em PNG
- **Fontes de emoji** para elementos visuais

## 🚀 Como Executar (Desenvolvimento)

### Linux/macOS
1. Certifique-se de que o Rust está instalado
2. Clone este repositório
3. Execute:
   ```bash
   cargo run --release
   ```

### Windows
Para compilar para Windows a partir de Linux/macOS:

1. **Instale o Cross** (ferramenta de compilação cruzada):
   ```bash
   cargo install cross
   ```

2. **Certifique-se de que o Docker está rodando**

3. **Compile para Windows**:
   ```bash
   # Usando script automático (recomendado)
   ./build-windows.sh

   # Ou manualmente
   cross build --target x86_64-pc-windows-gnu --release
   ```

4. **O executável será criado em**: `target/x86_64-pc-windows-gnu/release/dani_seres_do_papel.exe`

**Nota**: O alvo MSVC não é suportado pelo Cross e exige o Visual Studio. Recomendamos o alvo GNU, que é mais estável e compatível.

## 📂 Estrutura do Projeto
```
src/
├── main.rs            # Ponto de entrada
├── deck.rs            # Sistema de cartas
├── effects.rs         # Efeitos do jogo
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
Este projeto utiliza fontes sob a Open Font License (OFL).

