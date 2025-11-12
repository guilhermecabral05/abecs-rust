/// Comandos ABECS pré-definidos com tipos seguros
use crate::response::AbecsResponse;
use crate::serialize::{AbecsDeserialize, AbecsSerialize, AbecsTypedCommand};

// ═══════════════════════════════════════════════════════════════════════════
// Comandos Básicos
// ═══════════════════════════════════════════════════════════════════════════

/// OPN - Abertura de Sessão
#[derive(Debug, Clone)]
pub struct OpenCommand;

#[derive(Debug, Clone)]
pub struct OpenResponse;

impl AbecsTypedCommand for OpenCommand {
    type Response = OpenResponse;

    fn command_id(&self) -> &str {
        "OPN"
    }

    fn serialize_params(&self) -> Vec<Vec<u8>> {
        Vec::new()
    }
}

impl AbecsDeserialize for OpenResponse {
    fn deserialize_abecs(_response: &AbecsResponse) -> Result<Self, String> {
        Ok(OpenResponse)
    }
}

/// CLO - Fechamento de Sessão
#[derive(Debug, Clone)]
pub struct CloseCommand;

#[derive(Debug, Clone)]
pub struct CloseResponse;

impl AbecsTypedCommand for CloseCommand {
    type Response = CloseResponse;

    fn command_id(&self) -> &str {
        "CLO"
    }

    fn serialize_params(&self) -> Vec<Vec<u8>> {
        Vec::new()
    }
}

