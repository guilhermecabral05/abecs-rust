// Biblioteca para comunicação com Pinpad via Protocolo ABECS 2.12

pub mod command;
pub mod commands;
pub mod connection;
pub mod error;
pub mod protocol;
pub mod response;
pub mod serialize;

// Re-exporta os tipos principais para facilitar o uso
pub use command::AbecsCommand as RawAbecsCommand;
pub use commands::AbecsCommand; // Novo namespace de comandos tipados
pub use connection::PinpadConnection;
pub use error::AbecsError;
pub use response::AbecsResponse;
pub use serialize::{AbecsDeserialize, AbecsSerialize, AbecsTypedCommand};

// Re-exporta as respostas
pub use commands::{
    EmptyResponse, GetDataResponse, GetInfoResponse, GetKeyResponse, GetPinResponse, MenuResponse,
};

/// Tipo Result padrão da biblioteca
pub type Result<T> = std::result::Result<T, AbecsError>;
