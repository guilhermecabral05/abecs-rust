/// Exemplo 05: Captura de PIN
///
/// Este exemplo demonstra como capturar o PIN (senha) do cartão de forma segura.
/// O PIN é criptografado pelo Pinpad e retornado como um bloco criptografado.
///
/// ⚠️  ATENÇÃO: Este é um comando BLOCANTE!
/// O programa aguardará até que o usuário digite o PIN ou o timeout expire.
///
/// ⚠️  IMPORTANTE: Este exemplo é apenas demonstrativo!
/// Em produção, você precisa:
/// - Configurar as chaves criptográficas corretamente
/// - Usar o tipo de criptografia adequado
/// - Processar o PIN block conforme o padrão da adquirente
///
/// Execute com: cargo run --example 05_captura_pin
use pinpad::{AbecsCommand, PinpadConnection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════════════════");
    println!("  Exemplo 05: Captura de PIN");
    println!("═══════════════════════════════════════════════════════\n");

    println!("⚠️  AVISO: Este é apenas um exemplo demonstrativo!");
    println!("   Em produção, configure as chaves criptográficas\n");

    // Conectar ao Pinpad
    let port_name = "/dev/ttyACM1";
    println!("🔌 Conectando em {}...", port_name);
    let mut pinpad = PinpadConnection::open(port_name)?;
    println!("✅ Conectado!\n");

    // Abrir sessão
    let cmd = AbecsCommand::Open::new();
    pinpad.execute_typed(&cmd)?;

    // ═══════════════════════════════════════════════════════════
    // Capturar PIN
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Capturando PIN do cartão...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Dados do cartão (exemplo - PAN)
    let pan = "1234567890123456";

    println!("💡 Configuração:");
    println!("   Mensagem: DIGITE A SENHA");
    println!("   PIN mínimo: 4 dígitos");
    println!("   PIN máximo: 12 dígitos");
    println!("   Timeout: 30 segundos");
    println!("   PAN: {}\n", pan);

    println!("💡 Aguardando digitação da senha no Pinpad...\n");

    let cmd = AbecsCommand::GetPin::new(
        "DIGITE A SENHA", // Mensagem
        4,                // Tamanho mínimo do PIN
        12,               // Tamanho máximo do PIN
        30,               // Timeout em segundos
        "01",             // Tipo de criptografia (01 = DUKPT/3DES)
        pan,              // PAN do cartão (últimos 12 dígitos)
    );

    match pinpad.execute_typed(&cmd) {
        Ok(response) => {
            println!("✅ PIN capturado com sucesso!\n");
            println!("📦 PIN Block (criptografado):");
            println!("   Tamanho: {} bytes", response.pin_block.len());
            print!("   Hex: ");
            for byte in &response.pin_block {
                print!("{:02X}", byte);
            }
            println!("\n");

            println!("💡 Este PIN block deve ser enviado para a adquirente");
            println!("   para validação junto ao banco emissor.\n");
        }
        Err(pinpad::AbecsError::UserCancelled) => {
            println!("❌ Operação cancelada pelo usuário (botão vermelho)\n");
            // Fechar sessão antes de sair
            let cmd = AbecsCommand::Close::new();
            let _ = pinpad.execute_typed(&cmd);
            return Ok(());
        }
        Err(e) => {
            println!("❌ Erro ou timeout: {}", e);
            println!("   Possíveis causas:");
            println!("   - Timeout expirado");
            println!("   - Chaves não configuradas");
            println!("   - Erro de comunicação\n");
        }
    }

    // ═══════════════════════════════════════════════════════════
    // Exibir mensagem de confirmação
    // ═══════════════════════════════════════════════════════════
    let cmd = AbecsCommand::Display::new("    SENHA OK!       PROCESSANDO...  ");
    pinpad.execute_typed(&cmd)?;

    std::thread::sleep(std::time::Duration::from_secs(2));

    // Fechar sessão
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let cmd = AbecsCommand::Close::new();
    pinpad.execute_typed(&cmd)?;

    println!("\n═══════════════════════════════════════════════════════");
    println!("  ✅ Exemplo concluído com sucesso!");
    println!("═══════════════════════════════════════════════════════");

    println!("\n📚 Próximos passos:");
    println!("   1. Configure as chaves criptográficas no Pinpad");
    println!("   2. Use o tipo de criptografia correto da adquirente");
    println!("   3. Envie o PIN block para validação");
    println!("   4. Implemente tratamento de erros robusto\n");

    Ok(())
}
