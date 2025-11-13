# Biblioteca Pinpad ABECS 2.12

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Biblioteca Rust para comunicação com Pinpads via Protocolo ABECS 2.12.

## ✨ Características

- ✅ **Fácil de usar** - API simples e intuitiva com comandos tipados
- ✅ **Type-safe** - API tipada com segurança em tempo de compilação
- ✅ **Protocolo completo** - Implementação conforme especificação ABECS 2.12
- ✅ **Transações EMV** - Suporte completo a chip (GOX/FCX) e contactless
- ✅ **Parsing EMV** - Parser TLV para dados EMV (tags ISO 7816)
- ✅ **Confiável** - CRC-16, retransmissão automática, validação de pacotes
- ✅ **Bem documentado** - Exemplos e documentação completa
- ✅ **Modular** - Código organizado em módulos
- ✅ **Flexível** - Suporta comandos personalizados

## 📦 Instalação

Adicione ao seu `Cargo.toml`:

```toml
[dependencies]
pinpad = { path = "../pinpad" }
```

## 🚀 Uso Rápido

### Nova API Tipada (Recomendada) ⭐

```rust
use pinpad::{AbecsCommand, PinpadConnection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Conecta ao Pinpad
    let mut pinpad = PinpadConnection::open("/dev/ttyACM1")?;
    
    // Abre uma sessão - Sintaxe clara e intuitiva!
    let cmd = AbecsCommand::Open::new();
    pinpad.execute_typed(&cmd)?;
    
    // Exibe uma mensagem
    let cmd = AbecsCommand::Display::new("BEM-VINDO!");
    pinpad.execute_typed(&cmd)?;
    
    // Obter informações
    let cmd = AbecsCommand::GetInfo::new("01");
    let response = pinpad.execute_typed(&cmd)?;
    println!("Info: {}", response.info);
    
    // Fechar sessão
    let cmd = AbecsCommand::Close::new();
    pinpad.execute_typed(&cmd)?;
    
    Ok(())
}
```

### API de Baixo Nível (Para casos avançados)

```rust
use pinpad::{RawAbecsCommand, PinpadConnection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pinpad = PinpadConnection::open("/dev/ttyACM1")?;
    
    // Construir comando manualmente
    let mut cmd = RawAbecsCommand::new("DSP");
    cmd.add_block(b"032Olá Pinpad!".to_vec());
    
    let response = pinpad.execute(&cmd)?;
    
    if response.is_success() {
        println!("✓ Comando executado!");
    }
    
    Ok(())
}
```

## 📖 Exemplos

### 📚 Exemplos Completos

A biblioteca inclui **7 exemplos completos e documentados** na pasta `examples/`:

```bash
cargo run --example 01_basico              # Uso básico da biblioteca
cargo run --example 02_informacoes         # Obter info do Pinpad
cargo run --example 03_menu                # Menu interativo
cargo run --example 04_entrada_dados       # Capturar dados
cargo run --example 05_captura_pin         # Captura segura de PIN
cargo run --example 06_comando_personalizado  # Criar seus comandos
cargo run --example 07_transacao_completa  # Fluxo completo (tarja magnética)
cargo run --example 08_transacao_emv_completa # Transação EMV com chip
```

**[📖 Ver todos os exemplos em detalhes](examples/README.md)**

### Comandos Disponíveis

A biblioteca oferece comandos tipados para maior segurança e facilidade de uso:

