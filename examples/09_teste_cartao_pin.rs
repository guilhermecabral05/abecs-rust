/// Exemplo 09: Teste Básico - Leitura de Cartão e PIN
///
/// Este exemplo foca APENAS em testar:
/// 1. Conectar ao Pinpad
/// 2. Ler cartão (chip, tarja ou contactless)
/// 3. Capturar PIN
///
/// Execute com: cargo run --example 09_teste_cartao_pin
use pinpad::{AbecsCommand, PinpadConnection};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════════════════");
    println!("  Teste Básico: Leitura de Cartão e PIN");
    println!("═══════════════════════════════════════════════════════\n");

    // ═══════════════════════════════════════════════════════════
    // Conectar ao Pinpad
    // ═══════════════════════════════════════════════════════════
    let port_name = "/dev/ttyACM0";
    println!("🔌 Conectando em {}...", port_name);
    let mut pinpad = PinpadConnection::open(port_name)?;

    // Ativar modo verbose para ver os bytes trocados
    pinpad.set_verbose(true);

    println!("✅ Conectado!\n");

    // ═══════════════════════════════════════════════════════════
    // Abrir Sessão
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 1: Abrir Sessão");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cmd = AbecsCommand::Open::new();
    pinpad.execute_typed(&cmd)?;
    println!("✅ Sessão aberta\n");

    // ═══════════════════════════════════════════════════════════
    // Obter Informações do Pinpad
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 2: Informações do Pinpad");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cmd = AbecsCommand::GetInfo::new("00"); // GIN_ACQIDX deve ser 2 dígitos
    let info = pinpad.execute_typed(&cmd)?;
    println!("📱 Info: {}\n", info.info);

    // ═══════════════════════════════════════════════════════════
    // Ler Cartão
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 3: Leitura de Cartão");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let valor = 10000u64; // R$ 100,00
    let (date, time) = get_current_datetime();

    println!("💰 Valor: R$ {},{:02}", valor / 100, valor % 100);
    println!("📅 Data: {} Hora: {}", date, time);
    println!("\n📱 Por favor, insira, passe ou aproxime o cartão...\n");

    let cmd = AbecsCommand::GetCard::new(
        valor,
        date.clone(),
        time.clone(),
        60, // 60 segundos de timeout
    );

    let card_response = match pinpad.execute_typed(&cmd) {
        Ok(response) => {
            println!("\n✅ Cartão detectado!");
            println!("───────────────────────────────────────────────────────");

            // Tipo de cartão
            println!("Tipo: {} (código {})", response.card_type, response.card_type.to_code());

            // PAN
            if let Some(ref pan) = response.pan {
                println!("PAN: {}", mask_pan(pan));
            }

            // Tracks
            if let Some(ref track1) = response.track1 {
                println!("Track 1: {} bytes", track1.len());
            }
            if let Some(ref track2) = response.track2 {
                println!("Track 2: {} bytes", track2.len());
            }
            if let Some(ref track3) = response.track3 {
                println!("Track 3: {} bytes", track3.len());
            }

            // Status ICC
            if let Some(ref status) = response.icc_status {
                println!("ICC Status: {}", status);
            }

            // Dados EMV
            if let Some(ref emv) = response.emv_data {
                println!("Dados EMV: {} tags encontradas", emv.tags().len());

                // Mostrar algumas tags importantes
                if let Some(aid) = emv.get_tag(&[0x4F]) {
                    println!("  AID: {}", hex_string(aid));
                }
                if let Some(app_label) = emv.get_tag(&[0x50]) {
                    println!(
                        "  Application Label: {}",
                        String::from_utf8_lossy(app_label)
                    );
                }
            }

            // Info da tabela AID
            if let Some(ref aid_info) = response.aid_table_info {
                println!("AID Table Info: {} bytes", aid_info.len());
            }

            println!("───────────────────────────────────────────────────────\n");
            response
        }
        Err(e) => {
            println!("\n❌ Erro ao ler cartão: {}\n", e);
            finalize(&mut pinpad, "ERRO CARTAO")?;
            return Err(e.into());
        }
    };

    // Extrair PAN para usar no PIN
    let pan_for_pin = card_response
        .pan
        .as_ref()
        .map(|p| p.clone())
        .unwrap_or_else(|| "0000000000000000".to_string());

    // ═══════════════════════════════════════════════════════════
    // Capturar PIN
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 4: Captura de PIN");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("🔐 Por favor, digite a senha no Pinpad...\n");

    // Método de criptografia:
    // "0" = sem criptografia (APENAS PARA TESTE!)
    // "1" = MK/WK (Master Key / Working Key)
    // "2" = DUKPT
    let crypto_method = "0"; // SEM CRIPTOGRAFIA PARA TESTE

    let cmd = AbecsCommand::GetPin::new(
        "DIGITE A SENHA",
        4,             // Min length
        12,            // Max length
        crypto_method, // Método de criptografia
        "00",          // Índice de chave
        "",            // Working Key (não usado em modo 0)
        &pan_for_pin,  // PAN
    );

    match pinpad.execute_typed(&cmd) {
        Ok(response) => {
            println!("\n✅ PIN capturado!");
            println!("───────────────────────────────────────────────────────");
            println!("PIN Block: {} bytes", response.pin_block.len());
            println!("PIN Block (hex): {}", hex_string(&response.pin_block));
            println!("───────────────────────────────────────────────────────\n");
        }
        Err(e) => {
            println!("\n❌ Erro ao capturar PIN: {}\n", e);
            finalize(&mut pinpad, "ERRO PIN")?;
            return Err(e.into());
        }
    }

    // ═══════════════════════════════════════════════════════════
    // Finalizar
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Finalizando...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    finalize(&mut pinpad, "SUCESSO!")?;

    println!("═══════════════════════════════════════════════════════");
    println!("  ✅ Teste Concluído com Sucesso!");
    println!("═══════════════════════════════════════════════════════\n");

    println!("📋 Resumo:");
    println!("   • Tipo de Cartão: {} ({})", card_response.card_type, card_response.card_type.to_code());
    if let Some(pan) = card_response.pan {
        println!("   • PAN: {}", mask_pan(&pan));
    }
    println!("   • PIN Capturado: ✅");
    println!();

    println!("💡 Próximos passos:");
    println!("   1. Se funcionou, configure criptografia real (DUKPT ou MK/WK)");
    println!("   2. Implemente processamento EMV para cartões com chip");
    println!("   3. Integre com processadora de pagamentos");
    println!();

    Ok(())
}

// ═══════════════════════════════════════════════════════════
// Funções Auxiliares
// ═══════════════════════════════════════════════════════════

fn get_current_datetime() -> (String, String) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let dias_desde_1970 = now / 86400;
    let ano = ((1970 + (dias_desde_1970 / 365)) % 100) as u32;
    let mes = (((dias_desde_1970 % 365) / 30) + 1).min(12) as u32;
    let dia = (((dias_desde_1970 % 365) % 30) + 1).min(28) as u32;

    let horas = ((now % 86400) / 3600) as u32;
    let minutos = ((now % 3600) / 60) as u32;
    let segundos = (now % 60) as u32;

    let date = format!("{:02}{:02}{:02}", ano, mes, dia);
    let time = format!("{:02}{:02}{:02}", horas, minutos, segundos);

    (date, time)
}

fn mask_pan(pan: &str) -> String {
    if pan.len() >= 10 {
        let first = &pan[..6];
        let last = &pan[pan.len() - 4..];
        format!("{}******{}", first, last)
    } else if pan.len() >= 4 {
        format!("****{}", &pan[pan.len() - 4..])
    } else {
        "****".to_string()
    }
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
