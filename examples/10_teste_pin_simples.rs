/// Exemplo 10: Teste Simples - Apenas PIN
///
/// Este exemplo testa APENAS a captura de PIN, sem leitura de cartão
///
/// Execute com: cargo run --example 10_teste_pin_simples
use pinpad::{AbecsCommand, PinpadConnection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════════════════");
    println!("  Teste Simples: Apenas PIN");
    println!("═══════════════════════════════════════════════════════\n");

    let port_name = "/dev/ttyACM0";
    println!("🔌 Conectando em {}...", port_name);
    let mut pinpad = PinpadConnection::open(port_name)?;

    // Modo verbose
    pinpad.set_verbose(true);

    println!("✅ Conectado!\n");

    // Abrir Sessão
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 1: Abrir Sessão");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cmd = AbecsCommand::Open::new();
    pinpad.execute_typed(&cmd)?;
    println!("✅ Sessão aberta\n");

    // Display
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 2: Exibir Mensagem");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cmd = AbecsCommand::Display::new("032  TESTE PIN     ");
    pinpad.execute_typed(&cmd)?;
    println!("✅ Mensagem exibida\n");

    std::thread::sleep(std::time::Duration::from_secs(2));

    // Capturar PIN
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 3: Captura de PIN");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("🔐 Digite a senha no Pinpad...\n");

    let pan = "1234567890123456"; // PAN de teste

    let cmd = AbecsCommand::GetPin::new(
        "DIGITE A SENHA",
        4,    // Min
        12,   // Max
        "0",  // SEM criptografia (teste)
        "00", // Índice
        "",   // WK
        pan,
    );

    match pinpad.execute_typed(&cmd) {
        Ok(response) => {
            println!("\n✅ PIN capturado com sucesso!");
            println!("───────────────────────────────────────────────────────");
            println!("PIN Block: {} bytes", response.pin_block.len());
            println!("PIN Block (hex): {}", hex_string(&response.pin_block));
            println!("───────────────────────────────────────────────────────\n");
        }
        Err(e) => {
            println!("\n❌ Erro: {}\n", e);
            finalize(&mut pinpad, "ERRO")?;
            return Err(e.into());
        }
    }

    // Finalizar
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Finalizando...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    finalize(&mut pinpad, "SUCESSO")?;

    println!("═══════════════════════════════════════════════════════");
    println!("  ✅ Teste Concluído!");
    println!("═══════════════════════════════════════════════════════\n");

    Ok(())
}

fn hex_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join("")
}

fn finalize(
    pinpad: &mut PinpadConnection,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let msg = format!("032  {}     ", message);
    let cmd = AbecsCommand::Display::new(&msg);
    pinpad.execute_typed(&cmd)?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    let cmd = AbecsCommand::Close::new();
    pinpad.execute_typed(&cmd)?;

    Ok(())
}