```rust
use pinpad::AbecsCommand;

// ═══════════════════════════════════════════════════════════
// Comandos Básicos
// ═══════════════════════════════════════════════════════════
let cmd = AbecsCommand::Open::new();         // Abrir sessão
let cmd = AbecsCommand::Close::new();        // Fechar sessão

// ═══════════════════════════════════════════════════════════
// Display
// ═══════════════════════════════════════════════════════════
let cmd = AbecsCommand::Display::new("MENSAGEM");
let cmd = AbecsCommand::ClearDisplay::new();

// ═══════════════════════════════════════════════════════════
// Informações
// ═══════════════════════════════════════════════════════════
let cmd = AbecsCommand::GetInfo::new("01");
let response = pinpad.execute_typed(&cmd)?;
println!("Info: {}", response.info);

// ═══════════════════════════════════════════════════════════
// Entrada de Dados (Blocantes) ⏱️
// ═══════════════════════════════════════════════════════════

// Capturar PIN (criptografado)
let cmd = AbecsCommand::GetPin::new(
    "DIGITE O PIN",        // mensagem
    4,                      // min length
    12,                     // max length
    30,                     // timeout (segundos)
    "01",                   // tipo de criptografia
    "1234567890123456"     // PAN do cartão
);
let response = pinpad.execute_typed(&cmd)?;
println!("PIN Block: {:02X?}", response.pin_block);

// Capturar dados (texto/números)
let cmd = AbecsCommand::GetData::new("DIGITE O VALOR", 1, 10, 60);
let response = pinpad.execute_typed(&cmd)?;
println!("Data: {}", response.data);

// Menu de seleção
let options = vec!["CREDITO".to_string(), "DEBITO".to_string()];
let cmd = AbecsCommand::Menu::new("FORMA PAGAMENTO", options, 30);
let response = pinpad.execute_typed(&cmd)?;
println!("Selecionado: {}", response.selected_index);

// ═══════════════════════════════════════════════════════════
// Transações com Cartão
// ═══════════════════════════════════════════════════════════

// Leitura de cartão (chip, tarja ou contactless)
let cmd = AbecsCommand::GetCard::new(
    25000,    // Valor em centavos (R$ 250,00)
    "251115", // Data AAMMDD
    "143000", // Hora HHMMSS
    60,       // Timeout em segundos
).with_message("INSIRA OU APROXIME");

let response = pinpad.execute_typed(&cmd)?;
println!("Tipo: {}", response.card_type); // "00"=Mag, "03"=ICC, "06"=CTLS
if let Some(pan) = response.pan {
    println!("PAN: {}", pan);
}
if let Some(emv) = response.emv_data {
    println!("Tags EMV: {}", emv.tags().len());
}

// ═══════════════════════════════════════════════════════════
// Transações EMV (Chip)
// ═══════════════════════════════════════════════════════════

// Processar chip EMV
let terminal_params = vec![0x9F, 0x33, 0x03, 0xE0, 0xF8, 0xC8];
let cmd = AbecsCommand::GoOnChip::new(
    "04",           // Tipo de aplicação (débito)
    25000,          // Valor em centavos
    "251115",       // Data
    "143000",       // Hora
    terminal_params,
).with_currency("0986"); // BRL

let response = pinpad.execute_typed(&cmd)?;
println!("GOX Result: {}", response.gox_result);
if let Some(emv) = response.emv_data {
    // Acessar cryptogram
    if let Some(cryptogram) = emv.get_tag(&[0x9F, 0x26]) {
        println!("Cryptogram: {:02X?}", cryptogram);
    }
}

// Finalizar chip EMV
let cmd = AbecsCommand::FinishChip::new("00") // ARC: "00" = aprovado
    .with_emv_data(issuer_emv_data);

let response = pinpad.execute_typed(&cmd)?;
println!("FCX Result: {}", response.fcx_result);

// ═══════════════════════════════════════════════════════════
// Tabelas
// ═══════════════════════════════════════════════════════════
let cmd = AbecsCommand::TableLoadInit::new("TAB01");
let cmd = AbecsCommand::TableLoadRecord::new(vec![0x01, 0x02]);
let cmd = AbecsCommand::TableLoadFinish::new();

// ═══════════════════════════════════════════════════════════
// Criptografia
// ═══════════════════════════════════════════════════════════
let cmd = AbecsCommand::GetKey::new(0);  // índice da chave
let response = pinpad.execute_typed(&cmd)?;
```

📚 **[Documentação completa dos comandos](TYPED_COMMANDS.md)**

### Listar Portas Disponíveis

```rust
let ports = PinpadConnection::list_ports()?;
for port in ports {
    println!("Porta: {}", port);
}
```

### Comandos Pré-definidos (API Tradicional)

```rust
// Abertura de sessão
pinpad.execute(&AbecsCommand::open())?;

// Fechamento de sessão
pinpad.execute(&AbecsCommand::close())?;

// Exibir mensagem
pinpad.execute(&AbecsCommand::display("032Mensagem"))?;

// Obter informações
let response = pinpad.execute(&AbecsCommand::get_info("01"))?;
```

### Comando Personalizado

```rust
let mut cmd = AbecsCommand::new("GIN");
cmd.add_string("01");

let response = pinpad.execute(&cmd)?;

for i in 0..response.block_count() {
    println!("Bloco {}: {}", i, response.get_string(i).unwrap());
}
```

