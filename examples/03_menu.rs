/// Exemplo 03: Menu Interativo
///
/// Este exemplo demonstra como criar um menu de seleção no Pinpad.
/// O usuário pode escolher entre várias opções usando as teclas do Pinpad.
///
/// ⚠️  ATENÇÃO: Este é um comando BLOCANTE!
/// O programa aguardará até que o usuário selecione uma opção ou o timeout expire.
///
/// Execute com: cargo run --example 03_menu
use pinpad::{AbecsCommand, PinpadConnection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════════════════");
    println!("  Exemplo 03: Menu Interativo");
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
    // Criar menu de formas de pagamento
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Exibindo menu no Pinpad...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let options = vec![
        "1 - CREDITO".to_string(),
        "2 - DEBITO".to_string(),
        "3 - VOUCHER".to_string(),
        "4 - PIX".to_string(),
    ];

    println!("💡 Aguardando seleção do usuário no Pinpad...");
    println!("   Timeout: 30 segundos\n");

    let cmd = AbecsCommand::Menu::new(
        "FORMA DE PAGAMENTO", // Título
        options.clone(),      // Opções
        30,                   // Timeout em segundos
    );

    match pinpad.execute_typed(&cmd) {
        Ok(response) => {
            println!("✅ Opção selecionada: {}", response.selected_index + 1);
            println!("   {}\n", options[response.selected_index as usize]);
        }
        Err(pinpad::AbecsError::UserCancelled) => {
            println!("❌ Operação cancelada pelo usuário (botão vermelho)\n");
            // Fechar sessão antes de sair
            let cmd = AbecsCommand::Close::new();
            let _ = pinpad.execute_typed(&cmd);
            return Ok(());
        }
        Err(e) => {
            println!("❌ Erro ou timeout: {}\n", e);
        }
    }

    // ═══════════════════════════════════════════════════════════
    // Segundo exemplo: Menu de confirmação
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Segundo menu: Confirmação");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let confirm_options = vec!["1 - SIM".to_string(), "2 - NAO".to_string()];

    println!("💡 Aguardando confirmação...\n");

    let cmd = AbecsCommand::Menu::new("CONFIRMA VALOR?", confirm_options.clone(), 20);

    match pinpad.execute_typed(&cmd) {
        Ok(response) => {
            let selected = &confirm_options[response.selected_index as usize];
            println!("✅ Resposta: {}\n", selected);

            if selected.contains("SIM") {
                println!("   ✓ Transação confirmada!");
            } else {
                println!("   ✗ Transação cancelada!");
            }
        }
        Err(pinpad::AbecsError::UserCancelled) => {
            println!("❌ Operação cancelada pelo usuário (botão vermelho)\n");
        }
        Err(e) => {
            println!("❌ Erro ou timeout: {}\n", e);
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
