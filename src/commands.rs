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
    pub emv_data: Option<crate::emv::EmvData>, // Dados EMV em formato TLV
    pub icc_status: Option<String>, // Status ICC (código de 2 dígitos)
    pub aid_table_info: Option<Vec<u8>>, // Informações da tabela AID
}

/// Resposta do comando Menu
#[derive(Debug, Clone)]
pub struct MenuResponse {
    /// Índice da opção selecionada (baseado em 0)
    ///
    /// Exemplo: Se o usuário seleciona a primeira opção, selected_index = 0
    pub selected_index: u8,
}

/// Resposta do comando GetTracks (GTK)
#[derive(Debug, Clone)]
pub struct GetTracksResponse {
    pub pan: Option<Vec<u8>>,        // PAN (em claro ou criptografado)
    pub track1: Option<Vec<u8>>,     // Trilha 1 (em claro ou criptografada)
    pub track2: Option<Vec<u8>>,     // Trilha 2 (em claro ou criptografada)
    pub track3: Option<Vec<u8>>,     // Trilha 3 (em claro ou criptografada)
    pub track1_ksn: Option<Vec<u8>>, // KSN da trilha 1 (se DUKPT)
    pub track2_ksn: Option<Vec<u8>>, // KSN da trilha 2 (se DUKPT)
    pub track3_ksn: Option<Vec<u8>>, // KSN da trilha 3 (se DUKPT)
    pub pan_ksn: Option<Vec<u8>>,    // KSN do PAN (se DUKPT)
    pub krand_enc: Option<Vec<u8>>,  // KRAND criptografado (se RSA)
}

/// Resposta do comando GetKey
#[derive(Debug, Clone)]
pub struct GetKeyResponse {
    pub key_check_value: Vec<u8>,
}

/// Resposta do comando GoOnChip (GOX)
#[derive(Debug, Clone)]
pub struct GoOnChipResponse {
    /// Resultado GOX - 6 dígitos indicando aprovação/PIN/status
    /// Formato: XXYYZZ onde:
    /// - XX: Status de processamento (00=OK, outros=erro)
    /// - YY: Indicação de PIN necessário (00=não, 01=sim)
    /// - ZZ: Resultado da transação
    pub gox_result: String,
    /// Dados EMV resultantes do processamento
    pub emv_data: Option<crate::emv::EmvData>,
    /// PIN Block criptografado (se PIN foi capturado)
    pub pin_block: Option<Vec<u8>>,
    /// Resultados de segurança do issuer
    pub issuer_results: Option<Vec<u8>>,
}

/// Resposta do comando FinishChip (FCX)
#[derive(Debug, Clone)]
pub struct FinishChipResponse {
    /// Resultado FCX - 3 dígitos indicando aprovação final
    /// "000" = Aprovado
    /// "001" = Negado
    pub fcx_result: String,
    /// Dados EMV finais após processamento
    pub emv_data: Option<crate::emv::EmvData>,
    /// Resultados finais de segurança do issuer
    pub issuer_results: Option<Vec<u8>>,
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
            // Todos os campos devem ser concatenados em um único bloco
            let mut data = Vec::new();

            // GPN_METHOD (1 byte)
            data.extend_from_slice(self.method.as_bytes());

            // GPN_KEYIDX (2 bytes)
            data.extend_from_slice(self.key_index.as_bytes());

            // GPN_WKENC (32 bytes hex) - preenchido com zeros se DUKPT ou vazio
            let wk = if self.working_key.is_empty() || self.method == "2" || self.method == "3" {
                "00000000000000000000000000000000".to_string()
            } else {
                format!("{:0<32}", self.working_key)
            };
            data.extend_from_slice(wk.as_bytes());

            // GPN_PANLEN (2 bytes)
            let pan_len = self.pan.len().min(19);
            data.extend_from_slice(format!("{:02}", pan_len).as_bytes());

            // GPN_PAN (19 bytes, alinhado à esquerda com espaços)
            data.extend_from_slice(format!("{:<19}", self.pan).as_bytes());

            // GPN_ENTRIES (1 byte) - fixo "1"
            data.push(b'1');

            // GPN_MIN1 (2 bytes)
            data.extend_from_slice(format!("{:02}", self.min_length).as_bytes());

            // GPN_MAX1 (2 bytes)
            data.extend_from_slice(format!("{:02}", self.max_length).as_bytes());

            // GPN_MSG1 (32 bytes)
            data.extend_from_slice(format!("{:<32}", self.message).as_bytes());