### Modo Verbose (Debug)

```rust
let mut pinpad = PinpadConnection::open("/dev/ttyACM1")?;
pinpad.set_verbose(true); // Mostra todos os bytes trocados
```

### Lendo Dados da Resposta

```rust
let response = pinpad.execute(&cmd)?;

// Verifica sucesso
if response.is_success() {
    println!("Sucesso!");
}

// Lê blocos como texto
for i in 0..response.block_count() {
    if let Some(text) = response.get_string(i) {
        println!("Bloco {}: {}", i, text);
    }
}

// Lê blocos como hexadecimal
if let Some(hex) = response.get_hex(0) {
    println!("Hex: {}", hex);
}

// Lê blocos como bytes brutos
if let Some(bytes) = response.get_block(0) {
    println!("Bytes: {:?}", bytes);
}
```

### Tratamento de Erros

```rust
use pinpad::AbecsError;

match pinpad.execute(&cmd) {
    Ok(response) => {
        if response.is_success() {
            println!("Sucesso!");
        } else {
            println!("Erro do Pinpad: {}", response.status_description());
        }
    }
    Err(AbecsError::Timeout(msg)) => {
        println!("Timeout: {}", msg);
    }
    Err(AbecsError::NakReceived(msg)) => {
        println!("NAK recebido: {}", msg);
    }
    Err(e) => {
        println!("Erro: {}", e);
    }
}
```

### Trabalhando com Dados EMV

A biblioteca inclui um módulo completo para parsing de dados EMV (TLV):

```rust
use pinpad::EmvData;

// Parse de dados EMV recebidos do cartão
let response = pinpad.execute_typed(&get_card_cmd)?;
if let Some(emv) = response.emv_data {
    // Acessar tags específicas
    if let Some(pan) = emv.get_tag(&[0x5A]) {
        println!("PAN: {:02X?}", pan);
    }
    
    if let Some(cryptogram) = emv.get_tag(&[0x9F, 0x26]) {
        println!("Application Cryptogram: {:02X?}", cryptogram);
    }
    
    // Iterar todas as tags
    for (tag, value) in emv.tags() {
        println!("Tag {:02X?}: {:02X?}", tag, value);
    }
}

// Criar dados EMV para enviar ao Pinpad
let mut emv = EmvData::new();
emv.add_tag(&[0x8A], b"00"); // Authorization Response Code
emv.add_tag(&[0x9F, 0x02], &[0x00, 0x00, 0x00, 0x00, 0x25, 0x00]); // Amount

// Serializar para bytes TLV
let tlv_bytes = emv.serialize();

// Parse de bytes TLV
let emv = EmvData::parse(&tlv_bytes)?;
```

**Tags EMV Comuns:**
- `0x5A` - PAN (Primary Account Number)
- `0x9F26` - Application Cryptogram
- `0x9F27` - Cryptogram Information Data
- `0x9F36` - Application Transaction Counter
- `0x9F37` - Unpredictable Number
- `0x95` - Terminal Verification Results
- `0x9A` - Transaction Date
- `0x9C` - Transaction Type

Veja `src/emv.rs` para lista completa de tags e documentação.
    Err(AbecsError::NakReceived(msg)) => {
        println!("NAK recebido: {}", msg);
    }
    Err(e) => {
        println!("Erro: {}", e);
    }
}
```

## 🏗️ Estrutura do Projeto

```
src/
├── lib.rs          # Ponto de entrada da biblioteca
├── protocol.rs     # Protocolo de baixo nível (CRC, codificação)
├── connection.rs   # Gerenciamento da conexão serial
├── command.rs      # Estrutura de comandos ABECS
├── response.rs     # Estrutura de respostas ABECS
├── error.rs        # Tipos de erro
└── main.rs         # Exemplo de uso
```

## 📚 API Principal

### `PinpadConnection`

```rust
// Abre conexão
PinpadConnection::open(port_name: &str) -> Result<Self>

// Lista portas
PinpadConnection::list_ports() -> Result<Vec<String>>

// Executa comando
pinpad.execute(&command) -> Result<AbecsResponse>

// Executa comando blocante (com timeout longo)
pinpad.execute_blocking(&command) -> Result<AbecsResponse>

// Cancela comando em execução
pinpad.cancel() -> Result<()>

