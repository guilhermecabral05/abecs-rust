/// Exemplo 07: Fluxo Completo de Transação
///
/// Este exemplo demonstra um fluxo completo de transação com cartão:
/// 1. Exibir valor
/// 2. Selecionar forma de pagamento
/// 3. Capturar PIN
/// 4. Processar (simulado)
/// 5. Exibir resultado
///
/// ⚠️  Este é um exemplo educacional simplificado!
///
/// Execute com: cargo run --example 07_transacao_completa
use pinpad::{AbecsCommand, PinpadConnection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════════════════");
    println!("  Exemplo 07: Fluxo Completo de Transação");
    println!("═══════════════════════════════════════════════════════\n");

    println!("🛒 Simulando uma transação de pagamento...\n");

    // Dados da transação
    let valor = 15000u64; // R$ 150,00 em centavos
    let pan = "1234567890123456";

    // Conectar ao Pinpad
    let port_name = "/dev/ttyACM1";
    println!("🔌 Conectando em {}...", port_name);
    let mut pinpad = PinpadConnection::open(port_name)?;
    println!("✅ Conectado!\n");

    // Abrir sessão
    let cmd = AbecsCommand::Open::new();
    pinpad.execute_typed(&cmd)?;
    println!("✅ Sessão iniciada\n");

    // ═══════════════════════════════════════════════════════════
    // ETAPA 1: Exibir valor da transação
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 1: Exibir Valor");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let reais = valor / 100;
    let centavos = valor % 100;
    let mensagem = format!("032   VALOR: R$      {},{:02}          ", reais, centavos);

    println!("💰 Valor: R$ {},{:02}", reais, centavos);

    let cmd = AbecsCommand::Display::new(&mensagem);
    pinpad.execute_typed(&cmd)?;

    std::thread::sleep(std::time::Duration::from_secs(2));

    // ═══════════════════════════════════════════════════════════
    // ETAPA 2: Selecionar forma de pagamento
    // ═══════════════════════════════════════════════════════════
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 2: Selecionar Forma de Pagamento");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let opcoes = vec![
        "1 - DEBITO".to_string(),
        "2 - CREDITO VISTA".to_string(),
        "3 - CREDITO PARCELADO".to_string(),
    ];

    println!("💳 Aguardando seleção no Pinpad...");

    let cmd = AbecsCommand::Menu::new("FORMA PAGAMENTO", opcoes.clone(), 30);

    let forma_pagamento = match pinpad.execute_typed(&cmd) {
        Ok(response) => {
            let opcao = &opcoes[response.selected_index as usize];
            println!("✅ Selecionado: {}\n", opcao);
            opcao.clone()
        }
        Err(pinpad::AbecsError::UserCancelled) => {
            println!("❌ Operação cancelada pelo usuário (botão vermelho)\n");
            let cmd = AbecsCommand::Display::new("032  CANCELADO      ");
            pinpad.execute_typed(&cmd)?;
            std::thread::sleep(std::time::Duration::from_secs(2));
            let cmd = AbecsCommand::Close::new();
            pinpad.execute_typed(&cmd)?;
            return Ok(());
        }
        Err(e) => {
            println!("❌ Erro: {}\n", e);
            let cmd = AbecsCommand::Display::new("032     ERRO        ");
            pinpad.execute_typed(&cmd)?;
            std::thread::sleep(std::time::Duration::from_secs(2));
            let cmd = AbecsCommand::Close::new();
            pinpad.execute_typed(&cmd)?;
            return Ok(());
        }
    };

    // ═══════════════════════════════════════════════════════════
    // ETAPA 3: Inserir/Passar cartão (simulado)
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 3: Leitura do Cartão");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cmd = AbecsCommand::Display::new("032 INSIRA O CARTAO ");
    pinpad.execute_typed(&cmd)?;

    println!("📱 Aguardando cartão...");
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("✅ Cartão detectado!");
    println!("   PAN: ****{}\n", &pan[12..]);

    // ═══════════════════════════════════════════════════════════
    // ETAPA 4: Capturar PIN (se débito ou crédito com senha)
    // ═══════════════════════════════════════════════════════════
    if forma_pagamento.contains("DEBITO") || forma_pagamento.contains("PARCELADO") {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("ETAPA 4: Captura de PIN");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        println!("🔐 Aguardando senha no Pinpad...");

        let cmd = AbecsCommand::GetPin::new("DIGITE A SENHA", 4, 12, 30, "01", pan);

        match pinpad.execute_typed(&cmd) {
            Ok(response) => {
                println!("✅ PIN capturado!");
                println!("   PIN Block: {} bytes\n", response.pin_block.len());
            }
            Err(pinpad::AbecsError::UserCancelled) => {
                println!("❌ Operação cancelada pelo usuário (botão vermelho)\n");
                let cmd = AbecsCommand::Display::new("032  CANCELADO      ");
                pinpad.execute_typed(&cmd)?;
                std::thread::sleep(std::time::Duration::from_secs(2));
                let cmd = AbecsCommand::Close::new();
                pinpad.execute_typed(&cmd)?;
                return Ok(());
            }
            Err(e) => {
                println!("❌ Erro na captura: {}\n", e);
                let cmd = AbecsCommand::Display::new("032  SENHA INVALIDA ");
                pinpad.execute_typed(&cmd)?;
                std::thread::sleep(std::time::Duration::from_secs(2));
                let cmd = AbecsCommand::Close::new();
                pinpad.execute_typed(&cmd)?;
                return Ok(());
            }
        }
    } else {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("ETAPA 4: PIN não necessário (crédito à vista)");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }

    // ═══════════════════════════════════════════════════════════
    // ETAPA 5: Processar transação (simulado)
    // ═══════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 5: Processamento");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cmd = AbecsCommand::Display::new("032  PROCESSANDO... ");
    pinpad.execute_typed(&cmd)?;

    println!("⏳ Enviando para processadora...");
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("⏳ Aguardando autorização...");
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Simular resposta da processadora
    let aprovado = true; // Em produção: depende da resposta real
    let codigo_autorizacao = "123456";

    // ═══════════════════════════════════════════════════════════
    // ETAPA 6: Exibir resultado
    // ═══════════════════════════════════════════════════════════
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("ETAPA 6: Resultado");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    if aprovado {
        println!("✅ TRANSAÇÃO APROVADA!");
        println!("   Código: {}", codigo_autorizacao);
        println!("   Forma: {}", forma_pagamento);
        println!("   Valor: R$ {},{:02}\n", reais, centavos);

        let cmd = AbecsCommand::Display::new("032   APROVADO!     ");
        pinpad.execute_typed(&cmd)?;
        std::thread::sleep(std::time::Duration::from_secs(2));

        let msg = format!("032  CODIGO: {}  ", codigo_autorizacao);
        let cmd = AbecsCommand::Display::new(&msg);
        pinpad.execute_typed(&cmd)?;
        std::thread::sleep(std::time::Duration::from_secs(2));
    } else {
        println!("❌ TRANSAÇÃO NEGADA\n");

        let cmd = AbecsCommand::Display::new("032   NEGADO!       ");
        pinpad.execute_typed(&cmd)?;
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    // ═══════════════════════════════════════════════════════════
    // ETAPA 7: Finalizar
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
    println!("  ✅ Transação finalizada!");
    println!("═══════════════════════════════════════════════════════");

    println!("\n📝 Resumo da transação:");
    println!(
        "   Status: {}",
        if aprovado { "APROVADA" } else { "NEGADA" }
    );
    println!("   Forma: {}", forma_pagamento);
    println!("   Valor: R$ {},{:02}", reais, centavos);
    if aprovado {
        println!("   Código: {}", codigo_autorizacao);
    }
    println!();

    println!("💡 Em produção, você deve:");
    println!("   • Integrar com a processadora real");
    println!("   • Implementar tratamento de erros robusto");
    println!("   • Armazenar logs da transação");
    println!("   • Emitir comprovantes");
    println!("   • Implementar estornos e cancelamentos\n");

    Ok(())
}
