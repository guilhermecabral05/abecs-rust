/// Comandos ABECS pré-definidos com tipos seguros
use crate::response::AbecsResponse;
use crate::serialize::{AbecsDeserialize, AbecsSerialize, AbecsTypedCommand};

// ═══════════════════════════════════════════════════════════════════════════
// Respostas
// ═══════════════════════════════════════════════════════════════════════════

/// Resposta genérica para comandos sem dados de retorno
#[derive(Debug, Clone)]
pub struct EmptyResponse;

/// Resposta do comando GetInfo
#[derive(Debug, Clone)]
pub struct GetInfoResponse {
    pub info: String,
}

/// Resposta do comando GetPin
#[derive(Debug, Clone)]
pub struct GetPinResponse {
    pub pin_block: Vec<u8>,
}

/// Resposta do comando GetData
#[derive(Debug, Clone)]
pub struct GetDataResponse {
    pub data: String,
}

/// Resposta do comando Menu
#[derive(Debug, Clone)]
pub struct MenuResponse {
    pub selected_index: u8,
}

/// Resposta do comando GetKey
#[derive(Debug, Clone)]
pub struct GetKeyResponse {
    pub key_check_value: Vec<u8>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Namespace AbecsCommand - módulo com submódulos para cada comando
// ═══════════════════════════════════════════════════════════════════════════

#[allow(non_snake_case)]
pub mod AbecsCommand {
    use super::*;

    // ═══════════════════════════════════════════════════════════════════════
    // Open - Abertura de Sessão (OPN)
    // ═══════════════════════════════════════════════════════════════════════

    #[derive(Debug, Clone)]
    pub struct Open;

    impl Open {
        pub fn new() -> Self {
            Self
        }
    }

    impl AbecsTypedCommand for Open {
        type Response = EmptyResponse;

        fn command_id(&self) -> &str {
            "OPN"
        }

