/// Módulo para parsing e manipulação de dados EMV (TLV)
/// Implementa o formato Tag-Length-Value conforme ISO/IEC 7816-4
use std::collections::HashMap;

/// Representa um objeto EMV em formato TLV
#[derive(Debug, Clone, PartialEq)]
pub struct EmvTag {
    pub tag: Vec<u8>,
    pub value: Vec<u8>,
}

/// Parser de dados EMV TLV
#[derive(Debug, Clone)]
pub struct EmvData {
    tags: HashMap<Vec<u8>, Vec<u8>>,
}

impl EmvData {
    /// Cria um novo EmvData vazio
    pub fn new() -> Self {
        Self {
            tags: HashMap::new(),
        }
    }

    /// Parse bytes TLV em um EmvData
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        let mut emv_data = Self::new();
        let mut pos = 0;

        while pos < data.len() {
            // Parse tag
            let tag_start = pos;
            if pos >= data.len() {
                break;
            }

            // Tags EMV podem ter 1, 2 ou mais bytes
            let first_byte = data[pos];
            pos += 1;

            // Se os 5 bits inferiores do primeiro byte são 11111, a tag continua no próximo byte
            let tag_continues = (first_byte & 0x1F) == 0x1F;

            if tag_continues && pos < data.len() {
                // Tag de 2 bytes
                pos += 1;
                // Nota: tags de 3+ bytes são raras, não implementadas aqui
            }

            let tag = data[tag_start..pos].to_vec();

            // Parse length
            if pos >= data.len() {
                return Err(format!(
                    "Dados TLV incompletos: faltando length após tag {:02X?}",
                    tag
                ));
            }

            let length_byte = data[pos];
            pos += 1;

            let length = if length_byte & 0x80 == 0 {
                // Formato curto (1 byte)
                length_byte as usize
            } else {
                // Formato longo
                let num_bytes = (length_byte & 0x7F) as usize;
                if num_bytes == 0 || num_bytes > 4 {
                    return Err(format!("Length inválido: {}", num_bytes));
                }

                if pos + num_bytes > data.len() {
                    return Err("Dados TLV incompletos: length mal formado".to_string());
                }

                let mut len = 0usize;
                for i in 0..num_bytes {
                    len = (len << 8) | (data[pos + i] as usize);
                }
                pos += num_bytes;
                len
            };

            // Parse value
            if pos + length > data.len() {
                return Err(format!(
                    "Dados TLV incompletos: esperado {} bytes de valor, disponível {}",
                    length,
                    data.len() - pos
                ));
            }

            let value = data[pos..pos + length].to_vec();
            pos += length;

            emv_data.tags.insert(tag, value);
        }

        Ok(emv_data)
    }

    /// Serializa EmvData em bytes TLV
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        for (tag, value) in &self.tags {
            // Tag
            result.extend_from_slice(tag);

            // Length
            let len = value.len();
            if len <= 127 {
                // Formato curto
                result.push(len as u8);
            } else {
                // Formato longo (suporta até 65535 bytes - 2 bytes de length)
                if len <= 0xFF {
                    result.push(0x81);
                    result.push(len as u8);
                } else if len <= 0xFFFF {
                    result.push(0x82);
                    result.push((len >> 8) as u8);
                    result.push((len & 0xFF) as u8);
                } else {
                    // Mais de 65KB - raramente usado
                    result.push(0x83);
                    result.push((len >> 16) as u8);
                    result.push((len >> 8) as u8);
                    result.push((len & 0xFF) as u8);
                }
            }

            // Value
            result.extend_from_slice(value);
        }

        result
    }

    /// Adiciona uma tag
    pub fn add_tag(&mut self, tag: &[u8], value: &[u8]) {
        self.tags.insert(tag.to_vec(), value.to_vec());
    }

    /// Obtém o valor de uma tag
    pub fn get_tag(&self, tag: &[u8]) -> Option<&[u8]> {
        self.tags.get(tag).map(|v| v.as_slice())
    }

    /// Remove uma tag
    pub fn remove_tag(&mut self, tag: &[u8]) -> Option<Vec<u8>> {
        self.tags.remove(tag)
    }

    /// Retorna todas as tags
    pub fn tags(&self) -> &HashMap<Vec<u8>, Vec<u8>> {
        &self.tags
    }

    /// Verifica se uma tag existe
    pub fn has_tag(&self, tag: &[u8]) -> bool {
        self.tags.contains_key(tag)
    }
}

