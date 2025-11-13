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

    let mut attempts = 0;
    let max_attempts = 3;
    let card_result = loop {
        attempts += 1;

        let card_cmd = GetCard::new(
            1, // R$ 0,01
            date.clone(),
            time.clone(),
            60, // 60 segundos de timeout
        );

        match conn.execute_typed(&card_cmd) {
            Ok(result) => break result,
            Err(e) => {
                // Verifica se é erro 080 (múltiplos CTLS detectados)
                use pinpad::AbecsError;
                if let AbecsError::PinpadError { ref status, .. } = e {
                    if status == "080" {
                        println!(
                            "⚠️  Múltiplos cartões detectados! (tentativa {}/{})",
                            attempts, max_attempts
                        );

                        if attempts < max_attempts {
                            // Mostra mensagem no Pinpad
                            let msg =
                                format!("{:<16}{:<16}{:<16}", "APRESENTE", "APENAS UM", "CARTAO");
                            let _ = conn.execute_typed(&Display::new(&msg));

                            std::thread::sleep(std::time::Duration::from_secs(2));
                            println!("🔄 Tentando novamente...");
                            continue;
                        } else {
                            println!("❌ Transação cancelada após {} tentativas", max_attempts);
                            return Ok(());
                        }
                    }
                }

                // Outros erros: propaga
                return Err(e.into());
            }
        }
    };
    println!("✅ Cartão detectado!");
    println!(
        "📇 Tipo: {} (código {})",
        card_result.card_type,
        card_result.card_type.to_code()
    );

    if let Some(ref pan) = card_result.pan {
        println!("💳 PAN: {}", pan);
    }

    // 4. GTK - Obtém trilhas completas
    println!("\n4️⃣  Obtendo trilhas completas do cartão...");
    let tracks_cmd = GetTracks::new_plain(); // Sem criptografia

    let tracks_result = conn.execute_typed(&tracks_cmd)?;
    println!("✅ Trilhas obtidas!");

    // Parse estruturado da Track 1
    if let Some(track1_data) = tracks_result.parse_track1() {
        println!("\n╔══════════════════════════════════════════════╗");
        println!("║        INFORMAÇÕES DO CARTÃO                 ║");
        println!("╚══════════════════════════════════════════════╝");

        if let Some(ref pan) = track1_data.pan {
            println!("\n💳 PAN: {}", pan);
        }

        if let Some(ref name) = track1_data.cardholder_name {
            println!("👤 Nome: {}", name);
        }

        if track1_data.expiry_date.is_some() {
            if let Some(formatted) = track1_data.expiry_date_formatted() {
                println!("📅 Validade: {}", formatted);

                // Verifica se está expirado
                let is_expired = track1_data.is_expired(2025, 11);
                if is_expired {
                    println!("   ⚠️  Status: CARTÃO EXPIRADO");
                } else {
                    println!("   ✅ Status: Válido");
                }
            }
        }

        if let Some(ref sc) = track1_data.service_code {
            println!("\n🔧 Código de Serviço: {}", sc);
            println!(
                "💳 Método (estimativa): {} ⚠️ Pode estar incorreto!",
                track1_data.payment_method()
            );
            println!("   💡 Use a mensagem NTM do Pinpad como fonte confiável");
        }

        if let Some(ref dd) = track1_data.discretionary_data {
            println!("📋 Dados Discricionários: {}", dd);
        }

        // Mostra track raw para debug
        println!("\n🎫 Track 1 (raw): {}", track1_data.raw);
    } else {
        // Fallback: mostra trilhas brutas
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
