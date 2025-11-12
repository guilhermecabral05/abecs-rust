# Exemplos de Uso - Biblioteca Pinpad ABECS

Esta pasta contém exemplos completos e bem documentados de como usar a biblioteca Pinpad ABECS em diferentes cenários.

## 📋 Lista de Exemplos

### 🔰 Básico

#### [01_basico.rs](01_basico.rs) - Uso Básico
Demonstra as operações fundamentais:
- Listar portas disponíveis
- Conectar ao Pinpad
- Abrir sessão (OPN)
- Exibir mensagens (DSP)
- Limpar display (CLX)
- Fechar sessão (CLO)

```bash
cargo run --example 01_basico
```

#### [02_informacoes.rs](02_informacoes.rs) - Obter Informações
Demonstra como obter informações do Pinpad:
- Versão do protocolo ABECS
- Fabricante e modelo
- Número de série
- Capacidades do equipamento

```bash
cargo run --example 02_informacoes
```

### 🎯 Interação com Usuário

#### [03_menu.rs](03_menu.rs) - Menu Interativo
Demonstra como criar menus de seleção:
- Menu de formas de pagamento
- Menu de confirmação
- Tratamento de timeout
- ⚠️ Comando blocante

```bash
cargo run --example 03_menu
```

#### [04_entrada_dados.rs](04_entrada_dados.rs) - Entrada de Dados
Demonstra captura de dados digitados:
- Capturar valores monetários
- Códigos de autorização
- CPF com formatação
- ⚠️ Comando blocante

```bash
cargo run --example 04_entrada_dados
```

#### [05_captura_pin.rs](05_captura_pin.rs) - Captura de PIN
Demonstra captura segura de senha:
- Captura de PIN criptografado
- Configuração de parâmetros
- PIN block em hexadecimal
- ⚠️ Comando blocante
- ⚠️ Exemplo demonstrativo (configure chaves em produção!)

```bash
cargo run --example 05_captura_pin
```

### 🔧 Avançado

#### [06_comando_personalizado.rs](06_comando_personalizado.rs) - Comandos Personalizados
Demonstra como criar seus próprios comandos ABECS:
- Definir estruturas de comando e resposta
- Implementar traits necessários
- Serialização e desserialização personalizadas
- Comandos blocantes e não-blocantes
- Perfeito para novos comandos da especificação ABECS

```bash
cargo run --example 06_comando_personalizado
```

#### [07_transacao_completa.rs](07_transacao_completa.rs) - Fluxo Completo de Transação
Demonstra um fluxo completo de pagamento:
- Exibição de valor
- Seleção de forma de pagamento
- Leitura de cartão (simulado)
- Captura de PIN
- Processamento (simulado)
- Exibição de resultado
- ⚠️ Exemplo educacional simplificado

```bash
cargo run --example 07_transacao_completa
```

## 🚀 Como Executar

### Pré-requisitos

1. **Rust instalado** (1.70 ou superior)
2. **Pinpad conectado** via USB
3. **Permissões de acesso** à porta serial:
   ```bash
   sudo usermod -a -G dialout $USER
   # Faça logout e login novamente
   ```

### Executar um exemplo

```bash
# No diretório raiz do projeto
cargo run --example NOME_DO_EXEMPLO
```

Exemplo:
```bash
cargo run --example 01_basico
```

### Executar com verbose (ver bytes trocados)

Edite o exemplo e descomente a linha:
```rust
pinpad.set_verbose(true);
```

## 📝 Configuração

### Porta Serial

A maioria dos exemplos usa `/dev/ttyACM1` como porta padrão. Ajuste conforme necessário:

```rust
let port_name = "/dev/ttyACM1"; // Linux
// let port_name = "COM3";      // Windows
```

Para descobrir sua porta:
```bash
# Linux
ls /dev/tty*

# Ou use o exemplo para listar
cargo run --example 01_basico
```

## 🎓 Aprendendo a Biblioteca

### Ordem Recomendada

