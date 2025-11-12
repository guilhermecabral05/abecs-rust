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
    let port_name = "/dev/ttyACM1";
    println!("🔌 Conectando em {}...", port_name);
    let mut pinpad = PinpadConnection::open(port_name)?;
    println!("✅ Conectado!\n");

    // Abrir sessão
    let cmd = AbecsCommand::Open::new();
    pinpad.execute_typed(&cmd)?;

    // ═══════════════════════════════════════════════════════════
    // Tipos de informação disponíveis (código do bloco 1)
    // ═══════════════════════════════════════════════════════════
    let info_types = vec![
        ("01", "Versão do Protocolo ABECS"),
        ("02", "Nome do Fabricante"),
        ("03", "Modelo do Equipamento"),
        ("04", "Número de Série"),
        ("05", "Versão do Software"),
        ("06", "Capacidades"),
    ];

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Obtendo informações do Pinpad...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    for (code, description) in info_types {
        print!("📊 {}: ", description);

        let cmd = AbecsCommand::GetInfo::new(code);
        match pinpad.execute_typed(&cmd) {
            Ok(response) => {
                println!("{}", response.info.trim());
            }
            Err(e) => {
                println!("❌ Erro: {}", e);
            }
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
