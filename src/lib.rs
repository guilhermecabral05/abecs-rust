// Biblioteca para comunicação com Pinpad via Protocolo ABECS 2.12

pub mod command;
pub mod connection;
pub mod error;
pub mod protocol;
pub mod response;

// Re-exporta os tipos principais para facilitar o uso
pub use command::AbecsCommand;
pub use connection::PinpadConnection;
pub use error::AbecsError;
pub use response::AbecsResponse;

/// Tipo Result padrão da biblioteca
pub type Result<T> = std::result::Result<T, AbecsError>;
