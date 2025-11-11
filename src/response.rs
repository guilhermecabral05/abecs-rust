/// Estrutura para representar uma resposta ABECS desserializada

/// Resposta recebida do Pinpad
///
/// # Exemplo
/// ```ignore
/// let response = pinpad.execute(&cmd)?;
///
/// if response.is_success() {
///     println!("Comando executado com sucesso!");
/// }
///
/// for i in 0..response.block_count() {
///     println!("Bloco {}: {}", i, response.get_string(i).unwrap());
/// }
/// ```
#[derive(Debug, Clone)]
pub struct AbecsResponse {
    cmd_id: String,
    status: String,
    blocks: Vec<Vec<u8>>,
}

impl AbecsResponse {
    /// Desserializa uma resposta ABECS
    pub(crate) fn deserialize(data: &[u8]) -> Result<Self, String> {
        if data.len() < 6 {
            return Err(format!(
                "Resposta muito curta: {} bytes (mínimo 6)",
                data.len()
            ));
        }

        // CMD_ID (3 bytes ASCII)
        let cmd_id = String::from_utf8_lossy(&data[0..3]).to_string();

        // STATUS (3 bytes ASCII - sempre presente após CMD_ID)
        let status = String::from_utf8_lossy(&data[3..6]).to_string();

        // Parse dos blocos de dados
        let mut blocks = Vec::new();
        let mut pos = 6;

        while pos < data.len() {
            // Verifica se há espaço para o tamanho do bloco (3 bytes)
            if pos + 3 > data.len() {
                break;
            }

            // Lê o tamanho do bloco (3 dígitos ASCII)
            let len_str = String::from_utf8_lossy(&data[pos..pos + 3]);
            let block_len = match len_str.parse::<usize>() {
                Ok(len) => len,
                Err(_) => break,
            };

            pos += 3;

            // Verifica se há dados suficientes
            if pos + block_len > data.len() {
                return Err(format!(
                    "Bloco incompleto: esperado {} bytes, disponível {}",
                    block_len,
                    data.len() - pos
                ));
            }

            // Extrai o bloco
            let block = data[pos..pos + block_len].to_vec();
            blocks.push(block);
            pos += block_len;
        }

        Ok(AbecsResponse {
            cmd_id,
            status,
            blocks,
        })
    }

    /// Retorna o ID do comando
    pub fn cmd_id(&self) -> &str {
        &self.cmd_id
    }

    /// Retorna o código de status
    pub fn status(&self) -> &str {
        &self.status
    }

    /// Retorna true se o status indica sucesso (000)
    pub fn is_success(&self) -> bool {
        self.status == "000"
    }

    /// Retorna uma descrição do status
    pub fn status_description(&self) -> &str {
        match self.status.as_str() {
            "000" => "Sucesso",
            "001" => "Erro de execução",
            "002" => "Comando inválido",
            "003" => "Parâmetro inválido",
            "004" => "Timeout",
            "005" => "Cancelado pelo usuário",
            "006" => "Cartão não inserido",
            "007" => "Erro na leitura do cartão",
            "008" => "Erro na comunicação",
            "009" => "Criptografia não suportada",
            "010" => "Chave não carregada",
            "011" => "Cartão bloqueado",
            "012" => "Comando não disponível",
            _ => "Status desconhecido",
        }
    }

    /// Retorna o número de blocos
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Retorna um bloco como bytes brutos
    pub fn get_block(&self, index: usize) -> Option<&[u8]> {
        self.blocks.get(index).map(|v| v.as_slice())
    }

    /// Converte um bloco para String (UTF-8)
    pub fn get_string(&self, index: usize) -> Option<String> {
        self.blocks
            .get(index)
            .map(|b| String::from_utf8_lossy(b).to_string())
    }

    /// Converte um bloco para hexadecimal
    pub fn get_hex(&self, index: usize) -> Option<String> {
        self.blocks.get(index).map(|b| {
            b.iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<Vec<_>>()
                .join(" ")
        })
    }

    /// Retorna todos os blocos como strings
    pub fn get_all_strings(&self) -> Vec<String> {
        self.blocks
            .iter()
            .map(|b| String::from_utf8_lossy(b).to_string())
            .collect()
    }

    /// Exibe a resposta formatada no console
    pub fn print(&self) {
        println!("\n┌─────────────────────────────────────────────────────┐");
        println!("│ Resposta ABECS                                      │");
        println!("├─────────────────────────────────────────────────────┤");
        println!("│ Comando: {:43} │", self.cmd_id);
        println!(
            "│ Status:  {} ({:38}) │",
            self.status,
            self.status_description()
        );
        println!("│ Blocos:  {:43} │", self.block_count());
        println!("└─────────────────────────────────────────────────────┘");

        for (i, block) in self.blocks.iter().enumerate() {
            println!("\n  Bloco {} ({} bytes):", i + 1, block.len());

            // Tenta exibir como texto
            let as_text = String::from_utf8_lossy(block);
            if as_text
                .chars()
                .all(|c| c.is_ascii() && !c.is_control() || c == ' ')
            {
                println!("    Texto: \"{}\"", as_text.trim());
            }

            // Exibe em hex (primeiros 32 bytes)
            if let Some(hex) = self.get_hex(i) {
                let hex_preview = if hex.len() > 96 {
                    format!("{}...", &hex[..96])
                } else {
                    hex
                };
                println!("    Hex:   {}", hex_preview);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_deserialization() {
        let data = b"OPN000";
        let response = AbecsResponse::deserialize(data).unwrap();
        assert_eq!(response.cmd_id(), "OPN");
        assert_eq!(response.status(), "000");
        assert!(response.is_success());
    }

    #[test]
    fn test_response_with_blocks() {
        let data = b"GIN000005Hello";
        let response = AbecsResponse::deserialize(data).unwrap();
        assert_eq!(response.block_count(), 1);
        assert_eq!(response.get_string(0).unwrap(), "Hello");
    }

    #[test]
    fn test_status_description() {
        let data = b"OPN000";
        let response = AbecsResponse::deserialize(data).unwrap();
        assert_eq!(response.status_description(), "Sucesso");
    }
}
