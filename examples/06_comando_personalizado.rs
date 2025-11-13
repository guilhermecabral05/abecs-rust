/// Exemplo 06: Comando Personalizado
///
/// Este exemplo demonstra como criar seus próprios comandos ABECS personalizados.
/// Útil quando você precisa:
/// - Implementar comandos novos da especificação ABECS
/// - Criar comandos específicos do seu Pinpad
/// - Testar comandos experimentais
///
/// Execute com: cargo run --example 06_comando_personalizado
use pinpad::{
    AbecsDeserialize, AbecsResponse, AbecsSerialize, AbecsTypedCommand, PinpadConnection,
};

// ═══════════════════════════════════════════════════════════════════════════
// 1. Definir o comando personalizado
// ═══════════════════════════════════════════════════════════════════════════

/// Comando personalizado: Obter Status do Pinpad (exemplo fictício)
#[derive(Debug, Clone)]
pub struct GetStatusCommand {
    pub status_type: u8,
}

/// Resposta do comando GetStatus
#[derive(Debug, Clone)]
pub struct GetStatusResponse {
    pub status_code: String,
    pub status_message: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// 2. Implementar o construtor
// ═══════════════════════════════════════════════════════════════════════════

impl GetStatusCommand {
    pub fn new(status_type: u8) -> Self {
        Self { status_type }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 3. Implementar o trait AbecsTypedCommand
// ═══════════════════════════════════════════════════════════════════════════

impl AbecsTypedCommand for GetStatusCommand {
    type Response = GetStatusResponse;

    /// ID do comando (3 caracteres ASCII)
    fn command_id(&self) -> &str {
        "GST" // Get STatus (exemplo)
    }

    /// Serializar os parâmetros do comando
    fn serialize_params(&self) -> Vec<Vec<u8>> {
        vec![
            // Parâmetro 1: tipo de status como string de 2 dígitos
            format!("{:02}", self.status_type).serialize_abecs(),
        ]
    }

    /// Indica se o comando é blocante (aguarda interação do usuário)
    fn is_blocking(&self) -> bool {
        false // Este comando não aguarda entrada do usuário
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 4. Implementar desserialização da resposta
// ═══════════════════════════════════════════════════════════════════════════

impl AbecsDeserialize for GetStatusResponse {
    fn deserialize_abecs(response: &AbecsResponse) -> Result<Self, String> {
        // Bloco 0: código do status
        let status_code = response.get_string(0).unwrap_or_default();

        // Bloco 1: mensagem (opcional)
        let status_message = response.get_string(1).unwrap_or_default();

        Ok(GetStatusResponse {
            status_code,
            status_message,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Outro exemplo: Comando de Configuração
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct SetConfigCommand {
    pub parameter_id: String,
    pub parameter_value: String,
}

#[derive(Debug, Clone)]
pub struct SetConfigResponse;

impl SetConfigCommand {
    pub fn new(parameter_id: impl Into<String>, parameter_value: impl Into<String>) -> Self {
        Self {
            parameter_id: parameter_id.into(),
            parameter_value: parameter_value.into(),
        }
    }
}

impl AbecsTypedCommand for SetConfigCommand {
    type Response = SetConfigResponse;

    fn command_id(&self) -> &str {
        "CFG" // ConFiGuration
    }

    fn serialize_params(&self) -> Vec<Vec<u8>> {
        vec![
            self.parameter_id.serialize_abecs(),
            self.parameter_value.serialize_abecs(),
        ]
    }
}

impl AbecsDeserialize for SetConfigResponse {
    fn deserialize_abecs(_response: &AbecsResponse) -> Result<Self, String> {
        Ok(SetConfigResponse)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Main - Demonstração de uso
// ═══════════════════════════════════════════════════════════════════════════

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════════════════");
    println!("  Exemplo 06: Comandos Personalizados");
    println!("═══════════════════════════════════════════════════════\n");

    println!("📝 Este exemplo demonstra como criar comandos ABECS");
    println!("   personalizados para seu Pinpad.\n");

    // Conectar ao Pinpad
    let port_name = "/dev/ttyACM0";
    println!("🔌 Conectando em {}...", port_name);
    let mut pinpad = PinpadConnection::open(port_name)?;
    println!("✅ Conectado!\n");

    // Abrir sessão
    let cmd = pinpad::AbecsCommand::Open::new();
    pinpad.execute_typed(&cmd)?;

    // ═══════════════════════════════════════════════════════════
    // Exemplo 1: Usar comando personalizado GetStatus
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Testando comando personalizado: GetStatus");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cmd = GetStatusCommand::new(1);

    println!("📤 Enviando comando GST com parâmetro: {}", cmd.status_type);
    println!("⚠️  Nota: Este comando provavelmente retornará erro");
    println!("   pois é fictício e não existe no Pinpad!\n");

    match pinpad.execute_typed(&cmd) {
        Ok(response) => {
            println!("✅ Resposta recebida:");
            println!("   Status Code: {}", response.status_code);
            println!("   Mensagem: {}\n", response.status_message);
        }
        Err(e) => {
            println!("❌ Erro (esperado): {}\n", e);
            println!("💡 Isso é normal pois o comando não existe no Pinpad!");
        }
    }

    // ═══════════════════════════════════════════════════════════
    // Exemplo 2: Outro comando personalizado
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Testando comando personalizado: SetConfig");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cmd = SetConfigCommand::new("TIMEOUT", "30");

    println!("📤 Enviando comando CFG");
    println!("   Parâmetro: {}", cmd.parameter_id);
    println!("   Valor: {}\n", cmd.parameter_value);

    match pinpad.execute_typed(&cmd) {
        Ok(_) => {
            println!("✅ Configuração alterada!\n");
        }
        Err(e) => {
            println!("❌ Erro (esperado): {}\n", e);
        }
    }

    // ═══════════════════════════════════════════════════════════
    // Demonstração: Estrutura do comando
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Como funcionam os comandos personalizados");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("📚 Passos para criar um comando personalizado:\n");

    println!("1️⃣  Definir a struct do comando");
    println!("   #[derive(Debug, Clone)]");
    println!("   pub struct MeuComando {{ ... }}\n");

    println!("2️⃣  Definir a struct da resposta");
    println!("   #[derive(Debug, Clone)]");
    println!("   pub struct MinhaResposta {{ ... }}\n");

    println!("3️⃣  Implementar AbecsTypedCommand");
    println!("   - command_id(): ID do comando (3 chars)");
    println!("   - serialize_params(): Parâmetros do comando");
    println!("   - is_blocking(): Se aguarda usuário\n");

    println!("4️⃣  Implementar AbecsDeserialize");
    println!("   - deserialize_abecs(): Parsear resposta\n");

    println!("5️⃣  Usar com execute_typed()!");
    println!("   let response = pinpad.execute_typed(&cmd)?;\n");

    // Fechar sessão
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let cmd = pinpad::AbecsCommand::Close::new();
    pinpad.execute_typed(&cmd)?;

    println!("\n═══════════════════════════════════════════════════════");
    println!("  ✅ Exemplo concluído com sucesso!");
    println!("═══════════════════════════════════════════════════════");

    println!("\n💡 Dicas:");
    println!("   • Consulte a especificação ABECS do seu Pinpad");
    println!("   • Teste os comandos em ambiente de desenvolvimento");
    println!("   • Documente bem os comandos personalizados");
    println!("   • Implemente tratamento de erros robusto\n");

    Ok(())
}