// Ativa/desativa debug
pinpad.set_verbose(bool)
```

### `AbecsCommand`

```rust
// Cria comando
AbecsCommand::new(cmd_id: &str) -> Self

// Adiciona bloco de dados
cmd.add_block(data: Vec<u8>) -> &mut Self
cmd.add_string(text: &str) -> &mut Self

// Comandos pré-definidos
AbecsCommand::open() -> Self
AbecsCommand::close() -> Self
AbecsCommand::display(message: &str) -> Self
AbecsCommand::get_info(info_type: &str) -> Self
AbecsCommand::clear_display() -> Self
```

### `AbecsResponse`

```rust
// Verifica sucesso
response.is_success() -> bool

// Obtém dados
response.cmd_id() -> &str
response.status() -> &str
response.status_description() -> &str
response.block_count() -> usize

// Lê blocos
response.get_block(index) -> Option<&[u8]>
response.get_string(index) -> Option<String>
response.get_hex(index) -> Option<String>
response.get_all_strings() -> Vec<String>

// Exibe formatado
response.print()
```

## 🔐 Bytes Especiais do Protocolo

| Byte | Valor | Nome | Descrição |
|------|-------|------|-----------|
| EOT  | 0x04  | End of Transmission | Resposta ao CAN |
| ACK  | 0x06  | Acknowledge | Pacote aceito |
| DC3  | 0x13  | Device Control 3 | Byte de escape |
| NAK  | 0x15  | Negative Acknowledge | Pacote rejeitado |
| SYN  | 0x16  | Synchronous Idle | Início do pacote |
| ETB  | 0x17  | End of Transmission Block | Fim do pacote |
| CAN  | 0x18  | Cancel | Cancelar comando |

## ⚙️ Configuração Serial

- **Baud rate:** 19200 bps
- **Data bits:** 8
- **Parity:** None
- **Stop bits:** 1
- **Timeout padrão:** 2 segundos
- **Timeout blocante:** 5 minutos

## 🐛 Troubleshooting

### Porta não encontrada

```bash
# Linux/Mac
ls /dev/tty*

# Adicionar permissão
sudo usermod -a -G dialout $USER
```

### NAK constante

Verifique:
1. CRC-16 (polinômio 0x1021)
2. Substituição de bytes especiais
3. Formato do comando

### Timeout

Verifique:
1. Cabo e conexão física
2. Baud rate (19200)
3. Pinpad ligado e funcionando

## 📝 Códigos de Status

| Status | Descrição |
|--------|-----------|
| 000 | Sucesso |
| 001 | Erro de execução |
| 002 | Comando inválido |
| 003 | Parâmetro inválido |
| 004 | Timeout |
| 005 | Cancelado pelo usuário |
| 006 | Cartão não inserido |
| 007 | Erro na leitura do cartão |
| 008 | Erro na comunicação |
| 009 | Criptografia não suportada |
| 010 | Chave não carregada |

## 🧪 Executando o Exemplo

```bash
# Compilar
cargo build

# Executar (pode precisar de sudo no Linux)
sudo ./target/debug/pinpad-example

# Ou diretamente
sudo cargo run --bin pinpad-example
```

## 📄 Licença

MIT License - veja o arquivo LICENSE para detalhes.

## 👨‍💻 Desenvolvido por

Implementação completa do Protocolo ABECS 2.12 (versão 11-abr-19).

## 🔗 Referências

- [Especificação ABECS 2.12](protocolo_abecs.md)
- Seção 2.2: Nível de Enlace
- Seção 2.2.1: Formato do Pacote
- Seção 2.2.2: Fluxo de Comunicação

---

**Nota:** Esta biblioteca é fornecida como implementação de referência do Protocolo ABECS 2.12. Testada e validada com Pinpad real!


Implementação completa do Protocolo ABECS versão 2.12 para comunicação com Pinpads em Rust.

## 🚀 Características

- ✅ **Protocolo completo** conforme especificação ABECS 2.12
- ✅ **CRC-16** implementado corretamente
- ✅ **Substituição de bytes especiais** (DC3, SYN, ETB)
- ✅ **Retransmissão automática** em caso de NAK (até 3 tentativas)
- ✅ **Validação de pacotes** com CRC
- ✅ **Comandos blocantes e não-blocantes**
- ✅ **Mensagens de diagnóstico** detalhadas
- ✅ **Tratamento de erros** robusto

## 📋 Requisitos

- Rust 1.70 ou superior
- Pinpad conectado via porta serial (USB, RS-232, etc.)

## 🔧 Instalação

```bash
cargo build --release
```

## 📖 Uso

### Listando Portas Disponíveis

O programa automaticamente lista as portas seriais disponíveis ao iniciar.

### Configurando a Porta Serial

Edite a variável `port_name` no arquivo `src/main.rs`:

```rust
let port_name = "/dev/ttyUSB0"; // Linux/Mac
// let port_name = "COM3"; // Windows
```

### Executando

```bash
cargo run
```

## 💡 Exemplos de Comandos

### Comando OPN (Open - Abertura de Sessão)

```rust
let cmd_opn = AbecsCommand::new("OPN");
match pinpad.execute_command(&cmd_opn, false) {
    Ok(response) => {
        println!("Status: {}", String::from_utf8_lossy(&response[3..6]));
    }
    Err(e) => println!("Erro: {}", e),
}
```

### Comando DSP (Display - Mostrar Mensagem)

```rust
let mut cmd_dsp = AbecsCommand::new("DSP");
let message = b"032          BEM-VINDO       AO PINPAD ABECS ";
cmd_dsp.add_block(message.to_vec());