impl AbecsDeserialize for CloseResponse {
    fn deserialize_abecs(_response: &AbecsResponse) -> Result<Self, String> {
        Ok(CloseResponse)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Comandos de Display
// ═══════════════════════════════════════════════════════════════════════════

/// DSP - Mostrar Mensagem no Display
#[derive(Debug, Clone)]
pub struct DisplayCommand {
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct DisplayResponse;

impl DisplayCommand {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl AbecsTypedCommand for DisplayCommand {
    type Response = DisplayResponse;

    fn command_id(&self) -> &str {
        "DSP"
    }

    fn serialize_params(&self) -> Vec<Vec<u8>> {
        vec![self.message.serialize_abecs()]
    }
}

impl AbecsDeserialize for DisplayResponse {
    fn deserialize_abecs(_response: &AbecsResponse) -> Result<Self, String> {
        Ok(DisplayResponse)
    }
}

/// CLX - Limpar Display
#[derive(Debug, Clone)]
pub struct ClearDisplayCommand;

#[derive(Debug, Clone)]
pub struct ClearDisplayResponse;

impl AbecsTypedCommand for ClearDisplayCommand {
    type Response = ClearDisplayResponse;

    fn command_id(&self) -> &str {
        "CLX"
    }

    fn serialize_params(&self) -> Vec<Vec<u8>> {
        Vec::new()
    }
}

impl AbecsDeserialize for ClearDisplayResponse {
    fn deserialize_abecs(_response: &AbecsResponse) -> Result<Self, String> {
        Ok(ClearDisplayResponse)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Comandos de Informação
// ═══════════════════════════════════════════════════════════════════════════

/// GIN - Obter Informações
#[derive(Debug, Clone)]
pub struct GetInfoCommand {
    pub info_type: String,
}

#[derive(Debug, Clone)]
pub struct GetInfoResponse {
    pub info: String,
}

impl GetInfoCommand {
    pub fn new(info_type: impl Into<String>) -> Self {
        Self {
            info_type: info_type.into(),
        }
    }

    /// Informações gerais do Pinpad
    pub fn general() -> Self {
        Self::new("01")
    }

    /// Versão do aplicativo
    pub fn app_version() -> Self {
        Self::new("02")
    }
}

impl AbecsTypedCommand for GetInfoCommand {
    type Response = GetInfoResponse;

    fn command_id(&self) -> &str {
        "GIN"
    }

    fn serialize_params(&self) -> Vec<Vec<u8>> {
        vec![self.info_type.serialize_abecs()]
    }
}

impl AbecsDeserialize for GetInfoResponse {
    fn deserialize_abecs(response: &AbecsResponse) -> Result<Self, String> {
        // O primeiro bloco contém a informação
        let info = response.get_string(0).unwrap_or_default();

        Ok(GetInfoResponse { info })
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Comandos de Entrada de Dados
// ═══════════════════════════════════════════════════════════════════════════

/// GPN - Obter PIN (Generic PIN)
#[derive(Debug, Clone)]
pub struct GetPinCommand {
    pub message: String,
    pub min_length: u8,
    pub max_length: u8,
    pub timeout: u16,
}

#[derive(Debug, Clone)]
pub struct GetPinResponse {
    pub pin_block: Vec<u8>,
}

impl GetPinCommand {
    pub fn new(message: impl Into<String>, min: u8, max: u8, timeout: u16) -> Self {
        Self {
            message: message.into(),
            min_length: min,
            max_length: max,
            timeout,
        }
    }
}

impl AbecsTypedCommand for GetPinCommand {
    type Response = GetPinResponse;

    fn command_id(&self) -> &str {
        "GPN"
    }

    fn serialize_params(&self) -> Vec<Vec<u8>> {
        vec![
            self.message.serialize_abecs(),
            format!("{:02}", self.min_length).serialize_abecs(),
            format!("{:02}", self.max_length).serialize_abecs(),
            format!("{:03}", self.timeout).serialize_abecs(),
        ]
    }

    fn is_blocking(&self) -> bool {
        true
    }
}

impl AbecsDeserialize for GetPinResponse {
    fn deserialize_abecs(response: &AbecsResponse) -> Result<Self, String> {
        // O primeiro bloco contém o PIN block
        let pin_block = response
            .get_block(0)
            .ok_or("PIN block não encontrado")?
            .to_vec();

        Ok(GetPinResponse { pin_block })
    }
}

/// GDU - Obter Dados (Generic Data Input)
#[derive(Debug, Clone)]
pub struct GetDataCommand {
    pub message: String,
    pub min_length: u8,
    pub max_length: u8,
    pub timeout: u16,
}

#[derive(Debug, Clone)]
pub struct GetDataResponse {
    pub data: String,
}

impl GetDataCommand {
    pub fn new(message: impl Into<String>, min: u8, max: u8, timeout: u16) -> Self {
        Self {
            message: message.into(),
            min_length: min,
            max_length: max,
            timeout,
        }
    }
}

impl AbecsTypedCommand for GetDataCommand {
    type Response = GetDataResponse;

    fn command_id(&self) -> &str {
        "GDU"
    }

    fn serialize_params(&self) -> Vec<Vec<u8>> {
        vec![
            self.message.serialize_abecs(),
            format!("{:02}", self.min_length).serialize_abecs(),
            format!("{:02}", self.max_length).serialize_abecs(),
            format!("{:03}", self.timeout).serialize_abecs(),
        ]
    }

    fn is_blocking(&self) -> bool {
        true
    }
}

impl AbecsDeserialize for GetDataResponse {
    fn deserialize_abecs(response: &AbecsResponse) -> Result<Self, String> {
        // O primeiro bloco contém os dados digitados
        let data = response.get_string(0).unwrap_or_default();

        Ok(GetDataResponse { data })
    }
}

/// MNU - Menu de Seleção
#[derive(Debug, Clone)]
pub struct MenuCommand {
    pub title: String,
    pub options: Vec<String>,
    pub timeout: u16,
}

#[derive(Debug, Clone)]
pub struct MenuResponse {
    pub selected_index: u8,
}

impl MenuCommand {
    pub fn new(title: impl Into<String>, options: Vec<String>, timeout: u16) -> Self {
        Self {
            title: title.into(),
            options,
            timeout,
        }
    }
}

impl AbecsTypedCommand for MenuCommand {
    type Response = MenuResponse;

    fn command_id(&self) -> &str {
        "MNU"
    }

    fn serialize_params(&self) -> Vec<Vec<u8>> {
        let mut params = vec![self.title.serialize_abecs()];
        for option in &self.options {
            params.push(option.serialize_abecs());
        }
        params.push(format!("{:03}", self.timeout).serialize_abecs());
        params
    }

    fn is_blocking(&self) -> bool {
        true
    }
}

impl AbecsDeserialize for MenuResponse {
    fn deserialize_abecs(response: &AbecsResponse) -> Result<Self, String> {
        // O primeiro bloco contém o índice selecionado
        let index_str = response.get_string(0).ok_or("Índice não encontrado")?;

        let selected_index = index_str
            .trim()
            .parse::<u8>()
            .map_err(|e| format!("Erro ao parsear índice: {}", e))?;

        Ok(MenuResponse { selected_index })
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Comandos de Tabelas
// ═══════════════════════════════════════════════════════════════════════════

/// TLI - Table Load Initialize
#[derive(Debug, Clone)]
pub struct TableLoadInitCommand {
    pub table_id: String,
}

#[derive(Debug, Clone)]
pub struct TableLoadInitResponse;

impl TableLoadInitCommand {
    pub fn new(table_id: impl Into<String>) -> Self {
        Self {
            table_id: table_id.into(),
        }
    }
}

impl AbecsTypedCommand for TableLoadInitCommand {
    type Response = TableLoadInitResponse;

    fn command_id(&self) -> &str {
        "TLI"
    }

    fn serialize_params(&self) -> Vec<Vec<u8>> {
        vec![self.table_id.serialize_abecs()]
    }
}

impl AbecsDeserialize for TableLoadInitResponse {
    fn deserialize_abecs(_response: &AbecsResponse) -> Result<Self, String> {
        Ok(TableLoadInitResponse)
    }
}

/// TLR - Table Load Record
#[derive(Debug, Clone)]
pub struct TableLoadRecordCommand {
    pub record_data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct TableLoadRecordResponse;

impl TableLoadRecordCommand {
    pub fn new(record_data: Vec<u8>) -> Self {
        Self { record_data }
    }
}

impl AbecsTypedCommand for TableLoadRecordCommand {
    type Response = TableLoadRecordResponse;

    fn command_id(&self) -> &str {
        "TLR"
    }

    fn serialize_params(&self) -> Vec<Vec<u8>> {
        vec![self.record_data.clone()]
    }
}

impl AbecsDeserialize for TableLoadRecordResponse {
    fn deserialize_abecs(_response: &AbecsResponse) -> Result<Self, String> {
        Ok(TableLoadRecordResponse)
    }
}

/// TLF - Table Load Finish
#[derive(Debug, Clone)]
pub struct TableLoadFinishCommand;

#[derive(Debug, Clone)]
pub struct TableLoadFinishResponse;

impl AbecsTypedCommand for TableLoadFinishCommand {
    type Response = TableLoadFinishResponse;

    fn command_id(&self) -> &str {
        "TLF"
    }

    fn serialize_params(&self) -> Vec<Vec<u8>> {
        Vec::new()
    }
}

impl AbecsDeserialize for TableLoadFinishResponse {
    fn deserialize_abecs(_response: &AbecsResponse) -> Result<Self, String> {
        Ok(TableLoadFinishResponse)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Comandos de Criptografia
// ═══════════════════════════════════════════════════════════════════════════

/// GKY - Get Key (Obter Chave Criptográfica)
#[derive(Debug, Clone)]
pub struct GetKeyCommand {
    pub key_index: u8,
}

#[derive(Debug, Clone)]
pub struct GetKeyResponse {
    pub key_check_value: Vec<u8>,
}

impl GetKeyCommand {
    pub fn new(key_index: u8) -> Self {
        Self { key_index }
    }
}

impl AbecsTypedCommand for GetKeyCommand {
    type Response = GetKeyResponse;

    fn command_id(&self) -> &str {
        "GKY"
    }

    fn serialize_params(&self) -> Vec<Vec<u8>> {
        vec![format!("{:02}", self.key_index).serialize_abecs()]
    }
}

impl AbecsDeserialize for GetKeyResponse {
    fn deserialize_abecs(response: &AbecsResponse) -> Result<Self, String> {
        // O primeiro bloco contém o key check value
        let key_check_value = response
            .get_block(0)
            .ok_or("Key check value não encontrado")?
            .to_vec();

        Ok(GetKeyResponse { key_check_value })
    }
}
