/// Estrutura para representar um comando ABECS

/// Comando ABECS a ser enviado ao Pinpad
///
/// # Exemplo
/// ```
/// use pinpad::AbecsCommand;
///
/// // Comando simples (sem parâmetros)
/// let cmd = AbecsCommand::new("OPN");
///
/// // Comando com parâmetros
/// let mut cmd = AbecsCommand::new("DSP");
/// cmd.add_block(b"032          BEM-VINDO       ".to_vec());
/// ```
#[derive(Debug, Clone)]
pub struct AbecsCommand {
    pub(crate) cmd_id: String,
    pub(crate) blocks: Vec<Vec<u8>>,
}

impl AbecsCommand {
    /// Cria um novo comando ABECS
    ///
    /// # Argumentos
    /// * `cmd_id` - Identificador do comando (3 caracteres, ex: "OPN", "DSP", "GIN")
    pub fn new(cmd_id: &str) -> Self {
        AbecsCommand {
            cmd_id: cmd_id.to_string(),
            blocks: Vec::new(),
        }
    }

    /// Adiciona um bloco de dados ao comando
    ///
    /// # Argumentos
    /// * `data` - Dados do bloco (até 999 bytes)
    pub fn add_block(&mut self, data: Vec<u8>) -> &mut Self {
        self.blocks.push(data);
        self
    }

    /// Adiciona um bloco de dados a partir de uma string
    pub fn add_string(&mut self, text: &str) -> &mut Self {
        self.blocks.push(text.as_bytes().to_vec());
        self
    }

    /// Serializa o comando no formato ABECS
    ///
    /// Formato: CMD_ID(3) + [LEN(3) + DATA]...
    pub(crate) fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();

        // CMD_ID (3 bytes ASCII)
        data.extend(self.cmd_id.as_bytes());

        // Cada bloco é precedido por seu tamanho (3 dígitos ASCII)
        for block in &self.blocks {
            let len_str = format!("{:03}", block.len());
            data.extend(len_str.as_bytes());
            data.extend(block);
        }

        data
    }

    /// Retorna o ID do comando
    pub fn id(&self) -> &str {
        &self.cmd_id
    }

    /// Retorna o número de blocos
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }
}

// Construtores convenientes para comandos comuns
impl AbecsCommand {
    /// Cria comando OPN (Open - Abertura de sessão)
    pub fn open() -> Self {
        Self::new("OPN")
    }

    /// Cria comando CLO (Close - Fechamento de sessão)
    pub fn close() -> Self {
        Self::new("CLO")
    }

    /// Cria comando DSP (Display - Mostrar mensagem)
    ///
    /// # Argumentos
    /// * `message` - Mensagem a ser exibida (formato: "LLL" + texto, onde LLL é o tempo em segundos)
    pub fn display(message: &str) -> Self {
        let mut cmd = Self::new("DSP");
        cmd.add_string(message);
        cmd
    }

    /// Cria comando GIN (Get Info - Obter informações)
    ///
    /// # Argumentos
    /// * `info_type` - Tipo de informação a obter (ex: "01" para info geral)
    pub fn get_info(info_type: &str) -> Self {
        let mut cmd = Self::new("GIN");
        cmd.add_string(info_type);
        cmd
    }

    /// Cria comando CLX (Clear Display)
    pub fn clear_display() -> Self {
        Self::new("CLX")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_creation() {
        let cmd = AbecsCommand::new("OPN");
        assert_eq!(cmd.id(), "OPN");
        assert_eq!(cmd.block_count(), 0);
    }

    #[test]
    fn test_command_with_blocks() {
        let mut cmd = AbecsCommand::new("DSP");
        cmd.add_string("Hello");
        assert_eq!(cmd.block_count(), 1);
    }

    #[test]
    fn test_command_serialization() {
        let mut cmd = AbecsCommand::new("DSP");
        cmd.add_string("Hi");
        let data = cmd.serialize();
        assert_eq!(&data[0..3], b"DSP");
        assert_eq!(&data[3..6], b"002");
        assert_eq!(&data[6..8], b"Hi");
    }

    #[test]
    fn test_convenience_constructors() {
        let cmd = AbecsCommand::open();
        assert_eq!(cmd.id(), "OPN");

        let cmd = AbecsCommand::display("032Test");
        assert_eq!(cmd.id(), "DSP");
        assert_eq!(cmd.block_count(), 1);
    }
}