match pinpad.execute_command(&cmd_dsp, false) {
    Ok(_) => println!("Mensagem exibida!"),
    Err(e) => println!("Erro: {}", e),
}
```

### Comando GIN (Get Info - Obter Informações)

```rust
let mut cmd_gin = AbecsCommand::new("GIN");
cmd_gin.add_block(b"01".to_vec()); // Solicita informações do Pinpad

match pinpad.execute_command(&cmd_gin, false) {
    Ok(response) => {
        // Parse da resposta...
    }
    Err(e) => println!("Erro: {}", e),
}
```

### Comando CEX (Captura de PIN com Criptografia)

```rust
let mut cmd_cex = AbecsCommand::new("CEX");
// Adicione os blocos de dados necessários...
cmd_cex.add_block(/* ... */);

match pinpad.execute_command(&cmd_cex, true) { // true = comando blocante
    Ok(response) => {
        // PIN capturado com sucesso
    }
    Err(e) => println!("Erro: {}", e),
}
```

## 🔍 Diagnóstico de Erros

O programa fornece diagnóstico detalhado de erros:

### NAK Recebido

Se o Pinpad responder com **NAK** (0x15):

- ❌ **CRC incorreto**: Verifique o algoritmo de CRC
- ❌ **Formato do pacote**: Verifique a substituição de bytes especiais
- ❌ **Dados corrompidos**: Verifique a conexão física

O programa automaticamente **retransmite até 3 vezes**.

### Timeout

Se não houver resposta:

- ❌ **Pinpad desconectado**: Verifique o cabo
- ❌ **Porta serial errada**: Verifique o nome da porta
- ❌ **Configuração incorreta**: Verifique baud rate (19200), 8N1

### CRC Inválido na Resposta

Se o CRC da resposta não bater:

- ❌ **Ruído na linha**: Verifique o cabo
- ❌ **Implementação do CRC**: O programa mostra CRC calculado vs. recebido

## 📦 Estrutura do Código

```
src/
└── main.rs
    ├── Constantes (EOT, ACK, NAK, SYN, ETB, DC3, CAN)
    ├── calculate_crc16() - Calcula CRC-16
    ├── encode_data() - Aplica substituição de bytes especiais
    ├── decode_data() - Reverte substituição
    ├── build_packet() - Monta pacote completo
    ├── AbecsCommand - Estrutura para comandos
    └── PinpadConnection - Gerencia comunicação
        ├── open() - Abre porta serial
        ├── list_ports() - Lista portas disponíveis
        ├── cancel() - Cancela comando em execução
        ├── send_command() - Envia comando (com retransmissão)
        ├── receive_response() - Recebe resposta (com validação)
        └── execute_command() - Envia + Recebe
