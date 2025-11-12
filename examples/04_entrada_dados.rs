/// Exemplo 04: Entrada de Dados
///
/// Este exemplo demonstra como capturar dados digitados pelo usuário no Pinpad.
/// Útil para capturar valores, códigos, CPF, etc.
///
/// ⚠️  ATENÇÃO: Este é um comando BLOCANTE!
/// O programa aguardará até que o usuário digite os dados ou o timeout expire.
///
/// Execute com: cargo run --example 04_entrada_dados
use pinpad::{AbecsCommand, PinpadConnection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════════════════");
    println!("  Exemplo 04: Entrada de Dados");
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
    // Exemplo 1: Capturar valor
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Capturando valor da transação...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("💡 Aguardando digitação no Pinpad...");
    println!("   Mínimo: 1 dígito");
    println!("   Máximo: 10 dígitos");
    println!("   Timeout: 60 segundos\n");

    let cmd = AbecsCommand::GetData::new(
        "DIGITE O VALOR", // Mensagem
        1,                // Mínimo de caracteres
        10,               // Máximo de caracteres
        60,               // Timeout em segundos
    );

    match pinpad.execute_typed(&cmd) {
        Ok(response) => {
            println!("✅ Valor digitado: {}\n", response.data);

            // Tentar parsear como valor monetário (centavos)
            if let Ok(valor) = response.data.parse::<u64>() {
                let reais = valor / 100;
                let centavos = valor % 100;
                println!("   💰 R$ {},{:02}\n", reais, centavos);
            }
        }
        Err(e) => {
            println!("❌ Erro ou timeout: {}\n", e);
        }
    }

    // ═══════════════════════════════════════════════════════════
    // Exemplo 2: Capturar código
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Capturando código de autorização...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("💡 Aguardando código (6 dígitos)...\n");

    let cmd = AbecsCommand::GetData::new(
        "CODIGO AUTORIZACAO",
        6, // Exatamente 6 dígitos
        6,
        30,
    );

    match pinpad.execute_typed(&cmd) {
        Ok(response) => {
            println!("✅ Código: {}\n", response.data);
        }
        Err(e) => {
            println!("❌ Erro ou timeout: {}\n", e);
        }
    }

    // ═══════════════════════════════════════════════════════════
    // Exemplo 3: Capturar CPF
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Capturando CPF...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("💡 Digite o CPF (11 dígitos)...\n");

    let cmd = AbecsCommand::GetData::new(
        "DIGITE SEU CPF",
        11, // CPF tem 11 dígitos
        11,
        45,
    );

    match pinpad.execute_typed(&cmd) {
        Ok(response) => {
            let cpf = &response.data;
            // Formatar CPF: XXX.XXX.XXX-XX
            if cpf.len() == 11 {
                let formatted = format!(
                    "{}.{}.{}-{}",
                    &cpf[0..3],
                    &cpf[3..6],
                    &cpf[6..9],
                    &cpf[9..11]
                );
                println!("✅ CPF: {}\n", formatted);
            } else {
                println!("✅ Dados: {}\n", cpf);
            }
        }
        Err(e) => {
            println!("❌ Erro ou timeout: {}\n", e);
        }
    }

    // Fechar sessão
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let cmd = AbecsCommand::Close::new();
    pinpad.execute_typed(&cmd)?;

    println!("\n═══════════════════════════════════════════════════════");
    println!("  ✅ Exemplo concluído com sucesso!");
    println!("═══════════════════════════════════════════════════════");

    Ok(())
}