1. **01_basico.rs** - Comece aqui para entender o básico
2. **02_informacoes.rs** - Aprenda a obter dados do Pinpad
3. **03_menu.rs** - Interação básica com usuário
4. **04_entrada_dados.rs** - Captura de dados
5. **05_captura_pin.rs** - Captura segura de senha
6. **07_transacao_completa.rs** - Veja tudo junto em um fluxo real
7. **06_comando_personalizado.rs** - Crie seus próprios comandos

### Estrutura dos Exemplos

Todos os exemplos seguem uma estrutura similar:

```rust
use pinpad::{AbecsCommand, PinpadConnection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Conectar ao Pinpad
    let mut pinpad = PinpadConnection::open("/dev/ttyACM1")?;
    
    // 2. Abrir sessão
    let cmd = AbecsCommand::Open::new();
    pinpad.execute_typed(&cmd)?;
    
    // 3. Executar comandos
    let cmd = AbecsCommand::Display::new("MENSAGEM");
    pinpad.execute_typed(&cmd)?;
    
    // 4. Fechar sessão
    let cmd = AbecsCommand::Close::new();
    pinpad.execute_typed(&cmd)?;
    
    Ok(())
}
```

## 🔍 API da Biblioteca

### Comandos Disponíveis

```rust
// Básicos
AbecsCommand::Open::new()
AbecsCommand::Close::new()

// Display
AbecsCommand::Display::new("mensagem")
AbecsCommand::ClearDisplay::new()

// Informações
AbecsCommand::GetInfo::new("01")

// Entrada de dados (blocantes)
AbecsCommand::GetPin::new(msg, min, max, timeout, crypto, pan)
AbecsCommand::GetData::new(msg, min, max, timeout)
AbecsCommand::Menu::new(titulo, opcoes, timeout)

// Tabelas
AbecsCommand::TableLoadInit::new(table_id)
AbecsCommand::TableLoadRecord::new(data)
AbecsCommand::TableLoadFinish::new()

// Criptografia
AbecsCommand::GetKey::new(key_index)
```

### Executar Comandos

```rust
// Comando não-blocante (timeout: 10s)
let response = pinpad.execute_typed(&cmd)?;

// O método detecta automaticamente se o comando é blocante
// Comandos blocantes têm timeout de 5 minutos
```

## ⚠️ Importante

### Comandos Blocantes

Alguns comandos aguardam interação do usuário:
- `GetPin` - Aguarda digitação da senha
- `GetData` - Aguarda digitação de dados
- `Menu` - Aguarda seleção de opção

Estes comandos têm timeout de **5 minutos** por padrão.

### Em Produção

Os exemplos são **educacionais e simplificados**. Em produção você deve:

✅ Implementar tratamento de erros robusto
✅ Configurar chaves criptográficas corretamente
✅ Integrar com processadora de pagamentos
✅ Armazenar logs de transações
✅ Implementar estornos e cancelamentos
✅ Emitir comprovantes
✅ Validar dados do cartão
✅ Implementar retry logic apropriado

## 🐛 Solução de Problemas

### Erro: Permission Denied

```bash
sudo usermod -a -G dialout $USER
# Faça logout e login
```

### Erro: Port Not Found

Verifique se o Pinpad está conectado:
```bash
ls /dev/ttyACM*
```

### Timeout

- Verifique se o Pinpad está ligado
- Confirme a porta correta
- Tente aumentar o timeout

### Comando não funciona

- Consulte a especificação ABECS do seu Pinpad
- Alguns comandos podem ter parâmetros diferentes
- Use `set_verbose(true)` para debug

## 📚 Recursos Adicionais

- **[README.md](../README.md)** - Documentação principal
- **[TYPED_COMMANDS.md](../TYPED_COMMANDS.md)** - API de comandos tipados
- **Especificação ABECS** - Consulte a documentação do seu Pinpad

## 💡 Contribuindo

Tem um exemplo útil? Contribua com um Pull Request!

Exemplos desejados:
- Integração com diferentes processadoras
- Leitura de cartão com chip
- Leitura NFC/contactless
- Impressão de comprovantes
- Multi-threading
- Async/await

## 📄 Licença

MIT - Veja [LICENSE](../LICENSE) para detalhes.

---

**Divirta-se programando! 🚀**
