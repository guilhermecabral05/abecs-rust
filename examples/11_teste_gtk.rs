/// Exemplo 11: Teste do comando GTK (GetTracks)
///
/// Este exemplo demonstra como obter as trilhas completas do cartão
/// após a leitura com GCX.
///
/// Fluxo:
/// 1. OPN - Abre conexão
/// 2. DSP - Mostra mensagem
/// 3. GCX - Lê cartão
/// 4. GTK - Obtém trilhas completas (em claro)
/// 5. CLO - Fecha conexão
use pinpad::AbecsCommand::{Close, Display, GetCard, GetTracks, Open};
use pinpad::PinpadConnection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════");
    println!("  Teste GTK - Obter Trilhas do Cartão");
    println!("═══════════════════════════════════════════\n");

    // Conecta ao Pinpad
    let port = "/dev/ttyACM0";
    let mut conn = PinpadConnection::open(port)?;
    conn.set_verbose(true);

    // 1. OPN - Abre sessão
    println!("\n1️⃣  Abrindo sessão com Pinpad...");
    let open_cmd = Open::new();
    conn.execute_typed(&open_cmd)?;
    println!("✅ Sessão aberta");

    // 2. DSP - Mostra mensagem
    println!("\n2️⃣  Mostrando mensagem no Pinpad...");
    let msg = format!(
        "{:<16}{:<16}{:<16}",
        "APROXIME,", "INSIRA OU", "PASSE CARTAO"
    );
    let display_cmd = Display::new(&msg);
    conn.execute_typed(&display_cmd)?;
    println!("✅ Mensagem exibida");

    // 3. GCX - Lê o cartão
    println!("\n3️⃣  Aguardando cartão...");

    // Data/hora simplificadas para teste
    let date = "251111".to_string(); // 25/11/11
    let time = "173000".to_string(); // 17:30:00

    let card_cmd = GetCard::new(
        1, // R$ 0,01
        date, time, 60, // 60 segundos de timeout
    );

    let card_result = conn.execute_typed(&card_cmd)?;
    println!("✅ Cartão detectado!");
    println!("📇 Tipo: {} (código {})", card_result.card_type, card_result.card_type.to_code());
    
    if let Some(ref pan) = card_result.pan {
        println!("💳 PAN: {}", pan);
    }

    // 4. GTK - Obtém trilhas completas
    println!("\n4️⃣  Obtendo trilhas completas do cartão...");
    let tracks_cmd = GetTracks::new_plain(); // Sem criptografia

    let tracks_result = conn.execute_typed(&tracks_cmd)?;
    println!("✅ Trilhas obtidas!");

    // Mostra trilhas obtidas
    if let Some(pan_str) = tracks_result.pan_as_string() {
        println!("\n💳 PAN: {}", pan_str);
    }

    if let Some(t1_str) = tracks_result.track1_as_string() {
        println!("\n🎫 Trilha 1: {}", t1_str);
    }

    if let Some(t2_str) = tracks_result.track2_as_string() {
        println!("\n🎫 Trilha 2: {}", t2_str);
    }

    if let Some(t3_str) = tracks_result.track3_as_string() {
        println!("\n🎫 Trilha 3: {}", t3_str);
    }

    // Mostra formato hexadecimal se necessário (para debug)
    if tracks_result.is_encrypted() {
        println!("\n🔐 Dados criptografados detectados!");
        
        if let Some(ref pan) = tracks_result.pan {
            println!("   PAN HEX: {}", hex_format(pan));
        }
        if let Some(ref ksn) = tracks_result.pan_ksn {
            println!("   PAN KSN: {}", hex_format(ksn));
        }
    }

    // 5. CLO - Fecha sessão
    println!("\n5️⃣  Fechando sessão...");
    let close_cmd = Close::new();
    conn.execute_typed(&close_cmd)?;
    println!("✅ Sessão fechada");

    println!("\n═══════════════════════════════════════════");
    println!("✅ Teste GTK concluído com sucesso!");
    println!("═══════════════════════════════════════════");

    Ok(())
}

fn hex_format(data: &[u8]) -> String {
    data.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}
