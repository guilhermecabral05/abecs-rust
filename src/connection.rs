/// Gerenciamento da conexão serial com o Pinpad
use serialport::{SerialPort, SerialPortType};
use std::io::{Read, Write};
use std::time::Duration;

use crate::command::AbecsCommand;
use crate::error::AbecsError;
use crate::protocol::*;
use crate::response::AbecsResponse;
use crate::serialize::{AbecsDeserialize, AbecsTypedCommand};
use crate::Result;

/// Conexão com o Pinpad via porta serial
///
/// # Exemplo
/// ```ignore
/// use pinpad::PinpadConnection;
///
/// let mut pinpad = PinpadConnection::open("/dev/ttyACM0")?;
/// let response = pinpad.execute(&AbecsCommand::open())?;
/// ```
pub struct PinpadConnection {
    port: Box<dyn SerialPort>,
    verbose: bool,
}
impl PinpadConnection {
    /// Abre uma conexão com o Pinpad
    ///
    /// # Argumentos
    /// * `port_name` - Nome da porta serial (ex: "/dev/ttyACM0" no Linux, "COM3" no Windows)
    ///
    /// # Configuração
    /// - Baud rate: 19200 bps
    /// - Data bits: 8
    /// - Parity: None
    /// - Stop bits: 1
    pub fn open(port_name: &str) -> Result<Self> {
        let port = serialport::new(port_name, 19_200)
            .timeout(Duration::from_millis(2000))
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .open()
            .map_err(|e| {
                AbecsError::SerialError(format!("Erro ao abrir porta {}: {}", port_name, e))
            })?;

        Ok(PinpadConnection {
            port,
            verbose: false,
        })
    }

    /// Lista as portas seriais disponíveis
    pub fn list_ports() -> Result<Vec<String>> {
        let ports = serialport::available_ports()
            .map_err(|e| AbecsError::SerialError(format!("Erro ao listar portas: {}", e)))?;

        Ok(ports
            .iter()
            .map(|p| {
                let extra = match &p.port_type {
                    SerialPortType::UsbPort(info) => {
                        format!(" [USB: {:04x}:{:04x}]", info.vid, info.pid)
                    }
                    _ => String::new(),
                };
                format!("{}{}", p.port_name, extra)
            })
            .collect())
    }

    /// Ativa/desativa modo verbose (debug)
    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    /// Cancela qualquer comando em execução
    pub fn cancel(&mut self) -> Result<()> {
        if self.verbose {
            println!("→ Enviando CAN...");
        }

        for attempt in 1..=3 {
            self.port
                .write_all(&[CAN])
                .map_err(|e| AbecsError::SerialError(format!("Erro ao enviar CAN: {}", e)))?;

            let mut buffer = [0u8; 1];
            match self.port.read(&mut buffer) {
                Ok(_) if buffer[0] == EOT => {
                    if self.verbose {
                        println!("← Recebido EOT");
                    }
                    return Ok(());
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    if attempt == 3 {
                        return Err(AbecsError::Timeout("Não recebeu EOT".to_string()));
                    }
                }
                _ => continue,
            }
        }

        Err(AbecsError::Timeout("Falha ao cancelar comando".to_string()))
    }

    /// Executa um comando e retorna a resposta
    ///
    /// # Argumentos
    /// * `command` - Comando a ser executado
    ///
    /// # Exemplo
    /// ```ignore
    /// let response = pinpad.execute(&AbecsCommand::open())?;
    /// if response.is_success() {
    ///     println!("Sucesso!");
    /// }
    /// ```
    pub fn execute(&mut self, command: &AbecsCommand) -> Result<AbecsResponse> {
        self.send_command(command)?;
        let raw_response = self.receive_response(false)?;
        AbecsResponse::deserialize(&raw_response).map_err(|e| AbecsError::InvalidResponse(e))
    }

    /// Executa um comando blocante (que requer interação do usuário)
    ///
    /// Comandos blocantes têm timeout de 5 minutos ao invés de 10 segundos.
    pub fn execute_blocking(&mut self, command: &AbecsCommand) -> Result<AbecsResponse> {
        self.send_command(command)?;
        let raw_response = self.receive_response(true)?;
        AbecsResponse::deserialize(&raw_response).map_err(|e| AbecsError::InvalidResponse(e))
    }

    /// Executa um comando tipado e retorna uma resposta tipada
    ///
    /// # Argumentos
    /// * `command` - Comando tipado a ser executado
    ///
    /// # Exemplo
    /// ```ignore
    /// use pinpad::{PinpadConnection, OpenCommand};
    ///
    /// let mut pinpad = PinpadConnection::open("/dev/ttyACM0")?;
    /// let command = OpenCommand;
    /// let response = pinpad.execute_typed(&command)?;
    /// ```
    pub fn execute_typed<C: AbecsTypedCommand>(&mut self, command: &C) -> Result<C::Response> {
        // Cria o comando ABECS
        let mut abecs_cmd = AbecsCommand::new(command.command_id());

        // Adiciona os parâmetros
        for param in command.serialize_params() {
            abecs_cmd.add_block(param);
        }

        // Executa o comando (blocante ou não)
        let raw_response = if command.is_blocking() {
            self.execute_blocking(&abecs_cmd)?
        } else {
            self.execute(&abecs_cmd)?
        };

        // Desserializa a resposta tipada
        C::Response::deserialize_abecs(&raw_response).map_err(|e| AbecsError::InvalidResponse(e))
    }

