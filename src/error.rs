/// Tipos de erro da biblioteca ABECS
#[derive(Debug, Clone)]
pub enum AbecsError {
    /// Erro de comunicação serial
    SerialError(String),

    /// Erro no protocolo (CRC, formato, etc)
    ProtocolError(String),

    /// Timeout na comunicação
    Timeout(String),

    /// NAK recebido do Pinpad
    NakReceived(String),

    /// Resposta inválida do Pinpad
    InvalidResponse(String),

    /// Comando inválido
    InvalidCommand(String),

    /// Operação cancelada pelo usuário (botão vermelho pressionado)
    UserCancelled,

    /// Erro retornado pelo Pinpad (status != 000)
    PinpadError { status: String, description: String },
}

impl std::fmt::Display for AbecsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbecsError::SerialError(msg) => write!(f, "Erro de comunicação serial: {}", msg),
            AbecsError::ProtocolError(msg) => write!(f, "Erro no protocolo: {}", msg),
            AbecsError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            AbecsError::NakReceived(msg) => write!(f, "NAK recebido: {}", msg),
            AbecsError::InvalidResponse(msg) => write!(f, "Resposta inválida: {}", msg),
            AbecsError::InvalidCommand(msg) => write!(f, "Comando inválido: {}", msg),
            AbecsError::UserCancelled => write!(f, "Operação cancelada pelo usuário"),
            AbecsError::PinpadError {
                status,
                description,
            } => {
                write!(f, "Erro do Pinpad [{}]: {}", status, description)
            }
        }
    }
}

impl std::error::Error for AbecsError {}

/// Converte String em AbecsError
impl From<String> for AbecsError {
    fn from(s: String) -> Self {
        if s.contains("Timeout") || s.contains("timeout") {
            AbecsError::Timeout(s)
        } else if s.contains("NAK") {
            AbecsError::NakReceived(s)
        } else if s.contains("serial") || s.contains("porta") {
            AbecsError::SerialError(s)
        } else {
            AbecsError::ProtocolError(s)
        }
    }
}
