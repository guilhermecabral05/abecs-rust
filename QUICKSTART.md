# Guia de Início Rápido - Pinpad ABECS

Este guia vai te ajudar a começar a usar a biblioteca em **5 minutos**! ⚡

## 🎯 Objetivo

Ao final deste guia você será capaz de:
- Conectar ao Pinpad
- Executar comandos básicos
- Entender a estrutura da API

## 📋 Pré-requisitos

1. **Rust instalado** (1.70+)
2. **Pinpad conectado** via USB
3. **5 minutos** do seu tempo

## 🚀 Passo a Passo

### 1. Adicionar a Dependência

No seu `Cargo.toml`:

```toml
[dependencies]
pinpad = { path = "../pinpad" }  # Ajuste o caminho
```

### 2. Seu Primeiro Programa

Crie `src/main.rs`:

```rust
use pinpad::{AbecsCommand, PinpadConnection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Conectar
    let mut pinpad = PinpadConnection::open("/dev/ttyACM1")?;
    
    // 2. Abrir sessão
    let cmd = AbecsCommand::Open::new();
    pinpad.execute_typed(&cmd)?;
    
    // 3. Exibir mensagem
    let cmd = AbecsCommand::Display::new("032   OLA MUNDO!    ");
    pinpad.execute_typed(&cmd)?;
    
    // 4. Fechar sessão
    let cmd = AbecsCommand::Close::new();
    pinpad.execute_typed(&cmd)?;
    
    println!("✅ Sucesso!");
    Ok(())
}
```

### 3. Executar

```bash
cargo run
```

**Resultado esperado:**
- Mensagem "OLA MUNDO!" aparece no Pinpad
- Console exibe "✅ Sucesso!"

## 🎓 Entendendo o Código

### Estrutura Básica

Todo programa segue este padrão:

```rust
// 1. Conectar ao Pinpad
let mut pinpad = PinpadConnection::open(PORTA)?;

// 2. Abrir sessão (obrigatório)
pinpad.execute_typed(&AbecsCommand::Open::new())?;

// 3. Executar comandos
pinpad.execute_typed(&AbecsCommand::Display::new("MSG"))?;

// 4. Fechar sessão (obrigatório)
pinpad.execute_typed(&AbecsCommand::Close::new())?;
```

### Padrão de Comandos

Todos os comandos seguem a sintaxe:

```rust
AbecsCommand::NOME_COMANDO::new(parâmetros)
```

Exemplos:
```rust
AbecsCommand::Open::new()              // Sem parâmetros
AbecsCommand::Display::new("texto")    // Com 1 parâmetro
AbecsCommand::GetData::new(            // Com múltiplos parâmetros
    "mensagem",
    min,
    max,
    timeout
)
```

## 📚 Próximos Passos

### Experimente Estes Comandos

#### Obter Informações

```rust
let cmd = AbecsCommand::GetInfo::new("01");
let response = pinpad.execute_typed(&cmd)?;
println!("Versão ABECS: {}", response.info);
```

#### Limpar Display

```rust
let cmd = AbecsCommand::ClearDisplay::new();
pinpad.execute_typed(&cmd)?;
```

### Explorar Exemplos

A biblioteca inclui 7 exemplos completos:

```bash
cargo run --example 01_basico
cargo run --example 02_informacoes
cargo run --example 03_menu
# ... e mais!
```

**[📖 Ver todos os exemplos](examples/README.md)**

## 🔧 Solução de Problemas

### "Permission Denied"

```bash
sudo usermod -a -G dialout $USER
# Faça logout e login
```

### "No such file or directory"

Verifique a porta:
```bash
ls /dev/ttyACM*
```

Ajuste no código:
```rust
let mut pinpad = PinpadConnection::open("/dev/ttyACM1")?;  // ou COM3 no Windows
```

### "Timeout"

- Verifique se o Pinpad está ligado
- Confirme a porta correta
- Tente com `set_verbose(true)` para debug

## 💡 Dicas

### 1. Sempre abra e feche a sessão

```rust
// ✅ Correto
pinpad.execute_typed(&AbecsCommand::Open::new())?;
// ... seus comandos ...
pinpad.execute_typed(&AbecsCommand::Close::new())?;

// ❌ Errado
pinpad.execute_typed(&AbecsCommand::Display::new("MSG"))?;  // Sem Open
```

### 2. Use o modo verbose para debug

```rust
let mut pinpad = PinpadConnection::open("/dev/ttyACM1")?;
pinpad.set_verbose(true);  // Ver todos os bytes trocados
```

### 3. Trate erros apropriadamente

```rust
match pinpad.execute_typed(&cmd) {
    Ok(response) => {
        println!("✅ Sucesso: {:?}", response);
    }
    Err(e) => {
        eprintln!("❌ Erro: {}", e);
    }
}
```

### 4. Comandos blocantes

Alguns comandos aguardam o usuário:
- `GetPin` - Aguarda senha
- `GetData` - Aguarda digitação
- `Menu` - Aguarda seleção

Eles têm timeout de 5 minutos automaticamente.

## 📖 Documentação Completa

- **[README.md](README.md)** - Documentação principal
- **[examples/](examples/)** - 7 exemplos completos
- **[TYPED_COMMANDS.md](TYPED_COMMANDS.md)** - Todos os comandos disponíveis

## 🎯 Desafio

Tente criar um programa que:
1. Exibe o valor de uma compra
2. Mostra um menu com formas de pagamento
3. Exibe "OBRIGADO!"

**Solução:** Veja `examples/07_transacao_completa.rs`

## 🆘 Precisa de Ajuda?

- Veja os exemplos em `examples/`
- Consulte a documentação ABECS do seu Pinpad
- Use `set_verbose(true)` para debug

---

**Parabéns! Você está pronto para usar a biblioteca! 🎉**

Explore os exemplos e a documentação para aprender mais.
