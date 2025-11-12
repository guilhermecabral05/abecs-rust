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
    /// Índice da opção selecionada (baseado em 0)
    ///
    /// Exemplo: Se o usuário seleciona a primeira opção, selected_index = 0
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
    // ═══════════════════════════════════════════════════════════════════════
    // GetData - Obter Dados (GCD - Get Collected Data)
    // ═══════════════════════════════════════════════════════════════════════

    #[derive(Debug, Clone)]
    pub struct GetData {
        pub message_index: u16, // Índice da mensagem pré-definida (ver protocolo)
        pub min_length: u8,
        pub max_length: u8,
        pub timeout: u16,
    }

    impl GetData {
        pub fn new(message_index: u16, min_length: u8, max_length: u8, timeout: u16) -> Self {
            Self {
                message_index,
                min_length,
                max_length,
                timeout,
            }
        }
    }

    impl AbecsTypedCommand for GetData {
        type Response = GetDataResponse;

        fn command_id(&self) -> &str {
            "GCD"
        }

        fn serialize_params(&self) -> Vec<Vec<u8>> {
            use crate::serialize::abecs_param;

            let mut all_params = Vec::new();

            // SPE_MSGIDX (0x000B) - 2 bytes
            let idx_bytes = [
                (self.message_index >> 8) as u8,
                (self.message_index & 0xFF) as u8,
            ];
            all_params.extend_from_slice(&abecs_param(0x000B, &idx_bytes));

            // SPE_MINDIG (0x000D) - 1 byte (opcional)
            if self.min_length > 0 {
                all_params.extend_from_slice(&abecs_param(0x000D, &[self.min_length]));
            }

            // SPE_MAXDIG (0x000E) - 1 byte (opcional)
            if self.max_length > 0 && self.max_length != 32 {
                all_params.extend_from_slice(&abecs_param(0x000E, &[self.max_length]));
            }

            // SPE_TIMEOUT (0x000C) - 1 byte (opcional)
            if self.timeout > 0 {
                all_params.extend_from_slice(&abecs_param(0x000C, &[self.timeout as u8]));
            }

            // Retorna como um único bloco
            vec![all_params]
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
            use crate::serialize::abecs_param;

            let mut all_params = Vec::new();

            // SPE_TIMEOUT (0x000C) - 1 byte
            all_params.extend_from_slice(&abecs_param(0x000C, &[self.timeout as u8]));

            // SPE_MNUOPT (0x0020) - cada opção
            for option in &self.options {
                all_params.extend_from_slice(&abecs_param(0x0020, option.as_bytes()));
            }

            // SPE_DSPMSG (0x001B) - título do menu
            if !self.title.is_empty() {
                all_params.extend_from_slice(&abecs_param(0x001B, self.title.as_bytes()));
            }

            // Retorna como um único bloco
            vec![all_params]
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
        // A resposta de GCD vem no formato TLV: PP_VALUE (0x804D)
        let block = response.get_block(0).ok_or("Bloco não encontrado")?;

        // Parse TLV: ID(2) + Len(2) + Value
        if block.len() < 4 {
            return Err("Resposta muito curta".to_string());
        }

        let param_id = ((block[0] as u16) << 8) | (block[1] as u16);
        let param_len = ((block[2] as u16) << 8) | (block[3] as u16);

        if param_id != 0x804D {
            return Err(format!("ID de parâmetro inesperado: 0x{:04X}", param_id));
        }

        if block.len() < 4 + param_len as usize {
            return Err("Dados incompletos".to_string());
        }

        let data = String::from_utf8_lossy(&block[4..4 + param_len as usize]).to_string();
        Ok(GetDataResponse { data })
    }
}

impl AbecsDeserialize for MenuResponse {
    fn deserialize_abecs(response: &AbecsResponse) -> Result<Self, String> {
        // A resposta de MNU vem no formato TLV: PP_VALUE (0x804D)
        let block = response.get_block(0).ok_or("Bloco não encontrado")?;

        // Parse TLV: ID(2) + Len(2) + Value
        if block.len() < 4 {
            return Err("Resposta muito curta".to_string());
        }

        let param_id = ((block[0] as u16) << 8) | (block[1] as u16);
        let param_len = ((block[2] as u16) << 8) | (block[3] as u16);

        if param_id != 0x804D {
            return Err(format!("ID de parâmetro inesperado: 0x{:04X}", param_id));
        }

        if block.len() < 4 + param_len as usize {
            return Err("Dados incompletos".to_string());
        }

        let index_str = String::from_utf8_lossy(&block[4..4 + param_len as usize]);
        let option_number = index_str
            .trim()
            .parse::<u8>()
            .map_err(|e| format!("Erro ao parsear índice: {}", e))?;

        // O Pinpad retorna o número da opção (1, 2, 3...), mas precisamos do índice do array (0, 1, 2...)
        let selected_index = if option_number > 0 {
            option_number - 1
        } else {
            return Err("Número de opção inválido (deve ser >= 1)".to_string());
        };

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