            // Retorna como um único bloco
            vec![data]
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
    // GoOnChip - Processar Chip EMV (GOX)
    // ═══════════════════════════════════════════════════════════════════════

    #[derive(Debug, Clone)]
    pub struct GoOnChip {
        pub app_type: String,             // Tipo de aplicação (ex: "04" = débito)
        pub amount: u64,                  // Valor em centavos
        pub date: String,                 // Data AAMMDD
        pub time: String,                 // Hora HHMMSS
        pub gox_options: String,          // Opções GOX (ex: "00000000")
        pub terminal_params: Vec<u8>,     // Parâmetros do terminal
        pub transaction_currency: String, // Código da moeda (ex: "0986" = BRL)
        pub emv_data: Option<crate::emv::EmvData>, // Dados EMV adicionais
    }

    impl GoOnChip {
        pub fn new(
            app_type: impl Into<String>,
            amount: u64,
            date: impl Into<String>,
            time: impl Into<String>,
            terminal_params: Vec<u8>,
        ) -> Self {
            Self {
                app_type: app_type.into(),
                amount,
                date: date.into(),
                time: time.into(),
                gox_options: "00000000".to_string(), // Padrão: sem opções especiais
                terminal_params,
                transaction_currency: "0986".to_string(), // BRL por padrão
                emv_data: None,
            }
        }

        pub fn with_options(mut self, options: impl Into<String>) -> Self {
            self.gox_options = options.into();
            self
        }

        pub fn with_currency(mut self, currency: impl Into<String>) -> Self {
            self.transaction_currency = currency.into();
            self
        }

        pub fn with_emv_data(mut self, emv_data: crate::emv::EmvData) -> Self {
            self.emv_data = Some(emv_data);
            self
        }
    }

    impl AbecsTypedCommand for GoOnChip {
        type Response = GoOnChipResponse;

        fn command_id(&self) -> &str {
            "GOX"
        }

        fn serialize_params(&self) -> Vec<Vec<u8>> {
            use crate::serialize::abecs_param;

            let mut all_params = Vec::new();

            // SPE_APPTYPE (0x0011) - Tipo de aplicação
            all_params.extend_from_slice(&abecs_param(0x0011, self.app_type.as_bytes()));

            // SPE_AMOUNT (0x0013) - Valor em centavos, 12 dígitos
            let amount_str = format!("{:012}", self.amount);
            all_params.extend_from_slice(&abecs_param(0x0013, amount_str.as_bytes()));

            // SPE_TRNDATE (0x0015) - Data AAMMDD
            all_params.extend_from_slice(&abecs_param(0x0015, self.date.as_bytes()));

            // SPE_TRNTIME (0x0016) - Hora HHMMSS
            all_params.extend_from_slice(&abecs_param(0x0016, self.time.as_bytes()));

            // SPE_GOXOPT (0x0019) - Opções GOX
            all_params.extend_from_slice(&abecs_param(0x0019, self.gox_options.as_bytes()));

            // SPE_TRMPAR (0x001A) - Parâmetros do terminal
            all_params.extend_from_slice(&abecs_param(0x001A, &self.terminal_params));

            // SPE_TRNCURR (0x0022) - Código da moeda
            all_params
                .extend_from_slice(&abecs_param(0x0022, self.transaction_currency.as_bytes()));

            // SPE_EMVDATA (0x0023) - Dados EMV (opcional)
            if let Some(ref emv) = self.emv_data {
                let emv_bytes = emv.serialize();
                all_params.extend_from_slice(&abecs_param(0x0023, &emv_bytes));
            }

            vec![all_params]
        }