        fn serialize_params(&self) -> Vec<Vec<u8>> {
            Vec::new()
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Close - Fechamento de Sessão (CLO)
    // ═══════════════════════════════════════════════════════════════════════

    #[derive(Debug, Clone)]
    pub struct Close;

    impl Close {
        pub fn new() -> Self {
            Self
        }
    }

    impl AbecsTypedCommand for Close {
        type Response = EmptyResponse;

        fn command_id(&self) -> &str {
            "CLO"
        }

        fn serialize_params(&self) -> Vec<Vec<u8>> {
            Vec::new()
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Display - Mostrar Mensagem (DSP)
    // ═══════════════════════════════════════════════════════════════════════

    #[derive(Debug, Clone)]
    pub struct Display {
        pub message: String,
    }

    impl Display {
        pub fn new(message: impl Into<String>) -> Self {
            Self {
                message: message.into(),
            }
        }
    }

    impl AbecsTypedCommand for Display {
        type Response = EmptyResponse;

        fn command_id(&self) -> &str {
            "DSP"
        }

        fn serialize_params(&self) -> Vec<Vec<u8>> {
            vec![self.message.serialize_abecs()]
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // ClearDisplay - Limpar Display (CLX)
    // ═══════════════════════════════════════════════════════════════════════

    #[derive(Debug, Clone)]
    pub struct ClearDisplay;

    impl ClearDisplay {
        pub fn new() -> Self {
            Self
        }
    }

    impl AbecsTypedCommand for ClearDisplay {
        type Response = EmptyResponse;

        fn command_id(&self) -> &str {
            "CLX"
        }

        fn serialize_params(&self) -> Vec<Vec<u8>> {
            Vec::new()
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // GetInfo - Obter Informações (GIN)
    // ═══════════════════════════════════════════════════════════════════════

    #[derive(Debug, Clone)]
    pub struct GetInfo {
        pub info_type: String,
    }

    impl GetInfo {
        pub fn new(info_type: impl Into<String>) -> Self {
            Self {
                info_type: info_type.into(),
            }
        }
    }

    impl AbecsTypedCommand for GetInfo {
        type Response = GetInfoResponse;

        fn command_id(&self) -> &str {
            "GIN"
        }

        fn serialize_params(&self) -> Vec<Vec<u8>> {
            vec![self.info_type.serialize_abecs()]
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // GetPin - Obter PIN (GPN)
    // ═══════════════════════════════════════════════════════════════════════

    #[derive(Debug, Clone)]
    pub struct GetPin {
        pub message: String,
        pub min_length: u8,
        pub max_length: u8,
        pub timeout: u16,
        pub crypto_type: String,
        pub additional_data: String,
    }

    impl GetPin {
        pub fn new(
            message: impl Into<String>,
            min_length: u8,
            max_length: u8,
            timeout: u16,
            crypto_type: impl Into<String>,
            additional_data: impl Into<String>,
        ) -> Self {
            Self {
                message: message.into(),
                min_length,
                max_length,
                timeout,
                crypto_type: crypto_type.into(),
                additional_data: additional_data.into(),
            }
        }
    }

    impl AbecsTypedCommand for GetPin {
        type Response = GetPinResponse;

        fn command_id(&self) -> &str {
            "GPN"
        }

        fn serialize_params(&self) -> Vec<Vec<u8>> {
            vec![
                self.message.serialize_abecs(),
                vec![self.min_length],
                vec![self.max_length],
                format!("{:04}", self.timeout).serialize_abecs(),
                self.crypto_type.serialize_abecs(),
                self.additional_data.serialize_abecs(),
            ]
        }

        fn is_blocking(&self) -> bool {
            true
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // GetData - Obter Dados (GDU)
    // ═══════════════════════════════════════════════════════════════════════

    #[derive(Debug, Clone)]
    pub struct GetData {
        pub message: String,
        pub min_length: u8,
        pub max_length: u8,
        pub timeout: u16,
    }

    impl GetData {
        pub fn new(
            message: impl Into<String>,
            min_length: u8,
            max_length: u8,
            timeout: u16,
        ) -> Self {
            Self {
                message: message.into(),
                min_length,
                max_length,
                timeout,
            }
        }
    }

    impl AbecsTypedCommand for GetData {
        type Response = GetDataResponse;

        fn command_id(&self) -> &str {
            "GDU"
        }

        fn serialize_params(&self) -> Vec<Vec<u8>> {
            vec![
                self.message.serialize_abecs(),
                vec![self.min_length],
                vec![self.max_length],
                format!("{:04}", self.timeout).serialize_abecs(),
            ]
        }

        fn is_blocking(&self) -> bool {
            true
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Menu - Menu de Seleção (MNU)
    // ═══════════════════════════════════════════════════════════════════════

    #[derive(Debug, Clone)]
    pub struct Menu {
        pub title: String,
        pub options: Vec<String>,
        pub timeout: u16,
    }

    impl Menu {
        pub fn new(title: impl Into<String>, options: Vec<String>, timeout: u16) -> Self {
            Self {
                title: title.into(),
                options,
                timeout,
            }
        }
    }

    impl AbecsTypedCommand for Menu {
        type Response = MenuResponse;

        fn command_id(&self) -> &str {
            "MNU"
        }

        fn serialize_params(&self) -> Vec<Vec<u8>> {
            let mut params = vec![
                self.title.serialize_abecs(),
                format!("{:04}", self.timeout).serialize_abecs(),
                vec![self.options.len() as u8],
            ];

            for option in &self.options {
                params.push(option.serialize_abecs());
            }

            params
        }

        fn is_blocking(&self) -> bool {
            true
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // TableLoadInit - Iniciar Carga de Tabela (TLI)
    // ═══════════════════════════════════════════════════════════════════════

    #[derive(Debug, Clone)]
    pub struct TableLoadInit {
        pub table_id: String,
    }

    impl TableLoadInit {
        pub fn new(table_id: impl Into<String>) -> Self {
            Self {
                table_id: table_id.into(),
            }
        }
    }

    impl AbecsTypedCommand for TableLoadInit {
        type Response = EmptyResponse;

        fn command_id(&self) -> &str {
            "TLI"
        }

        fn serialize_params(&self) -> Vec<Vec<u8>> {
            vec![self.table_id.serialize_abecs()]
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // TableLoadRecord - Carregar Registro de Tabela (TLR)
    // ═══════════════════════════════════════════════════════════════════════

    #[derive(Debug, Clone)]
    pub struct TableLoadRecord {
        pub record_data: Vec<u8>,
    }

    impl TableLoadRecord {
        pub fn new(record_data: Vec<u8>) -> Self {
            Self { record_data }
        }
    }

    impl AbecsTypedCommand for TableLoadRecord {
        type Response = EmptyResponse;

        fn command_id(&self) -> &str {
            "TLR"
        }

        fn serialize_params(&self) -> Vec<Vec<u8>> {
            vec![self.record_data.clone()]
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // TableLoadFinish - Finalizar Carga de Tabela (TLF)
    // ═══════════════════════════════════════════════════════════════════════

    #[derive(Debug, Clone)]
    pub struct TableLoadFinish;

    impl TableLoadFinish {
        pub fn new() -> Self {
            Self
        }
    }

    impl AbecsTypedCommand for TableLoadFinish {
        type Response = EmptyResponse;

        fn command_id(&self) -> &str {
            "TLF"
        }

        fn serialize_params(&self) -> Vec<Vec<u8>> {
            Vec::new()
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // GetKey - Obter Chave (GKY)
    // ═══════════════════════════════════════════════════════════════════════

    #[derive(Debug, Clone)]
    pub struct GetKey {
        pub key_index: u8,
    }

    impl GetKey {
        pub fn new(key_index: u8) -> Self {
            Self { key_index }
        }
    }

    impl AbecsTypedCommand for GetKey {
        type Response = GetKeyResponse;

        fn command_id(&self) -> &str {
            "GKY"
        }

        fn serialize_params(&self) -> Vec<Vec<u8>> {
            vec![format!("{:02}", self.key_index).serialize_abecs()]
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Implementações de Desserialização
// ═══════════════════════════════════════════════════════════════════════════

impl AbecsDeserialize for EmptyResponse {
    fn deserialize_abecs(_response: &AbecsResponse) -> Result<Self, String> {
        Ok(EmptyResponse)
    }
}

impl AbecsDeserialize for GetInfoResponse {
    fn deserialize_abecs(response: &AbecsResponse) -> Result<Self, String> {
        let info = response.get_string(0).unwrap_or_default();
        Ok(GetInfoResponse { info })
    }
}

impl AbecsDeserialize for GetPinResponse {
    fn deserialize_abecs(response: &AbecsResponse) -> Result<Self, String> {
        let pin_block = response
            .get_block(0)
            .ok_or("PIN block não encontrado")?
            .to_vec();

        Ok(GetPinResponse { pin_block })
    }
}

impl AbecsDeserialize for GetDataResponse {
    fn deserialize_abecs(response: &AbecsResponse) -> Result<Self, String> {
        let data = response.get_string(0).unwrap_or_default();
        Ok(GetDataResponse { data })
    }
}

impl AbecsDeserialize for MenuResponse {
    fn deserialize_abecs(response: &AbecsResponse) -> Result<Self, String> {
        let index_str = response.get_string(0).ok_or("Índice não encontrado")?;

        let selected_index = index_str
            .trim()
            .parse::<u8>()
            .map_err(|e| format!("Erro ao parsear índice: {}", e))?;

        Ok(MenuResponse { selected_index })
    }
}

impl AbecsDeserialize for GetKeyResponse {
    fn deserialize_abecs(response: &AbecsResponse) -> Result<Self, String> {
        let key_check_value = response
            .get_block(0)
            .ok_or("Key check value não encontrado")?
            .to_vec();

        Ok(GetKeyResponse { key_check_value })
    }
}
