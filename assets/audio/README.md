# Assets de Áudio - Dani e os Seres de Papel 🎵

Este diretório contém todos os arquivos de áudio do jogo.

## Estrutura de Diretórios

```
assets/audio/
├── music/
│   └── background.ogg      # Música de fundo da batalha
└── sfx/
    ├── card_use.wav        # Som ao usar uma carta
    └── enemy_attack.wav    # Som do ataque do inimigo
```

## Formatos Suportados

- **Música**: `.ogg` (recomendado para música de fundo)
- **Efeitos Sonoros**: `.wav` (recomendado para efeitos rápidos)

## Como Adicionar Áudios

### 1. Música de Fundo
- Substitua o arquivo `music/background.ogg`
- Recomendado: música instrumental, loop suave
- Volume será automaticamente ajustado para 30%

### 2. Som de Carta
- Substitua o arquivo `sfx/card_use.wav`
- Recomendado: som curto e claro (~0.5s)
- Exemplo: "swoosh", "magic", "paper"

### 3. Som de Ataque do Inimigo
- Substitua o arquivo `sfx/enemy_attack.wav`
- Recomendado: som intimidador (~1s)
- Exemplo: rugido, golpe, magia sombria

## Fontes de Áudio Gratuitas

### Música:
- [Freesound.org](https://freesound.org/)
- [Zapsplat](https://www.zapsplat.com/)
- [OpenGameArt.org](https://opengameart.org/)

### Efeitos Sonoros:
- [BBC Sound Effects](https://sound-effects.bbcrewind.co.uk/)
- [Adobe Audition (samples gratuitos)](https://www.adobe.com/products/audition.html)

## Dicas Técnicas

- **Tamanho**: Mantenha arquivos pequenos (<5MB para música, <1MB para SFX)
- **Qualidade**: 44.1kHz, 16-bit é suficiente
- **Loop**: Para música de fundo, certifique-se que o loop é suave
- **Volume**: Normalize os áudios para evitar picos

## Controle de Volume

Os volumes estão configurados no arquivo `src/deck.rs` através de constantes:

```rust
const BACKGROUND_MUSIC_VOLUME: f32 = 0.3;  // Música de fundo (30%)
const CARD_USE_VOLUME: f32 = 0.3;          // Som das cartas (30%)
const ENEMY_ATTACK_VOLUME: f32 = 0.4;      // Som do inimigo (40%)
```

### Como Ajustar Volumes:

1. **Abra**: `src/deck.rs`
2. **Localize**: As constantes no topo do arquivo
3. **Modifique**: Os valores (0.0 = silêncio, 1.0 = volume máximo)
4. **Compile**: `cargo build` para aplicar mudanças

### Volumes Recomendados:
- **Música de fundo**: 0.2 - 0.4 (não deve atrapalhar)
- **Efeitos sonoros**: 0.3 - 0.6 (deve ser audível mas não alto)
- **Sons de combate**: 0.4 - 0.7 (pode ser mais dramático)

## Status Atual

✅ **Feature de Áudio Habilitada**: A feature "audio" do macroquad está ativa no projeto.
✅ **Arquivos de Áudio Válidos**: Arquivos silenciosos criados para funcionamento.
✅ **Controle de Volume**: Volumes ajustáveis através de constantes.

O jogo funcionará normalmente mesmo sem áudio - há fallbacks implementados.

## Configuração Técnica

A feature de áudio está habilitada no `Cargo.toml`:
```toml
macroquad = { version = "0.4.14", features = ["audio"] }
```

Isso permite usar todas as funções de áudio do macroquad sem warnings.