```

## 🛠️ API Principal

### `PinpadConnection::open(port_name: &str)`

Abre conexão com o Pinpad na porta especificada.

**Configurações:**
- Baud rate: 19200 bps
- Data bits: 8
- Parity: None
- Stop bits: 1
- Timeout: 2 segundos (padrão)

### `AbecsCommand::new(cmd_id: &str)`

Cria um novo comando ABECS.

**Parâmetros:**
- `cmd_id`: Identificador do comando (3 caracteres, ex: "OPN", "DSP", "GIN")

### `command.add_block(data: Vec<u8>)`

Adiciona um bloco de dados ao comando.

**Parâmetros:**
- `data`: Dados do bloco (até 999 bytes)

### `pinpad.execute_command(command: &AbecsCommand, blocking: bool)`

Executa um comando completo (envia + aguarda resposta).

**Parâmetros:**
- `command`: Comando a ser executado
- `blocking`: 
  - `false`: Timeout de 10 segundos (comandos rápidos)
  - `true`: Timeout de 5 minutos (comandos que requerem interação)

**Retorno:**
- `Ok(Vec<u8>)`: Dados da resposta (já decodificados)
- `Err(String)`: Mensagem de erro detalhada

## 📊 Formato dos Pacotes

### Pacote de Comando/Resposta

```
┌─────┬──────────┬──────┬─────────┐
│ SYN │ PKTDATA  │ ETB  │ CRC-16  │
├─────┼──────────┼──────┼─────────┤
│ 16h │ 0-2049 B │ 17h  │ 2 bytes │
└─────┴──────────┴──────┴─────────┘
```

### Substituição de Bytes Especiais em PKTDATA

- `13h` → `13h 33h`
- `16h` → `13h 36h`
- `17h` → `13h 37h`

### Formato do Comando ABECS

```
┌────────┬─────────┬─────────┬─────────┬─────────┐
│ CMD_ID │ LEN1    │ BLOCK1  │ LEN2    │ BLOCK2  │ ...
├────────┼─────────┼─────────┼─────────┼─────────┤
│ 3 B    │ 3 digits│ 0-999 B │ 3 digits│ 0-999 B │
└────────┴─────────┴─────────┴─────────┴─────────┘
```

## 🔐 Bytes Especiais do Protocolo

| Byte | Valor | Nome | Descrição |
|------|-------|------|-----------|
| EOT  | 0x04  | End of Transmission | Resposta ao CAN |
| ACK  | 0x06  | Acknowledge | Pacote aceito |
| DC3  | 0x13  | Device Control 3 | Byte de escape |
| NAK  | 0x15  | Negative Acknowledge | Pacote rejeitado |
| SYN  | 0x16  | Synchronous Idle | Início do pacote |
| ETB  | 0x17  | End of Transmission Block | Fim do pacote |
| CAN  | 0x18  | Cancel | Cancelar comando |

## 📚 Referências

- [Protocolo ABECS 2.12](protocolo_abecs.md) - Especificação completa
- Seção 2.2: Nível de Enlace
- Seção 2.2.1: Formato do Pacote
- Seção 2.2.2: Fluxo de Comunicação
- Seção 7.2: Algoritmo CRC-16

## ⚠️ Observações Importantes

1. **Sempre inicie com CAN**: O programa sempre envia um CAN ao iniciar para cancelar qualquer comando em execução.

2. **Tratamento de NAK**: O programa retransmite automaticamente até 3 vezes quando recebe NAK.

3. **Validação de CRC**: Tanto envio quanto recebimento validam o CRC-16 dos pacotes.

4. **Timeout ajustável**: Comandos blocantes têm timeout de 5 minutos, não-blocantes de 10 segundos.

5. **Debug detalhado**: Todos os pacotes são exibidos em hexadecimal para facilitar debug.

## 🐛 Troubleshooting

### Problema: Porta serial não encontrada

**Solução:** Verifique se o dispositivo está conectado e liste as portas com `ls /dev/tty*` (Linux/Mac) ou Device Manager (Windows).

### Problema: Permissão negada

**Solução Linux/Mac:**
```bash
sudo usermod -a -G dialout $USER
# Faça logout e login novamente
```

### Problema: NAK constante

**Solução:** Verifique:
1. Algoritmo CRC-16 (polinômio 0x1021)
2. Substituição de bytes especiais
3. Formato do comando ABECS

### Problema: Timeout na resposta

**Solução:** Verifique:
1. Cabo e conexão física
2. Baud rate correto (19200)
3. Pinpad está ligado e funcionando

## 📝 Licença

Este projeto é fornecido como exemplo de implementação do Protocolo ABECS 2.12.

## 👨‍💻 Autor

Desenvolvido seguindo rigorosamente a especificação ABECS 2.12 (11-abr-19).
