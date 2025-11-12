/// Exemplo de uso da biblioteca Pinpad ABECS
use pinpad::{AbecsCommand, DisplayCommand, GetInfoCommand, OpenCommand, PinpadConnection};

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
    // Exemplo 1: Comando OPN (Open) - API Antiga
    // ═══════════════════════════════════════════════════════════
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Exemplo 1: Abertura de Sessão (API Antiga)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let response = pinpad.execute(&AbecsCommand::open())?;

    if response.is_success() {
        println!("\n✓ Sessão aberta com sucesso!");
    } else {
        println!("\n✗ Erro: {}", response.status_description());
    }

    // ═══════════════════════════════════════════════════════════
    // Exemplo 2: Comando OPN (Open) - API Tipada
    // ═══════════════════════════════════════════════════════════
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Exemplo 2: Abertura de Sessão (API Tipada)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let command = OpenCommand;
    let _response = pinpad.execute_typed(&command)?;
    println!("\n✓ Sessão aberta com sucesso (API Tipada)!");

    // ═══════════════════════════════════════════════════════════
    // Exemplo 3: Mostrar mensagem no display - API Tipada
    // ═══════════════════════════════════════════════════════════
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Exemplo 3: Exibir Mensagem (API Tipada)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let command = DisplayCommand::new("032 BIBLIOTECA PINPAD RUST ");
    let _response = pinpad.execute_typed(&command)?;
    println!("\n✓ Mensagem exibida no Pinpad!");

    // ═══════════════════════════════════════════════════════════
    // Exemplo 4: Obter informações do Pinpad - API Tipada
    // ═══════════════════════════════════════════════════════════
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Exemplo 4: Informações do Pinpad (API Tipada)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let command = GetInfoCommand::new("01");
    let response = pinpad.execute_typed(&command)?;
    println!("\n✓ Informações recebidas:");
    println!("  • {}", response.info.trim());

    // ═══════════════════════════════════════════════════════════
    // Exemplo 5: Comando personalizado (API Antiga)
    // ═══════════════════════════════════════════════════════════
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Exemplo 5: Comando Personalizado (API Antiga)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let mut cmd = AbecsCommand::new("DSP");
    cmd.add_string("FINALIZADO");

    let response = pinpad.execute(&cmd)?;
    response.print();

    println!("\n═══════════════════════════════════════════════════════");
    println!("  Exemplo finalizado com sucesso!");
    println!("═══════════════════════════════════════════════════════");

    Ok(())
}