    /// Envia um comando para o Pinpad (interno)
    fn send_command(&mut self, command: &AbecsCommand) -> Result<()> {
        let cmd_data = command.serialize();
        let packet = build_packet(&cmd_data).map_err(|e| AbecsError::ProtocolError(e))?;

        if self.verbose {
            println!("\n→ Enviando: {} ({} bytes)", command.id(), cmd_data.len());
            self.print_hex("  ", &packet);
        }

        // Tenta enviar até 3 vezes
        for attempt in 1..=3 {
            self.port
                .write_all(&packet)
                .map_err(|e| AbecsError::SerialError(format!("Erro ao enviar: {}", e)))?;
            self.port
                .flush()
                .map_err(|e| AbecsError::SerialError(format!("Erro ao flush: {}", e)))?;

            let mut buffer = [0u8; 1];
            match self.port.read(&mut buffer) {
                Ok(_) => match buffer[0] {
                    ACK => {
                        if self.verbose {
                            println!("← ACK");
                        }
                        return Ok(());
                    }
                    NAK => {
                        if self.verbose {
                            println!("← NAK (tentativa {})", attempt);
                        }
                        if attempt == 3 {
                            return Err(AbecsError::NakReceived("Recebeu NAK 3 vezes".to_string()));
                        }
                    }
                    b => {
                        return Err(AbecsError::ProtocolError(format!(
                            "Byte inesperado: 0x{:02X}",
                            b
                        )));
                    }
                },
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    return Err(AbecsError::Timeout(format!(
                        "Timeout aguardando ACK/NAK (tentativa {})",
                        attempt
                    )));
                }
                Err(e) => {
                    return Err(AbecsError::SerialError(format!("Erro ao ler: {}", e)));
                }
            }
        }

        Err(AbecsError::NakReceived(
            "Falha após 3 tentativas".to_string(),
        ))
    }

    /// Recebe resposta do Pinpad (interno)
    fn receive_response(&mut self, blocking: bool) -> Result<Vec<u8>> {
        if self.verbose {
            println!("\n← Aguardando resposta...");
        }

        let timeout = if blocking {
            Duration::from_secs(300)
        } else {
            Duration::from_secs(10)
        };

        self.port
            .set_timeout(timeout)
            .map_err(|e| AbecsError::SerialError(format!("Erro ao configurar timeout: {}", e)))?;

        for attempt in 1..=3 {
            // Aguarda SYN
            let mut buffer = [0u8; 1];
            loop {
                match self.port.read(&mut buffer) {
                    Ok(_) if buffer[0] == SYN => break,
                    Ok(_) => continue,
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                        return Err(AbecsError::Timeout(
                            "Timeout aguardando resposta".to_string(),
                        ));
                    }
                    Err(e) => {
                        return Err(AbecsError::SerialError(format!("Erro ao ler: {}", e)));
                    }
                }
            }

            // Lê dados até ETB
            let mut pkt_data = Vec::new();
            loop {
                match self.port.read(&mut buffer) {
                    Ok(_) if buffer[0] == ETB => break,
                    Ok(_) => {
                        pkt_data.push(buffer[0]);
                        if pkt_data.len() > 2049 {
                            return Err(AbecsError::ProtocolError(
                                "Pacote muito grande".to_string(),
                            ));
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                        return Err(AbecsError::Timeout("Timeout lendo pacote".to_string()));
                    }
                    Err(e) => {
                        return Err(AbecsError::SerialError(format!("Erro ao ler: {}", e)));
                    }
                }
            }

            // Lê CRC
            let mut crc_bytes = [0u8; 2];
            self.port
                .read_exact(&mut crc_bytes)
                .map_err(|e| AbecsError::SerialError(format!("Erro ao ler CRC: {}", e)))?;

            let received_crc = ((crc_bytes[0] as u16) << 8) | (crc_bytes[1] as u16);

            // Decodifica e valida CRC
            let decoded_data = decode_data(&pkt_data).map_err(|e| AbecsError::ProtocolError(e))?;

            let mut crc_check = decoded_data.clone();
            crc_check.push(ETB);
            let calculated_crc = calculate_crc16(&crc_check);

            if calculated_crc == received_crc {
                if self.verbose {
                    println!("✓ CRC válido ({} bytes)", decoded_data.len());
                    self.print_hex("  ", &decoded_data);
                }

                self.port
                    .set_timeout(Duration::from_millis(2000))
                    .map_err(|e| {
                        AbecsError::SerialError(format!("Erro ao restaurar timeout: {}", e))
                    })?;

                return Ok(decoded_data);
            } else {
                if self.verbose {
                    println!("✗ CRC inválido (tentativa {})", attempt);
                }

                if attempt < 3 {
                    self.port.write_all(&[NAK]).map_err(|e| {
                        AbecsError::SerialError(format!("Erro ao enviar NAK: {}", e))
                    })?;
                } else {
                    return Err(AbecsError::ProtocolError(
                        "CRC inválido após 3 tentativas".to_string(),
                    ));
                }
            }
        }

        Err(AbecsError::ProtocolError(
            "Falha após 3 tentativas".to_string(),
        ))
    }

    /// Imprime bytes em hexadecimal (debug)
    fn print_hex(&self, prefix: &str, data: &[u8]) {
        const BYTES_PER_LINE: usize = 16;

        for (i, chunk) in data.chunks(BYTES_PER_LINE).enumerate() {
            print!("{}[{:04X}] ", prefix, i * BYTES_PER_LINE);

            for (j, &byte) in chunk.iter().enumerate() {
                print!("{:02X} ", byte);
                if j == 7 {
                    print!(" ");
                }
            }

            for _ in chunk.len()..BYTES_PER_LINE {
                print!("   ");
                if chunk.len() <= 8 {
                    print!(" ");
                }
            }

            print!(" | ");

            for &byte in chunk {
                if byte >= 0x20 && byte <= 0x7E {
                    print!("{}", byte as char);
                } else {
                    print!(".");
                }
            }

            println!();
        }
    }
}
