/// Protocolo de baixo nível ABECS
/// Implementa CRC, codificação/decodificação de pacotes

// Bytes especiais do protocolo ABECS (Seção 2.2)
pub const EOT: u8 = 0x04; // End of Transmission
pub const ACK: u8 = 0x06; // Acknowledge
pub const DC3: u8 = 0x13; // Device Control 3 (byte de escape)
pub const NAK: u8 = 0x15; // Negative Acknowledge
pub const SYN: u8 = 0x16; // Synchronous Idle (início do pacote)
pub const ETB: u8 = 0x17; // End of Transmission Block (fim do pacote)
pub const CAN: u8 = 0x18; // Cancel

/// Calcula CRC-16 conforme especificação ABECS (polinômio 0x1021)
pub fn calculate_crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0;

    for &byte in data {
        crc ^= (byte as u16) << 8;
        for _ in 0..8 {
            if (crc & 0x8000) != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }

    crc
}

/// Codifica os dados aplicando substituição de bytes especiais (Seção 2.2.1)
///
/// Substitui:
/// - 0x13 (DC3) → 0x13 0x33
/// - 0x16 (SYN) → 0x13 0x36
/// - 0x17 (ETB) → 0x13 0x37
pub fn encode_data(data: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(data.len());

    for &byte in data {
        match byte {
            DC3 => {
                encoded.push(DC3);
                encoded.push(0x33);
            }
            SYN => {
                encoded.push(DC3);
                encoded.push(0x36);
            }
            ETB => {
                encoded.push(DC3);
                encoded.push(0x37);
            }
            _ => encoded.push(byte),
        }
    }

    encoded
}

/// Decodifica os dados revertendo a substituição de bytes especiais
pub fn decode_data(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut decoded = Vec::with_capacity(data.len());
    let mut i = 0;

    while i < data.len() {
        if data[i] == DC3 {
            if i + 1 >= data.len() {
                return Err("Dados incompletos após DC3".to_string());
            }
            match data[i + 1] {
                0x33 => decoded.push(DC3),
                0x36 => decoded.push(SYN),
                0x37 => decoded.push(ETB),
                b => return Err(format!("Byte inválido após DC3: 0x{:02X}", b)),
            }
            i += 2;
        } else {
            decoded.push(data[i]);
            i += 1;
        }
    }

    Ok(decoded)
}

/// Monta um pacote ABECS completo
///
/// Formato: SYN + DADOS_CODIFICADOS + ETB + CRC(2 bytes)
pub fn build_packet(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() > 2049 {
        return Err(format!(
            "Dados muito grandes: {} bytes (máximo 2049)",
            data.len()
        ));
    }

    // Calcula CRC dos dados originais + ETB
    let mut crc_data = data.to_vec();
    crc_data.push(ETB);
    let crc = calculate_crc16(&crc_data);

    // Codifica os dados aplicando substituições
    let encoded_data = encode_data(data);

    // Monta o pacote completo
    let mut packet = Vec::with_capacity(encoded_data.len() + 4);
    packet.push(SYN); // PKTSTART
    packet.extend(encoded_data); // PKTDATA (codificado)
    packet.push(ETB); // PKTSTOP
    packet.push((crc >> 8) as u8); // CRC high byte
    packet.push((crc & 0xFF) as u8); // CRC low byte

    Ok(packet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc16() {
        let data = b"OPN";
        let crc = calculate_crc16(data);
        assert_ne!(crc, 0);
    }

    #[test]
    fn test_encode_decode() {
        let data = vec![DC3, SYN, ETB, 0x41];
        let encoded = encode_data(&data);
        let decoded = decode_data(&encoded).unwrap();
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_build_packet() {
        let data = b"OPN";
        let packet = build_packet(data).unwrap();
        assert_eq!(packet[0], SYN);
        assert_eq!(packet[packet.len() - 3], ETB);
    }
}
