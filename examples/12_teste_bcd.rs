/// Exemplo 12: Teste de conversão BCD e parse de Track1
///
/// Este exemplo testa a conversão de BCD para String
/// e o parse estruturado da Track 1

use pinpad::{GetTracksResponse, Track1Data};

fn main() {
    println!("═══════════════════════════════════════════");
    println!("  Teste de Conversão BCD e Track1");
    println!("═══════════════════════════════════════════\n");

    // Dados de exemplo do GTK real
    let pan_bytes = vec![0x63, 0x96, 0x64, 0x99, 0x00, 0x13, 0x80, 0x69];
    let track1_bytes = vec![
        0x63, 0x96, 0x64, 0x99, 0x00, 0x13, 0x80, 0x69,
        0xD3, 0x20, 0x32, 0x06, 0x00, 0x00, 0x00, 0x72,
        0x50, 0x32, 0x5F
    ];

    println!("📦 Dados BCD originais:");
    println!("   PAN bytes: {:02X?}", pan_bytes);
    println!("   Track1 bytes: {:02X?}\n", track1_bytes);

    // Cria resposta de teste
    let response = GetTracksResponse {
        pan: Some(pan_bytes),
        track1: Some(track1_bytes),
        track2: None,
        track3: None,
        track1_ksn: None,
        track2_ksn: None,
        track3_ksn: None,
        pan_ksn: None,
        krand_enc: None,
    };

    // Testa conversão BCD
    println!("🔄 Conversão BCD → String:");
    if let Some(pan_str) = response.pan_as_string() {
        println!("   ✅ PAN: {}", pan_str);
    }

    if let Some(t1_str) = response.track1_as_string() {
        println!("   ✅ Track1: {}\n", t1_str);
        
        // Parse estruturado da Track1
        println!("🎫 Parse estruturado da Track1:");
        let track1_data = Track1Data::parse(&t1_str);
        
        println!("   {}", track1_data);
        println!();
        
        if let Some(pan) = &track1_data.pan {
            println!("   📇 PAN: {}", pan);
        }
        
        if let Some(name) = &track1_data.cardholder_name {
            println!("   👤 Nome: {}", name);
        }
        
        if let Some(exp) = &track1_data.expiry_date {
            println!("   📅 Validade (raw): {}", exp);
            if let Some(formatted) = track1_data.expiry_date_formatted() {
                println!("   📅 Validade (formatada): {}", formatted);
            }
        }
        
        if let Some(sc) = &track1_data.service_code {
            println!("   🔧 Código de Serviço: {}", sc);
        }
        
        if let Some(dd) = &track1_data.discretionary_data {
            println!("   📋 Dados Discricionários: {}", dd);
        }
        
        println!();
        
        // Verifica validade
        let is_expired = track1_data.is_expired(2025, 11);
        if is_expired {
            println!("   ❌ Cartão EXPIRADO!");
        } else {
            println!("   ✅ Cartão VÁLIDO");
        }
    }
    
    println!();
    
    // Testa também o método parse_track1() direto
    println!("🚀 Teste do método parse_track1():");
    if let Some(track1) = response.parse_track1() {
        println!("   PAN: {:?}", track1.pan);
        println!("   Validade: {:?}", track1.expiry_date_formatted());
    }

    println!("\n═══════════════════════════════════════════");
}
