/// Exemplo de uso da biblioteca Pinpad ABECS
use pinpad::{AbecsCommand, PinpadConnection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════════════════");
    println!("  Protocolo ABECS 2.12 - Exemplo de Uso");
    println!("═══════════════════════════════════════════════════════\n");

    // Lista portas disponíveis
    println!("Portas seriais disponíveis:");
    let ports = PinpadConnection::list_ports()?;
    if ports.is_empty() {
        println!("  Nenhuma porta encontrada!");
        return Ok(());
    }
    for port in &ports {
        println!("  • {}", port);
    }

    // Conecta ao Pinpad
    let port_name = "/dev/ttyACM0"; // Ajuste conforme necessário
    println!("\n✓ Conectando em {}...", port_name);

    let mut pinpad = PinpadConnection::open(port_name)?;
    pinpad.set_verbose(true); // Ativa modo debug

    // Cancela qualquer comando anterior
    let _ = pinpad.cancel();

    // ═══════════════════════════════════════════════════════════
    // Exemplo 1: Comando OPN (Open)
    // ═══════════════════════════════════════════════════════════
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Exemplo 1: Abertura de Sessão");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let response = pinpad.execute(&AbecsCommand::open())?;

    if response.is_success() {
        println!("\n✓ Sessão aberta com sucesso!");
    } else {
        println!("\n✗ Erro: {}", response.status_description());
    }

    // ═══════════════════════════════════════════════════════════
    // Exemplo 2: Mostrar mensagem no display
    // ═══════════════════════════════════════════════════════════
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Exemplo 2: Exibir Mensagem");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let message = "032          BIBLIOTECA      PINPAD RUST     ";
    let response = pinpad.execute(&AbecsCommand::display(message))?;

    if response.is_success() {
        println!("\n✓ Mensagem exibida no Pinpad!");
    }

    // ═══════════════════════════════════════════════════════════
    // Exemplo 3: Obter informações do Pinpad
    // ═══════════════════════════════════════════════════════════
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Exemplo 3: Informações do Pinpad");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let response = pinpad.execute(&AbecsCommand::get_info("01"))?;

    if response.is_success() {
        println!("\n✓ Informações recebidas:");
        for i in 0..response.block_count() {
            if let Some(text) = response.get_string(i) {
                println!("  • {}", text.trim());
            }
        }
    }

    // ═══════════════════════════════════════════════════════════
    // Exemplo 4: Comando personalizado
    // ═══════════════════════════════════════════════════════════
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Exemplo 4: Comando Personalizado");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let mut cmd = AbecsCommand::new("DSP");
    cmd.add_string("032        FINALIZADO       ");

    let response = pinpad.execute(&cmd)?;
    response.print();

    println!("\n═══════════════════════════════════════════════════════");
    println!("  Exemplo finalizado com sucesso!");
    println!("═══════════════════════════════════════════════════════");

    Ok(())
}
