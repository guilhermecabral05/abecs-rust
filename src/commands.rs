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

/// Resposta do comando GetCard (GCX)
#[derive(Debug, Clone)]
pub struct GetCardResponse {
    pub card_type: String, // "00"=Magnético, "03"=ICC EMV, "05"=CTLS tarja, "06"=CTLS EMV
    pub pan: Option<String>, // PAN do cartão (se disponível)
    pub track1: Option<String>, // Trilha 1 (incompleta)
    pub track2: Option<String>, // Trilha 2 (incompleta)
    pub track3: Option<String>, // Trilha 3 (incompleta)
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
        pub method: String, // "0"=MK/WK:DES, "1"=MK/WK:TDES, "2"=DUKPT:DES, "3"=DUKPT:TDES
        pub key_index: String, // Índice da MK ou DUKPT (2 dígitos)
        pub working_key: String, // Working Key criptografada (32 hex chars, ignorado se DUKPT)
        pub pan: String,    // PAN do cartão
    }

    impl GetPin {
        pub fn new(
            message: impl Into<String>,
            min_length: u8,
            max_length: u8,
            method: impl Into<String>,
            key_index: impl Into<String>,
            working_key: impl Into<String>,
            pan: impl Into<String>,
        ) -> Self {
            Self {
                message: message.into(),
                min_length,
                max_length,
                method: method.into(),
                key_index: key_index.into(),
                working_key: working_key.into(),
                pan: pan.into(),
            }
        }
    }

    impl AbecsTypedCommand for GetPin {
        type Response = GetPinResponse;

        fn command_id(&self) -> &str {
            "GPN"
        }

        fn serialize_params(&self) -> Vec<Vec<u8>> {
            // Formato do GPN segundo protocolo ABECS 2.12
            let mut params = Vec::new();

            // GPN_METHOD (1 byte)
            params.push(self.method.as_bytes().to_vec());

            // GPN_KEYIDX (2 bytes)
            params.push(format!("{:02}", self.key_index).as_bytes().to_vec());

            // GPN_WKENC (32 bytes hex) - preenchido com zeros se DUKPT
            let wk = if self.working_key.is_empty() {
                "00000000000000000000000000000000".to_string()
            } else {
                format!("{:0<32}", self.working_key)
            };
            params.push(wk.as_bytes().to_vec());

            // GPN_PANLEN (2 bytes)
            let pan_len = self.pan.len().min(19);
            params.push(format!("{:02}", pan_len).as_bytes().to_vec());

            // GPN_PAN (19 bytes, alinhado à esquerda com espaços)
            params.push(format!("{:<19}", self.pan).as_bytes().to_vec());

            // GPN_ENTRIES (1 byte) - fixo "1"
            params.push(b"1".to_vec());

            // GPN_MIN1 (2 bytes)
            params.push(format!("{:02}", self.min_length).as_bytes().to_vec());

            // GPN_MAX1 (2 bytes)
            params.push(format!("{:02}", self.max_length).as_bytes().to_vec());

            // GPN_MSG1 (32 bytes)
            params.push(format!("{:<32}", self.message).as_bytes().to_vec());

            params
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
    // GetCard - Obter Cartão (GCX - Get Card Extended)
    // ═══════════════════════════════════════════════════════════════════════

    #[derive(Debug, Clone)]
    pub struct GetCard {
        pub amount: u64,             // Valor em centavos
        pub date: String,            // Data AAMMDD (ano 2 dígitos)
        pub time: String,            // Hora HHMMSS
        pub timeout: u16,            // Timeout em segundos
        pub message: Option<String>, // Mensagem customizada (opcional)
    }

    impl GetCard {
        pub fn new(
            amount: u64,
            date: impl Into<String>,
            time: impl Into<String>,
            timeout: u16,
        ) -> Self {
            Self {
                amount,
                date: date.into(),
                time: time.into(),
                timeout,
                message: None,
            }
        }

        pub fn with_message(mut self, message: impl Into<String>) -> Self {
            self.message = Some(message.into());
            self
        }
    }

    impl AbecsTypedCommand for GetCard {
        type Response = GetCardResponse;

        fn command_id(&self) -> &str {
            "GCX"
        }

        fn serialize_params(&self) -> Vec<Vec<u8>> {
            use crate::serialize::abecs_param;

            let mut all_params = Vec::new();

            // SPE_AMOUNT (0x0013) - valor em centavos, 12 dígitos
            let amount_str = format!("{:012}", self.amount);
            all_params.extend_from_slice(&abecs_param(0x0013, amount_str.as_bytes()));

            // SPE_TRNDATE (0x0015) - data AAMMDD
            all_params.extend_from_slice(&abecs_param(0x0015, self.date.as_bytes()));

            // SPE_TRNTIME (0x0016) - hora HHMMSS
            all_params.extend_from_slice(&abecs_param(0x0016, self.time.as_bytes()));

            // SPE_GCXOPT (0x0017) - opções: "10000" = aceita mag/ICC/CTLS e mostra valor
            all_params.extend_from_slice(&abecs_param(0x0017, b"10000"));

            // SPE_TIMEOUT (0x000C) - timeout em segundos
            all_params.extend_from_slice(&abecs_param(0x000C, &[self.timeout as u8]));

            // SPE_DSPMSG (0x001B) - mensagem customizada (opcional)
            if let Some(ref msg) = self.message {
                all_params.extend_from_slice(&abecs_param(0x001B, msg.as_bytes()));
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

impl AbecsDeserialize for GetCardResponse {
    fn deserialize_abecs(response: &AbecsResponse) -> Result<Self, String> {
        // A resposta de GCX vem em formato TLV com múltiplos parâmetros
        let block = response.get_block(0).ok_or("Bloco não encontrado")?;

        let mut card_type = String::new();
        let mut pan = None;
        let mut track1 = None;
        let mut track2 = None;
        let mut track3 = None;

        // Parser TLV simples
        let mut pos = 0;
        while pos + 4 <= block.len() {
            let param_id = ((block[pos] as u16) << 8) | (block[pos + 1] as u16);
            let param_len = ((block[pos + 2] as u16) << 8) | (block[pos + 3] as u16);
            pos += 4;

            if pos + param_len as usize > block.len() {
                break;
            }

            let value = &block[pos..pos + param_len as usize];

            match param_id {
                0x804F => {
                    // PP_CARDTYPE
                    card_type = String::from_utf8_lossy(value).to_string();
                }
                0x8036 => {
                    // PP_PAN
                    pan = Some(String::from_utf8_lossy(value).to_string());
                }
                0x8037 => {
                    // PP_TRK1INC
                    track1 = Some(String::from_utf8_lossy(value).to_string());
                }
                0x8038 => {
                    // PP_TRK2INC
                    track2 = Some(String::from_utf8_lossy(value).to_string());
                }
                0x8039 => {
                    // PP_TRK3INC
                    track3 = Some(String::from_utf8_lossy(value).to_string());
                }
                _ => {}
            }

            pos += param_len as usize;
        }

        Ok(GetCardResponse {
            card_type,
            pan,
            track1,
            track2,
            track3,
        })
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
