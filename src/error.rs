use std::fmt;

/// Tipos de erros específicos do jogo
#[derive(Debug, Clone)]
pub enum GameError {
    AssetLoadError(String),
    SaveLoadError(String),
    AudioError(String),
    InvalidGameState(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::AssetLoadError(msg) => write!(f, "Asset loading error: {}", msg),
            GameError::SaveLoadError(msg) => write!(f, "Save/load error: {}", msg),
            GameError::AudioError(msg) => write!(f, "Audio error: {}", msg),
            GameError::InvalidGameState(msg) => write!(f, "Invalid game state: {}", msg),
        }
    }
}

impl std::error::Error for GameError {}

pub type GameResult<T> = Result<T, GameError>;