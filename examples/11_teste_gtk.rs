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
    println!("📇 Tipo: {}", card_result.card_type);

    // 4. GTK - Obtém trilhas completas
    println!("\n4️⃣  Obtendo trilhas completas do cartão...");
    let tracks_cmd = GetTracks::new_plain(); // Sem criptografia

    let tracks_result = conn.execute_typed(&tracks_cmd)?;
    println!("✅ Trilhas obtidas!");

    // Mostra trilhas obtidas
    if let Some(ref pan) = tracks_result.pan {
        println!("\n💳 PAN: {} bytes", pan.len());
        println!("   HEX: {}", hex_format(pan));
        println!("   ASCII: {}", ascii_format(pan));
    }

    if let Some(ref t1) = tracks_result.track1 {
        println!("\n🎫 Trilha 1: {} bytes", t1.len());
        println!("   HEX: {}", hex_format(t1));
        println!("   ASCII: {}", ascii_format(t1));
    }

    if let Some(ref t2) = tracks_result.track2 {
        println!("\n🎫 Trilha 2: {} bytes", t2.len());
        println!("   HEX: {}", hex_format(t2));
        println!("   ASCII: {}", ascii_format(t2));
    }

    if let Some(ref t3) = tracks_result.track3 {
        println!("\n🎫 Trilha 3: {} bytes", t3.len());
        println!("   HEX: {}", hex_format(t3));
        println!("   ASCII: {}", ascii_format(t3));
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

fn ascii_format(data: &[u8]) -> String {
    data.iter()
        .map(|&b| {
            if b >= 0x20 && b <= 0x7E {
                b as char
            } else {
                '.'
            }
        })
        .collect()
}
