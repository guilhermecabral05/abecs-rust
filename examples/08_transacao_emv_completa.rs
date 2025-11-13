/// Exemplo 08: Transação EMV Completa (Chip/Contactless)
///
/// Este exemplo demonstra um fluxo completo de transação EMV com chip:
/// 1. Abrir sessão (OPN)
/// 2. Obter informações do Pinpad (GIX)
/// 3. Ler cartão (GCX - Get Card Extended)
/// 4. Processar EMV (GOX - Go On Chip) - se for cartão com chip
/// 5. Finalizar EMV (FCX - Finish Chip) - se for cartão com chip
/// 6. Exibir resultado
///
/// ⚠️  Este é um exemplo educacional! Em produção você precisa:
///     • Carregar tabelas AID e CAPK (TLI/TLR/TLF)
///     • Conectar com processadora real
///     • Implementar tratamento completo de erros
///     • Gerenciar criptografia DUKPT corretamente
///
/// Execute com: cargo run --example 08_transacao_emv_completa
use pinpad::{AbecsCommand, EmvData, PinpadConnection};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════════════════");
    println!("  Exemplo 08: Transação EMV Completa");
    println!("═══════════════════════════════════════════════════════\n");

    println!("💳 Simulando uma transação EMV (chip/contactless)...\n");

    // Dados da transação
    let valor = 25000u64; // R$ 250,00 em centavos
    let reais = valor / 100;
    let centavos = valor % 100;

    // Conectar ao Pinpad
    let port_name = "/dev/ttyACM0";
    println!("🔌 Conectando em {}...", port_name);
    let mut pinpad = PinpadConnection::open(port_name)?;
    println!("✅ Conectado!\n");

    // ═══════════════════════════════════════════════════════════
    // ETAPA 1: Abrir Sessão (OPN)
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 1: Inicializar Sessão");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cmd = AbecsCommand::Open::new();
    pinpad.execute_typed(&cmd)?;
    println!("✅ Sessão iniciada\n");

    // ═══════════════════════════════════════════════════════════
    // ETAPA 2: Obter Informações do Pinpad (GIX)
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 2: Informações do Pinpad");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cmd = AbecsCommand::GetInfo::new("00"); // GIN_ACQIDX deve ser 2 dígitos
    let info_response = pinpad.execute_typed(&cmd)?;
    println!("📱 Pinpad: {}\n", info_response.info.trim());

    // ═══════════════════════════════════════════════════════════
    // ETAPA 3: Exibir Valor da Transação
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 3: Exibir Valor");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mensagem = format!("032   VALOR: R$      {},{:02}          ", reais, centavos);
    println!("💰 Valor: R$ {},{:02}", reais, centavos);

    let cmd = AbecsCommand::Display::new(&mensagem);
    pinpad.execute_typed(&cmd)?;

    std::thread::sleep(std::time::Duration::from_secs(2));

    // ═══════════════════════════════════════════════════════════
    // ETAPA 4: Leitura do Cartão (GCX)
    // ═══════════════════════════════════════════════════════════
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 4: Leitura do Cartão");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("📱 Aguardando cartão no Pinpad...");
    println!("   Insira o chip, passe ou aproxime o cartão\n");

    // Obter data e hora atuais
    let (date, time) = get_current_datetime();

    let cmd = AbecsCommand::GetCard::new(
        valor,        // Valor em centavos
        date.clone(), // Data AAMMDD
        time.clone(), // Hora HHMMSS
        60,           // Timeout 60 segundos
    )
    .with_message("INSIRA OU APROXIME");

    let card_response = match pinpad.execute_typed(&cmd) {
        Ok(response) => {
            println!("✅ Cartão detectado: {}", response.card_type);
            println!(
                "   Tipo: {} (código {})",
                response.card_type,
                response.card_type.to_code()
            );

            if let Some(ref pan) = response.pan {
                let pan_mask = mask_pan(pan);
                println!("   PAN: {}", pan_mask);
            }

            if let Some(ref icc_status) = response.icc_status {
                println!("   ICC Status: {}", icc_status);
            }

            if let Some(ref emv) = response.emv_data {
                println!("   Dados EMV: {} tags encontradas", emv.tags().len());
            }

            println!();
            response
        }
        Err(pinpad::AbecsError::UserCancelled) => {
            println!("❌ Operação cancelada pelo usuário\n");
            finalize_and_exit(&mut pinpad, "CANCELADO")?;
            return Ok(());
        }
        Err(e) => {
            println!("❌ Erro na leitura: {}\n", e);
            finalize_and_exit(&mut pinpad, "ERRO CARTAO")?;
            return Err(e.into());
        }
    };

    // Determinar se é transação EMV (chip)
    let is_emv = card_response.card_type.is_emv();

    if !is_emv {
        println!("⚠️  Cartão não é EMV (chip)");
        println!("   Este exemplo foca em transações EMV");
        println!("   Para tarja magnética, veja exemplo 07\n");
        finalize_and_exit(&mut pinpad, "USAR CHIP")?;
        return Ok(());
    }

    // ═══════════════════════════════════════════════════════════
    // ETAPA 5: Processar EMV (GOX - Go On Chip)
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 5: Processar Chip EMV (GOX)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cmd = AbecsCommand::Display::new("032  PROCESSANDO... ");
    pinpad.execute_typed(&cmd)?;

    println!("⏳ Executando GOX (Go On Chip)...");

    // Parâmetros do terminal (simplificados para exemplo)
    // Em produção, estes devem vir da configuração do terminal
    let terminal_params = vec![
        0x9F, 0x33, 0x03, 0xE0, 0xF8, 0xC8, // Terminal Capabilities
        0x9F, 0x1A, 0x02, 0x07, 0x6C, // Terminal Country Code (Brasil)
        0x9F, 0x35, 0x01, 0x22, // Terminal Type
        0x5F, 0x2A, 0x02, 0x09, 0x86, // Transaction Currency Code (BRL)
    ];

    let gox_cmd = AbecsCommand::GoOnChip::new(
        "04", // App Type: Débito
        valor,
        date.clone(),
        time.clone(),
        terminal_params,
    )
    .with_currency("0986"); // BRL

    let gox_response = match pinpad.execute_typed(&gox_cmd) {
        Ok(response) => {
            println!("✅ GOX concluído!");
            println!("   Resultado: {}", response.gox_result);

            // Interpretar resultado GOX (6 dígitos: XXYYZZ)
            if response.gox_result.len() >= 6 {
                let status = &response.gox_result[0..2];
                let pin_required = &response.gox_result[2..4];
                let result = &response.gox_result[4..6];

                println!("   Status: {}", status);
                println!(
                    "   PIN necessário: {}",
                    if pin_required == "01" { "Sim" } else { "Não" }
                );
                println!("   Resultado transação: {}", result);
            }

            if let Some(ref emv) = response.emv_data {
                println!("   Dados EMV retornados: {} tags", emv.tags().len());

                // Exibir algumas tags importantes
                if let Some(cryptogram) = emv.get_tag(&[0x9F, 0x26]) {
                    println!("   Application Cryptogram: {}", hex_string(cryptogram));
                }
                if let Some(cid) = emv.get_tag(&[0x9F, 0x27]) {
                    println!("   Cryptogram Information Data: {}", hex_string(cid));
                }
                if let Some(atc) = emv.get_tag(&[0x9F, 0x36]) {
                    println!("   Application Transaction Counter: {}", hex_string(atc));
                }
            }

            if response.pin_block.is_some() {
                println!("   ✅ PIN capturado e criptografado");
            }

            println!();
            response
        }
        Err(e) => {
            println!("❌ Erro no processamento EMV: {}\n", e);
            finalize_and_exit(&mut pinpad, "ERRO EMV")?;
            return Err(e.into());
        }
    };

    // ═══════════════════════════════════════════════════════════
    // ETAPA 6: Enviar para Processadora (Simulado)
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 6: Comunicação com Processadora");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("⏳ Enviando dados EMV para processadora...");
    std::thread::sleep(std::time::Duration::from_secs(1));

    println!("⏳ Aguardando autorização...");
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Simular resposta da processadora
    // Em produção: enviar cryptogram, PAN, dados EMV para adquirente
    let aprovado = true; // Simulado
    let arc = if aprovado { "00" } else { "05" }; // Authorization Response Code
    let codigo_autorizacao = "123456";

    println!("✅ Resposta recebida");
    println!("   ARC: {}", arc);
    println!("   Código: {}\n", codigo_autorizacao);

    // ═══════════════════════════════════════════════════════════
    // ETAPA 7: Finalizar EMV (FCX - Finish Chip)
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 7: Finalizar Chip EMV (FCX)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("⏳ Executando FCX (Finish Chip)...");

    // Preparar dados EMV do issuer (se houver)
    // Em produção: pode incluir issuer scripts, etc
    let mut issuer_emv = EmvData::new();
    // Adicionar Authorization Response Code ao EMV
    issuer_emv.add_tag(&[0x8A], arc.as_bytes());

    let fcx_cmd = AbecsCommand::FinishChip::new(arc).with_emv_data(issuer_emv);

    let fcx_response = match pinpad.execute_typed(&fcx_cmd) {
        Ok(response) => {
            println!("✅ FCX concluído!");
            println!("   Resultado: {}", response.fcx_result);

            // Interpretar resultado FCX
            match response.fcx_result.as_str() {
                "000" => println!("   ✅ Transação APROVADA pelo chip"),
                "001" => println!("   ❌ Transação NEGADA pelo chip"),
                _ => println!("   ⚠️  Status desconhecido"),
            }

            if let Some(ref emv) = response.emv_data {
                println!("   Dados EMV finais: {} tags", emv.tags().len());
            }

            println!();
            response
        }
        Err(e) => {
            println!("❌ Erro ao finalizar: {}\n", e);
            finalize_and_exit(&mut pinpad, "ERRO FCX")?;
            return Err(e.into());
        }
    };

    // ═══════════════════════════════════════════════════════════
    // ETAPA 8: Exibir Resultado Final
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 8: Resultado Final");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let fcx_approved = fcx_response.fcx_result == "000";
    let transaction_approved = aprovado && fcx_approved;

    if transaction_approved {
        println!("✅ TRANSAÇÃO APROVADA!");
        println!("   Código: {}", codigo_autorizacao);
        println!("   Valor: R$ {},{:02}", reais, centavos);
        println!("   Tipo: EMV Chip\n");

        let cmd = AbecsCommand::Display::new("032   APROVADO!     ");
        pinpad.execute_typed(&cmd)?;
        std::thread::sleep(std::time::Duration::from_secs(2));

        let msg = format!("032  CODIGO: {}  ", codigo_autorizacao);
        let cmd = AbecsCommand::Display::new(&msg);
        pinpad.execute_typed(&cmd)?;
        std::thread::sleep(std::time::Duration::from_secs(2));
    } else {
        println!("❌ TRANSAÇÃO NEGADA\n");
        if !aprovado {
            println!("   Motivo: Negada pela processadora");
        } else if !fcx_approved {
            println!("   Motivo: Negada pelo chip do cartão");
        }
        println!();

        let cmd = AbecsCommand::Display::new("032   NEGADO!       ");
        pinpad.execute_typed(&cmd)?;
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    // ═══════════════════════════════════════════════════════════
    // ETAPA 9: Finalizar
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Finalizando...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cmd = AbecsCommand::Display::new("032   OBRIGADO!     ");
    pinpad.execute_typed(&cmd)?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    let cmd = AbecsCommand::Close::new();
    pinpad.execute_typed(&cmd)?;

    println!("═══════════════════════════════════════════════════════");
    println!("  ✅ Transação EMV Completa!");
    println!("═══════════════════════════════════════════════════════");

    println!("\n📝 Resumo da transação EMV:");
    println!(
        "   Status: {}",
        if transaction_approved {
            "APROVADA"
        } else {
            "NEGADA"
        }
    );
    println!(
        "   Tipo Cartão: {} ({})",
        card_response.card_type,
        card_response.card_type.to_code()
    );
    println!("   Valor: R$ {},{:02}", reais, centavos);
    if transaction_approved {
        println!("   Código: {}", codigo_autorizacao);
        println!("   ARC: {}", arc);
    }
    println!("   GOX Resultado: {}", gox_response.gox_result);
    println!("   FCX Resultado: {}", fcx_response.fcx_result);
    println!();

    println!("💡 Fluxo EMV Completo:");
    println!("   1. ✅ OPN - Sessão iniciada");
    println!("   2. ✅ GIX - Informações obtidas");
    println!("   3. ✅ GCX - Cartão lido");
    println!("   4. ✅ GOX - Processamento EMV executado");
    println!("   5. ✅ FCX - Transação finalizada no chip");
    println!("   6. ✅ CLO - Sessão fechada\n");

    println!("📚 Para produção, você DEVE:");
    println!("   • Carregar tabelas AID (TLI/TLR/TLF)");
    println!("   • Carregar tabelas CAPK (chaves públicas)");
    println!("   • Implementar integração real com processadora");
    println!("   • Validar cryptograms e certificados");
    println!("   • Implementar fluxo completo de exceções EMV");
    println!("   • Gerenciar logs e auditoria");
    println!("   • Implementar estornos e cancelamentos");
    println!("   • Testar com certificação EMV\n");

    Ok(())
}

// ═══════════════════════════════════════════════════════════
// Funções Auxiliares
// ═══════════════════════════════════════════════════════════

/// Obter data e hora atual no formato ABECS (AAMMDD e HHMMSS)
fn get_current_datetime() -> (String, String) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Cálculo aproximado (para exemplo - em produção use chrono ou similar)
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

/// Mascarar PAN para exibição segura
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

/// Converter bytes para string hexadecimal
fn hex_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join("")
}

/// Finalizar sessão e exibir mensagem de erro
fn finalize_and_exit(
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
