// Biblioteca para comunicação com Pinpad via Protocolo ABECS 2.12

pub mod command;
pub mod commands;
pub mod connection;
pub mod error;
pub mod protocol;
pub mod response;
pub mod serialize;

// Re-exporta os tipos principais para facilitar o uso
pub use command::AbecsCommand;
pub use connection::PinpadConnection;
pub use error::AbecsError;
pub use response::AbecsResponse;
pub use serialize::{AbecsDeserialize, AbecsSerialize, AbecsTypedCommand};

// Re-exporta os comandos tipados
pub use commands::*;

/// Tipo Result padrão da biblioteca
pub type Result<T> = std::result::Result<T, AbecsError>;
