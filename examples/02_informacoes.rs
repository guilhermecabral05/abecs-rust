/// Exemplo 02: Obter Informações do Pinpad
///
/// Este exemplo demonstra como obter informações do Pinpad:
/// - Versão do protocolo ABECS
/// - Fabricante
/// - Modelo
/// - Número de série
///
/// Execute com: cargo run --example 02_informacoes
use pinpad::{AbecsCommand, PinpadConnection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════════════════");
    println!("  Exemplo 02: Informações do Pinpad");
    println!("═══════════════════════════════════════════════════════\n");

    // Conectar ao Pinpad
    let port_name = "/dev/ttyACM0";
    println!("🔌 Conectando em {}...", port_name);
    let mut pinpad = PinpadConnection::open(port_name)?;
    // pinpad.set_verbose(true); // Descomente para debug
    println!("✅ Conectado!\n");

    // Abrir sessão
    let cmd = AbecsCommand::Open::new();
    pinpad.execute_typed(&cmd)?;

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Obtendo informações gerais do Pinpad (índice 00)...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Informações gerais do pinpad (GIN_ACQIDX = "00")
    let cmd = AbecsCommand::GetInfo::new("00");
    match pinpad.execute_typed(&cmd) {
        Ok(response) => {
            println!("📋 Informações Gerais:\n{}\n", response.info);
        }
        Err(e) => {
            println!("❌ Erro: {}\n", e);
        }
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Obtendo informações do Kernel Abecs ICC (índice 02)...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cmd = AbecsCommand::GetInfo::new("02");
    match pinpad.execute_typed(&cmd) {
        Ok(response) => {
            println!("📋 Kernel Abecs (ICC):\n{}\n", response.info);
        }
        Err(e) => {
            println!("❌ Erro: {}\n", e);
        }
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Obtendo informações do Kernel Abecs CTLS (índice 03)...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cmd = AbecsCommand::GetInfo::new("03");
    match pinpad.execute_typed(&cmd) {
        Ok(response) => {
            println!("📋 Kernel Abecs (CTLS):\n{}\n", response.info);
        }
        Err(e) => {
            println!("❌ Erro: {}\n", e);
        }
    }

    // Fechar sessão
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let cmd = AbecsCommand::Close::new();
    pinpad.execute_typed(&cmd)?;

    println!("\n═══════════════════════════════════════════════════════");
    println!("  ✅ Exemplo concluído com sucesso!");
    println!("═══════════════════════════════════════════════════════");

    Ok(())
}