/// Tags EMV comuns (para referência)
#[allow(dead_code)]
pub mod tags {
    // Application Data
    pub const APPLICATION_IDENTIFIER: &[u8] = &[0x4F]; // AID
    pub const APPLICATION_LABEL: &[u8] = &[0x50];
    pub const APPLICATION_PRIORITY_INDICATOR: &[u8] = &[0x87];
    pub const APPLICATION_PREFERRED_NAME: &[u8] = &[0x9F, 0x12];

    // Card Data
    pub const PAN: &[u8] = &[0x5A];
    pub const TRACK_2_EQUIVALENT_DATA: &[u8] = &[0x57];
    pub const APPLICATION_EXPIRATION_DATE: &[u8] = &[0x5F, 0x24];
    pub const APPLICATION_PAN_SEQUENCE_NUMBER: &[u8] = &[0x5F, 0x34];
    pub const CARDHOLDER_NAME: &[u8] = &[0x5F, 0x20];

    // Transaction Data
    pub const AMOUNT_AUTHORIZED: &[u8] = &[0x9F, 0x02];
    pub const AMOUNT_OTHER: &[u8] = &[0x9F, 0x03];
    pub const TRANSACTION_DATE: &[u8] = &[0x9A];
    pub const TRANSACTION_TYPE: &[u8] = &[0x9C];
    pub const TRANSACTION_CURRENCY_CODE: &[u8] = &[0x5F, 0x2A];

    // Cryptographic Data
    pub const APPLICATION_CRYPTOGRAM: &[u8] = &[0x9F, 0x26];
    pub const CRYPTOGRAM_INFORMATION_DATA: &[u8] = &[0x9F, 0x27];
    pub const APPLICATION_TRANSACTION_COUNTER: &[u8] = &[0x9F, 0x36];
    pub const UNPREDICTABLE_NUMBER: &[u8] = &[0x9F, 0x37];
    pub const ISSUER_APPLICATION_DATA: &[u8] = &[0x9F, 0x10];

    // Terminal Data
    pub const TERMINAL_VERIFICATION_RESULTS: &[u8] = &[0x95];
    pub const TERMINAL_COUNTRY_CODE: &[u8] = &[0x9F, 0x1A];
    pub const TERMINAL_TYPE: &[u8] = &[0x9F, 0x35];
    pub const TERMINAL_CAPABILITIES: &[u8] = &[0x9F, 0x33];
    pub const ADDITIONAL_TERMINAL_CAPABILITIES: &[u8] = &[0x9F, 0x40];

    // Issuer Data
    pub const ISSUER_COUNTRY_CODE: &[u8] = &[0x5F, 0x28];
    pub const ISSUER_SCRIPT_RESULTS: &[u8] = &[0xDF, 0x31];

    // Processing Data
    pub const AUTHORIZATION_RESPONSE_CODE: &[u8] = &[0x8A];
    pub const CVM_RESULTS: &[u8] = &[0x9F, 0x34];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_tlv() {
        // Tag 9F26 (Application Cryptogram) com 8 bytes de valor
        let data = vec![
            0x9F, 0x26, 0x08, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        ];

        let emv = EmvData::parse(&data).unwrap();
        let value = emv.get_tag(&[0x9F, 0x26]).unwrap();
        assert_eq!(value, &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    }

    #[test]
    fn test_serialize_tlv() {
        let mut emv = EmvData::new();
        emv.add_tag(&[0x9F, 0x26], &[0x01, 0x02, 0x03, 0x04]);
        emv.add_tag(&[0x5A], &[0x12, 0x34, 0x56]);

        let serialized = emv.serialize();
        let parsed = EmvData::parse(&serialized).unwrap();

        assert_eq!(
            parsed.get_tag(&[0x9F, 0x26]).unwrap(),
            &[0x01, 0x02, 0x03, 0x04]
        );
        assert_eq!(parsed.get_tag(&[0x5A]).unwrap(), &[0x12, 0x34, 0x56]);
    }

    #[test]
    fn test_parse_multiple_tags() {
        let data = vec![
            0x5A, 0x03, 0x11, 0x22, 0x33, // PAN
            0x9A, 0x03, 0x25, 0x11, 0x11, // Transaction Date
            0x9F, 0x02, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // Amount
        ];

        let emv = EmvData::parse(&data).unwrap();
        assert_eq!(emv.get_tag(&[0x5A]).unwrap(), &[0x11, 0x22, 0x33]);
        assert_eq!(emv.get_tag(&[0x9A]).unwrap(), &[0x25, 0x11, 0x11]);
        assert_eq!(
            emv.get_tag(&[0x9F, 0x02]).unwrap(),
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x01]
        );
    }
}