        fn is_blocking(&self) -> bool {
            true
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // FinishChip - Finalizar Transação Chip (FCX)
    // ═══════════════════════════════════════════════════════════════════════

    #[derive(Debug, Clone)]
    pub struct FinishChip {
        pub fcx_options: String,                   // Opções FCX (ex: "00")
        pub arc: String,                           // Authorization Response Code (2 bytes)
        pub emv_data: Option<crate::emv::EmvData>, // Dados EMV do issuer
    }

    impl FinishChip {
        pub fn new(arc: impl Into<String>) -> Self {
            Self {
                fcx_options: "00".to_string(), // Padrão: sem opções especiais
                arc: arc.into(),
                emv_data: None,
            }
        }

        pub fn with_options(mut self, options: impl Into<String>) -> Self {
            self.fcx_options = options.into();
            self
        }

        pub fn with_emv_data(mut self, emv_data: crate::emv::EmvData) -> Self {
            self.emv_data = Some(emv_data);
            self
        }
    }

    impl AbecsTypedCommand for FinishChip {
        type Response = FinishChipResponse;

        fn command_id(&self) -> &str {
            "FCX"
        }

        fn serialize_params(&self) -> Vec<Vec<u8>> {
            use crate::serialize::abecs_param;

            let mut all_params = Vec::new();

            // SPE_FCXOPT (0x001C) - Opções FCX
            all_params.extend_from_slice(&abecs_param(0x001C, self.fcx_options.as_bytes()));

            // SPE_ARC (0x001D) - Authorization Response Code
            all_params.extend_from_slice(&abecs_param(0x001D, self.arc.as_bytes()));

            // SPE_EMVDATA (0x0023) - Dados EMV do issuer (opcional)
            if let Some(ref emv) = self.emv_data {
                let emv_bytes = emv.serialize();
                all_params.extend_from_slice(&abecs_param(0x0023, &emv_bytes));
            }

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

    // ═══════════════════════════════════════════════════════════════════════
    // GetTracks - Obter Trilhas (GTK)
    // ═══════════════════════════════════════════════════════════════════════

    /// Comando GTK - Get Tracks
    ///
    /// Obtém as trilhas completas do cartão lido através de CEX ou GCX.
    /// As trilhas podem ser retornadas em claro ou criptografadas.
    ///
    /// IMPORTANTE: Este comando só pode ser usado UMA ÚNICA VEZ após CEX/GCX com sucesso.
    ///
    /// # Exemplos
    ///
    /// ```rust,no_run
    /// use pinpad::AbecsCommand::GetTracks;
    ///
    /// // Obter todas as trilhas em claro
    /// let cmd = GetTracks::new_plain();
    ///
    /// // Obter somente trilhas 1 e 2 em claro
    /// let cmd = GetTracks::new_plain()
    ///     .with_tracks(false, true, true, false); // PAN, T1, T2, T3
    ///
    /// // Obter trilhas criptografadas com DUKPT
    /// let cmd = GetTracks::new_dukpt("00", "30"); // keyidx=00, método DUKPT:TDES:DAT#1 ECB
    /// ```
    #[derive(Debug, Clone)]
    pub struct GetTracks {
        /// Quais trilhas retornar: (PAN, T1, T2, T3)
        pub tracks: Option<String>, // "1111" = todas, "0110" = só T1 e T2
        /// Método de criptografia (ex: "30" = DUKPT:TDES:DAT#1 ECB)
        pub crypt_method: Option<String>,
        /// IV para CBC (8 bytes hex)
        pub iv_cbc: Option<String>,
        /// Quantidade de dígitos em claro no início das trilhas
        pub open_digits: Option<String>,
        /// Índice da chave (MK:DAT ou DUKPT:DAT)
        pub key_index: Option<String>,
        /// Working Key criptografada pela MK (para método MK/WK)
        pub wk_enc: Option<Vec<u8>>,
        /// Módulo RSA (para método com chave aleatória)
        pub rsa_modulus: Option<Vec<u8>>,
        /// Expoente RSA (para método com chave aleatória)
        pub rsa_exponent: Option<Vec<u8>>,
    }

    impl GetTracks {
        /// Cria comando GTK para obter trilhas em claro
        pub fn new_plain() -> Self {
            Self {
                tracks: None,       // Retorna todas disponíveis
                crypt_method: None, // Sem criptografia
                iv_cbc: None,
                open_digits: None,
                key_index: None,
                wk_enc: None,
                rsa_modulus: None,
                rsa_exponent: None,
            }
        }

        /// Cria comando GTK com criptografia DUKPT
        pub fn new_dukpt(key_index: &str, method: &str) -> Self {
            Self {
                tracks: None,
                crypt_method: Some(method.to_string()),
                iv_cbc: None,
                open_digits: None,
                key_index: Some(key_index.to_string()),
                wk_enc: None,
                rsa_modulus: None,
                rsa_exponent: None,
            }
        }

        /// Define quais trilhas retornar
        pub fn with_tracks(mut self, pan: bool, t1: bool, t2: bool, t3: bool) -> Self {
            let mut tracks = String::new();
            tracks.push(if pan { '1' } else { '0' });
            tracks.push(if t1 { '1' } else { '0' });
            tracks.push(if t2 { '1' } else { '0' });
            tracks.push(if t3 { '1' } else { '0' });
            self.tracks = Some(tracks);
            self
        }

        /// Define quantidade de dígitos em claro no início das trilhas
        pub fn with_open_digits(mut self, digits: u8) -> Self {
            self.open_digits = Some(format!("{:02}", digits));
            self
        }

        /// Define IV para modo CBC
        pub fn with_iv_cbc(mut self, iv: &str) -> Self {
            self.iv_cbc = Some(iv.to_string());
            self
        }
    }

    impl AbecsTypedCommand for GetTracks {
        type Response = GetTracksResponse;

        fn command_id(&self) -> &str {
            "GTK"
        }

        fn serialize_params(&self) -> Vec<Vec<u8>> {
            let mut blocks = Vec::new();

            // SPE_TRACKS (opcional) - [07]
            if let Some(ref tracks) = self.tracks {
                let mut block = vec![0x00, 0x07]; // Tag
                let data = tracks.as_bytes();
                block.extend_from_slice(&[(data.len() >> 8) as u8, data.len() as u8]);
                block.extend_from_slice(data);
                blocks.push(block);
            }

            // SPE_MTHDDAT (opcional) - [03]
            if let Some(ref method) = self.crypt_method {
                let mut block = vec![0x00, 0x03]; // Tag
                let data = method.as_bytes();
                block.extend_from_slice(&[(data.len() >> 8) as u8, data.len() as u8]);
                block.extend_from_slice(data);
                blocks.push(block);
            }

            // SPE_IVCBC (opcional) - [15]
            if let Some(ref iv) = self.iv_cbc {
                let mut block = vec![0x00, 0x15]; // Tag
                                                  // IV deve estar em hex string, converter para bytes
                let data: Vec<u8> = iv
                    .as_bytes()
                    .chunks(2)
                    .filter_map(|chunk| {
                        if chunk.len() == 2 {
                            let byte_str = std::str::from_utf8(chunk).ok()?;
                            u8::from_str_radix(byte_str, 16).ok()
                        } else {
                            None
                        }
                    })
                    .collect();
                let len = data.len();
                block.extend_from_slice(&[(len >> 8) as u8, len as u8]);
                block.extend_from_slice(&data);
                blocks.push(block);
            }

            // SPE_OPNDIG (opcional) - [14]
            if let Some(ref digits) = self.open_digits {
                let mut block = vec![0x00, 0x14]; // Tag
                let data = digits.as_bytes();
                block.extend_from_slice(&[(data.len() >> 8) as u8, data.len() as u8]);
                block.extend_from_slice(data);
                blocks.push(block);
            }

            // SPE_KEYIDX (mandatório se criptografia) - [09]
            if let Some(ref keyidx) = self.key_index {
                let mut block = vec![0x00, 0x09]; // Tag
                let data = keyidx.as_bytes();
                block.extend_from_slice(&[(data.len() >> 8) as u8, data.len() as u8]);
                block.extend_from_slice(data);
                blocks.push(block);
            }

            // SPE_WKENC (mandatório para MK/WK) - [0A]
            if let Some(ref wk) = self.wk_enc {
                let mut block = vec![0x00, 0x0A]; // Tag
                block.extend_from_slice(&[(wk.len() >> 8) as u8, wk.len() as u8]);
                block.extend_from_slice(wk);
                blocks.push(block);
            }

            // SPE_PBKMOD (mandatório para RSA) - [0D]
            if let Some(ref modulus) = self.rsa_modulus {
                let mut block = vec![0x00, 0x0D]; // Tag
                block.extend_from_slice(&[(modulus.len() >> 8) as u8, modulus.len() as u8]);
                block.extend_from_slice(modulus);
                blocks.push(block);
            }

            // SPE_PBKEXP (mandatório para RSA) - [0E]
            if let Some(ref exponent) = self.rsa_exponent {
                let mut block = vec![0x00, 0x0E]; // Tag
                block.extend_from_slice(&[(exponent.len() >> 8) as u8, exponent.len() as u8]);
                block.extend_from_slice(exponent);
                blocks.push(block);
            }

            blocks
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
        let mut emv_data = None;
        let mut icc_status = None;
        let mut aid_table_info = None;

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
                0x8054 => {
                    // PP_EMVDATA - Dados EMV em formato TLV
                    emv_data = crate::emv::EmvData::parse(value).ok();
                }
                0x8057 => {
                    // PP_ICCSTAT - Status ICC
                    icc_status = Some(String::from_utf8_lossy(value).to_string());
                }
                0x805A => {
                    // PP_AIDTABINFO - Informações da tabela AID
                    aid_table_info = Some(value.to_vec());
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
            emv_data,
            icc_status,
            aid_table_info,
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

impl AbecsDeserialize for GoOnChipResponse {
    fn deserialize_abecs(response: &AbecsResponse) -> Result<Self, String> {
        // A resposta de GOX vem em formato TLV com múltiplos parâmetros
        let block = response.get_block(0).ok_or("Bloco não encontrado")?;

        let mut gox_result = String::new();
        let mut emv_data = None;
        let mut pin_block = None;
        let mut issuer_results = None;

        // Parser TLV
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
                0x8050 => {
                    // PP_GOXRES - Resultado GOX (6 dígitos)
                    gox_result = String::from_utf8_lossy(value).to_string();
                }
                0x8054 => {
                    // PP_EMVDATA - Dados EMV em formato TLV
                    emv_data = crate::emv::EmvData::parse(value).ok();
                }
                0x8055 => {
                    // PP_PINBLK - PIN Block criptografado
                    pin_block = Some(value.to_vec());
                }
                0x8056 => {
                    // PP_ISRESULTS - Resultados do issuer
                    issuer_results = Some(value.to_vec());
                }
                _ => {}
            }

            pos += param_len as usize;
        }

        Ok(GoOnChipResponse {
            gox_result,
            emv_data,
            pin_block,
            issuer_results,
        })
    }
}

impl AbecsDeserialize for FinishChipResponse {
    fn deserialize_abecs(response: &AbecsResponse) -> Result<Self, String> {
        // A resposta de FCX vem em formato TLV com múltiplos parâmetros
        let block = response.get_block(0).ok_or("Bloco não encontrado")?;

        let mut fcx_result = String::new();
        let mut emv_data = None;
        let mut issuer_results = None;

        // Parser TLV
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
                0x8051 => {
                    // PP_FCXRES - Resultado FCX (3 dígitos)
                    fcx_result = String::from_utf8_lossy(value).to_string();
                }
                0x8054 => {
                    // PP_EMVDATA - Dados EMV em formato TLV
                    emv_data = crate::emv::EmvData::parse(value).ok();
                }
                0x8056 => {
                    // PP_ISRESULTS - Resultados do issuer
                    issuer_results = Some(value.to_vec());
                }
                _ => {}
            }

            pos += param_len as usize;
        }

        Ok(FinishChipResponse {
            fcx_result,
            emv_data,
            issuer_results,
        })
    }
}

impl AbecsDeserialize for GetTracksResponse {
    fn deserialize_abecs(response: &AbecsResponse) -> Result<Self, String> {
        // A resposta de GTK vem em formato TLV com múltiplos parâmetros
        let block = response.get_block(0).ok_or("Bloco não encontrado")?;

        let mut pan = None;
        let mut track1 = None;
        let mut track2 = None;
        let mut track3 = None;
        let mut track1_ksn = None;
        let mut track2_ksn = None;
        let mut track3_ksn = None;
        let mut pan_ksn = None;
        let mut krand_enc = None;

        // Parser TLV
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
                0x804A => {
                    // PP_ENCPAN - PAN (em claro ou criptografado)
                    pan = Some(value.to_vec());
                }
                0x8045 => {
                    // PP_TRACK1 - Trilha 1
                    track1 = Some(value.to_vec());
                }
                0x8042 => {
                    // PP_TRACK2 - Trilha 2
                    track2 = Some(value.to_vec());
                }
                0x8043 => {
                    // PP_TRACK3 - Trilha 3
                    track3 = Some(value.to_vec());
                }
                0x804B => {
                    // PP_TRK1KSN - KSN da trilha 1
                    track1_ksn = Some(value.to_vec());
                }
                0x804C => {
                    // PP_TRK2KSN - KSN da trilha 2
                    track2_ksn = Some(value.to_vec());
                }
                0x804D => {
                    // PP_TRK3KSN - KSN da trilha 3
                    track3_ksn = Some(value.to_vec());
                }
                0x8049 => {
                    // PP_ENCPANKSN - KSN do PAN
                    pan_ksn = Some(value.to_vec());
                }
                0x8048 => {
                    // PP_ENCKRAND - KRAND criptografado
                    krand_enc = Some(value.to_vec());
                }
                _ => {}
            }

            pos += param_len as usize;
        }

        Ok(GetTracksResponse {
            pan,
            track1,
            track2,
            track3,
            track1_ksn,
            track2_ksn,
            track3_ksn,
            pan_ksn,
            krand_enc,
        })
    }
}
