# Comandos Tipados ABECS

Esta biblioteca agora oferece uma API tipada para todos os comandos ABECS, proporcionando maior segurança em tempo de compilação e facilidade de uso.

## Índice

1. [Uso Básico](#uso-básico)
2. [Comandos Disponíveis](#comandos-disponíveis)
3. [Criando Comandos Personalizados](#criando-comandos-personalizados)

## Uso Básico

Existem duas formas de usar a biblioteca:

### API Antiga (Flexível)

```rust
use pinpad::{AbecsCommand, PinpadConnection};

let mut pinpad = PinpadConnection::open("/dev/ttyACM0")?;
let response = pinpad.execute(&AbecsCommand::open())?;

if response.is_success() {
    println!("Sucesso!");
}
```

### API Tipada (Type-Safe)

```rust
use pinpad::{PinpadConnection, OpenCommand};

let mut pinpad = PinpadConnection::open("/dev/ttyACM0")?;
let command = OpenCommand;
let response = pinpad.execute_typed(&command)?;
// response é do tipo OpenResponse
```

## Comandos Disponíveis

### 1. Comandos Básicos

#### OpenCommand - Abertura de Sessão (OPN)

```rust
use pinpad::{PinpadConnection, OpenCommand};

let command = OpenCommand;
let response = pinpad.execute_typed(&command)?;
```

#### CloseCommand - Fechamento de Sessão (CLO)

```rust
use pinpad::{PinpadConnection, CloseCommand};

let command = CloseCommand;
let response = pinpad.execute_typed(&command)?;
```

### 2. Comandos de Display

#### DisplayCommand - Mostrar Mensagem (DSP)

```rust
use pinpad::{PinpadConnection, DisplayCommand};

let command = DisplayCommand::new("BEM-VINDO!");
let response = pinpad.execute_typed(&command)?;
```

**Parâmetros:**
- `message`: String - Mensagem a ser exibida (formato: "032          MENSAGEM       ")

#### ClearDisplayCommand - Limpar Display (CLX)

```rust
use pinpad::{PinpadConnection, ClearDisplayCommand};

let command = ClearDisplayCommand;
let response = pinpad.execute_typed(&command)?;
```

### 3. Comando de Informações

#### GetInfoCommand - Obter Informações (GIN)

```rust
use pinpad::{PinpadConnection, GetInfoCommand};

let command = GetInfoCommand::new("01");
let response = pinpad.execute_typed(&command)?;

println!("Info: {}", response.info);
```

**Parâmetros:**
- `info_type`: String - Tipo de informação solicitada
  - "01": Versão do protocolo ABECS
  - "02": Nome do fabricante
  - "03": Modelo do equipamento
  - etc.

**Resposta:**
- `info`: String - Informação solicitada

### 4. Comandos de Entrada de Dados

#### GetPinCommand - Obter PIN (GPN)

```rust
use pinpad::{PinpadConnection, GetPinCommand};

let command = GetPinCommand::new(
    "DIGITE O PIN",        // mensagem
    4,                      // tamanho mínimo
    12,                     // tamanho máximo
    30,                     // timeout (segundos)
    "01",                   // tipo de criptografia
    "1234567890123456"     // dados adicionais (PAN, etc.)
);
let response = pinpad.execute_typed(&command)?;

println!("PIN Block: {:02X?}", response.pin_block);
```

**Parâmetros:**
- `message`: String - Mensagem a ser exibida
- `min_length`: u8 - Tamanho mínimo do PIN
- `max_length`: u8 - Tamanho máximo do PIN
- `timeout`: u16 - Timeout em segundos
- `crypto_type`: String - Tipo de criptografia
- `additional_data`: String - Dados adicionais (PAN, etc.)

**Resposta:**
- `pin_block`: Vec<u8> - Bloco PIN criptografado

**Nota:** Este é um comando blocante (aguarda interação do usuário).

#### GetDataCommand - Obter Dados (GDU)

```rust
use pinpad::{PinpadConnection, GetDataCommand};

let command = GetDataCommand::new(
    "DIGITE O VALOR",      // mensagem
    1,                      // tamanho mínimo
    10,                     // tamanho máximo
    60                      // timeout (segundos)
);
let response = pinpad.execute_typed(&command)?;

println!("Dados digitados: {}", response.data);
```

**Parâmetros:**
- `message`: String - Mensagem a ser exibida
- `min_length`: u8 - Tamanho mínimo
- `max_length`: u8 - Tamanho máximo
- `timeout`: u16 - Timeout em segundos

**Resposta:**
- `data`: String - Dados digitados pelo usuário

**Nota:** Este é um comando blocante.

#### MenuCommand - Menu de Seleção (MNU)

```rust
use pinpad::{PinpadConnection, MenuCommand};

let options = vec![
    "CREDITO".to_string(),
    "DEBITO".to_string(),
    "VOUCHER".to_string(),
];

let command = MenuCommand::new("SELECIONE", options, 30);
let response = pinpad.execute_typed(&command)?;

println!("Opção selecionada: {}", response.selected_index);
```

**Parâmetros:**
- `title`: String - Título do menu
- `options`: Vec<String> - Lista de opções
- `timeout`: u16 - Timeout em segundos

**Resposta:**
- `selected_index`: u8 - Índice da opção selecionada (base 0)

**Nota:** Este é um comando blocante.

### 5. Comandos de Tabelas

#### TableLoadInitCommand - Iniciar Carga de Tabela (TLI)

```rust
use pinpad::{PinpadConnection, TableLoadInitCommand};

let command = TableLoadInitCommand::new("TAB01");
let response = pinpad.execute_typed(&command)?;
```

**Parâmetros:**
- `table_id`: String - Identificador da tabela

#### TableLoadRecordCommand - Carregar Registro (TLR)

```rust
use pinpad::{PinpadConnection, TableLoadRecordCommand};

let record_data = vec![0x01, 0x02, 0x03, 0x04];
let command = TableLoadRecordCommand::new(record_data);
let response = pinpad.execute_typed(&command)?;
```

**Parâmetros:**
- `record_data`: Vec<u8> - Dados do registro

#### TableLoadFinishCommand - Finalizar Carga de Tabela (TLF)

```rust
use pinpad::{PinpadConnection, TableLoadFinishCommand};

let command = TableLoadFinishCommand;
let response = pinpad.execute_typed(&command)?;
```

### 6. Comandos de Criptografia

#### GetKeyCommand - Obter Chave (GKY)

```rust
use pinpad::{PinpadConnection, GetKeyCommand};

let command = GetKeyCommand::new(0); // índice da chave
let response = pinpad.execute_typed(&command)?;

println!("Key Check Value: {:02X?}", response.key_check_value);
```

**Parâmetros:**
- `key_index`: u8 - Índice da chave

**Resposta:**
- `key_check_value`: Vec<u8> - Valor de verificação da chave (KCV)

## Criando Comandos Personalizados

Você pode criar seus próprios comandos implementando os traits `AbecsTypedCommand` e `AbecsDeserialize`:

```rust
use pinpad::{AbecsTypedCommand, AbecsDeserialize, AbecsResponse};

// Define o comando
#[derive(Debug, Clone)]
pub struct MyCustomCommand {
    pub param1: String,
    pub param2: u8,
}

// Define a resposta
#[derive(Debug, Clone)]
pub struct MyCustomResponse {
    pub result: String,
}

impl MyCustomCommand {
    pub fn new(param1: impl Into<String>, param2: u8) -> Self {
        Self {
            param1: param1.into(),
            param2,
        }
    }
}

// Implementa o comando
impl AbecsTypedCommand for MyCustomCommand {
    type Response = MyCustomResponse;
    
    fn command_id(&self) -> &str {
        "MYC" // ID do seu comando (3 caracteres)
    }
    
    fn serialize_params(&self) -> Vec<Vec<u8>> {
        vec![
            self.param1.as_bytes().to_vec(),
            vec![self.param2],
        ]
    }
    
    fn is_blocking(&self) -> bool {
        false // ou true se aguardar interação do usuário
    }
}

// Implementa a desserialização da resposta
impl AbecsDeserialize for MyCustomResponse {
    fn deserialize_abecs(response: &AbecsResponse) -> Result<Self, String> {
        let result = response.get_string(0)
            .ok_or("Resultado não encontrado")?;
        
        Ok(MyCustomResponse { result })
    }
}

// Uso
let command = MyCustomCommand::new("test", 42);
let response = pinpad.execute_typed(&command)?;
println!("Resultado: {}", response.result);
```

## Vantagens da API Tipada

1. **Type Safety**: O compilador garante que você está usando os tipos corretos
2. **Autocompletar**: IDEs podem sugerir os campos e métodos disponíveis
3. **Documentação**: Cada comando tem sua estrutura bem definida
4. **Facilidade**: Não precisa se preocupar com a serialização dos dados
5. **Compatibilidade**: A API antiga continua funcionando normalmente

## Comparação: API Antiga vs API Tipada

### API Antiga

```rust
// Mais flexível, mas menos segura
let mut cmd = AbecsCommand::new("GIN");
cmd.add_string("01");
let response = pinpad.execute(&cmd)?;

// Precisa parsear manualmente
if let Some(info) = response.get_string(0) {
    println!("Info: {}", info);
}
```

### API Tipada

```rust
// Mais segura e clara
let command = GetInfoCommand::new("01");
let response = pinpad.execute_typed(&command)?;

// Campo tipado, sem parsing manual
println!("Info: {}", response.info);
```

## Dicas

1. Use a **API Tipada** quando o comando já estiver implementado
2. Use a **API Antiga** para prototipação rápida ou comandos raros
3. Crie **comandos personalizados** para comandos específicos da sua aplicação
4. Comandos blocantes (`is_blocking() = true`) têm timeout de 5 minutos
5. Comandos não-blocantes têm timeout de 10 segundos

## Próximos Passos

- Implemente mais comandos conforme necessário
- Estenda os comandos existentes com validações adicionais
- Crie utilitários helper para operações comuns
- Adicione logging e debugging avançado

---

Para mais informações, consulte a [documentação do protocolo ABECS](protocolo_abecs.md).
