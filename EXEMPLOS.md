# Exemplos de Uso Simples

## Exemplo Mínimo

```rust
use pinpad::{PinpadConnection, AbecsCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pinpad = PinpadConnection::open("/dev/ttyACM0")?;
    pinpad.execute(&AbecsCommand::open())?;
    println!("✓ Conectado!");
    Ok(())
}
```

## Mostrar Mensagem no Display

```rust
use pinpad::{PinpadConnection, AbecsCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pinpad = PinpadConnection::open("/dev/ttyACM0")?;
    
    // Formato: "TTT" + mensagem
    // TTT = tempo em segundos (032 = 32 segundos)
    let msg = "032          OLÁ MUNDO!      ";
    pinpad.execute(&AbecsCommand::display(msg))?;
    
    Ok(())
}
```

## Obter Informações do Pinpad

```rust
use pinpad::{PinpadConnection, AbecsCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pinpad = PinpadConnection::open("/dev/ttyACM0")?;
    
    let response = pinpad.execute(&AbecsCommand::get_info("01"))?;
    
    if response.is_success() {
        for i in 0..response.block_count() {
            println!("Info: {}", response.get_string(i).unwrap());
        }
    }
    
    Ok(())
}
```

## Sequência de Comandos

```rust
use pinpad::{PinpadConnection, AbecsCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pinpad = PinpadConnection::open("/dev/ttyACM0")?;
    
    // Abre sessão
    pinpad.execute(&AbecsCommand::open())?;
    println!("✓ Sessão aberta");
    
    // Mostra mensagem 1
    pinpad.execute(&AbecsCommand::display("005Aguarde..."))?;
    println!("✓ Mensagem 1 exibida");
    
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // Mostra mensagem 2
    pinpad.execute(&AbecsCommand::display("005Pronto!"))?;
    println!("✓ Mensagem 2 exibida");
    
    // Fecha sessão
    pinpad.execute(&AbecsCommand::close())?;
    println!("✓ Sessão fechada");
    
    Ok(())
}
```

## Modo Debug

```rust
use pinpad::{PinpadConnection, AbecsCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pinpad = PinpadConnection::open("/dev/ttyACM0")?;
    
    // Ativa modo verbose para ver todos os bytes
    pinpad.set_verbose(true);
    
    let response = pinpad.execute(&AbecsCommand::open())?;
    
    // Você verá algo como:
    // → Enviando: OPN (3 bytes)
    //   [0000] 16 4F 50 4E 17 A8 A9 | .OPN...
    // ← ACK
    // ← Aguardando resposta...
    // ✓ CRC válido (6 bytes)
    //   [0000] 4F 50 4E 30 30 30 | OPN000
    
    Ok(())
}
```

## Tratamento de Erros Detalhado

```rust
use pinpad::{PinpadConnection, AbecsCommand, AbecsError};

fn main() {
    match run() {
        Ok(_) => println!("✓ Sucesso!"),
        Err(e) => println!("✗ Erro: {}", e),
    }
}

fn run() -> Result<(), AbecsError> {
    let mut pinpad = PinpadConnection::open("/dev/ttyACM0")?;
    
    let response = pinpad.execute(&AbecsCommand::open())?;
    
    if !response.is_success() {
        return Err(AbecsError::PinpadError {
            status: response.status().to_string(),
            description: response.status_description().to_string(),
        });
    }
    
    Ok(())
}
```

## Comando Personalizado com Múltiplos Blocos

```rust
use pinpad::{PinpadConnection, AbecsCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pinpad = PinpadConnection::open("/dev/ttyACM0")?;
    
    let mut cmd = AbecsCommand::new("MYC"); // Seu comando
    cmd.add_string("param1");
    cmd.add_string("param2");
    cmd.add_block(vec![0x01, 0x02, 0x03]); // Dados binários
    
    let response = pinpad.execute(&cmd)?;
    response.print(); // Exibe resposta formatada
    
    Ok(())
}
```

## Lendo Diferentes Tipos de Dados

```rust
use pinpad::{PinpadConnection, AbecsCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pinpad = PinpadConnection::open("/dev/ttyACM0")?;
    
    let response = pinpad.execute(&AbecsCommand::get_info("01"))?;
    
    if response.is_success() {
        // Como texto
        if let Some(text) = response.get_string(0) {
            println!("Texto: {}", text);
        }
        
        // Como hexadecimal
        if let Some(hex) = response.get_hex(0) {
            println!("Hex: {}", hex);
        }
        
        // Como bytes brutos
        if let Some(bytes) = response.get_block(0) {
            println!("Bytes: {:?}", bytes);
        }
        
        // Todos os blocos como strings
        let all = response.get_all_strings();
        println!("Todos: {:?}", all);
    }
    
    Ok(())
}
```
