/// Traits para serialização e desserialização de comandos ABECS
use crate::response::AbecsResponse;

/// Trait para tipos que podem ser serializados como parâmetros de comandos ABECS
pub trait AbecsSerialize {
    /// Serializa o tipo para bytes no formato ABECS
    fn serialize_abecs(&self) -> Vec<u8>;
}

/// Trait para tipos que podem ser desserializados de respostas ABECS
pub trait AbecsDeserialize: Sized {
    /// Desserializa de uma resposta ABECS
    fn deserialize_abecs(response: &AbecsResponse) -> Result<Self, String>;
}

// Implementações para tipos primitivos

impl AbecsSerialize for String {
    fn serialize_abecs(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl AbecsSerialize for &str {
    fn serialize_abecs(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl AbecsSerialize for Vec<u8> {
    fn serialize_abecs(&self) -> Vec<u8> {
        self.clone()
    }
}

impl AbecsSerialize for &[u8] {
    fn serialize_abecs(&self) -> Vec<u8> {
        self.to_vec()
    }
}

/// Cria um parâmetro no formato ABECS (ID + Length + Data)
/// Usado para comandos Abecs que requerem parâmetros com identificadores
pub fn abecs_param(param_id: u16, data: &[u8]) -> Vec<u8> {
    let mut param = Vec::new();

    // ID do parâmetro (2 bytes, big-endian)
    param.push((param_id >> 8) as u8);
    param.push((param_id & 0xFF) as u8);

    // Tamanho dos dados (2 bytes, big-endian)
    let len = data.len() as u16;
    param.push((len >> 8) as u8);
    param.push((len & 0xFF) as u8);

    // Dados
    param.extend_from_slice(data);

    param
}

/// Trait para comandos ABECS tipados
pub trait AbecsTypedCommand {
    /// Tipo da resposta esperada
    type Response: AbecsDeserialize;

    /// ID do comando (3 caracteres)
    fn command_id(&self) -> &str;

    /// Serializa os parâmetros do comando
    fn serialize_params(&self) -> Vec<Vec<u8>>;

    /// Indica se o comando é blocante (requer interação do usuário)
    fn is_blocking(&self) -> bool {
        false
    }
}

/// Macro para criar comandos ABECS de forma fácil
#[macro_export]
macro_rules! abecs_command {
    (
        $(#[$meta:meta])*
        $name:ident {
            id: $cmd_id:literal,
            blocking: $blocking:literal,
            request: { $($req_field:ident: $req_type:ty),* $(,)? },
            response: { $($resp_field:ident: $resp_type:ty),* $(,)? }
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone)]
        pub struct $name {
            $(pub $req_field: $req_type,)*
        }

        paste::paste! {
            #[derive(Debug, Clone)]
            pub struct [<$name Response>] {
                $(pub $resp_field: $resp_type,)*
            }
        }

        impl $crate::serialize::AbecsTypedCommand for $name {
            type Response = paste::paste! { [<$name Response>] };

            fn command_id(&self) -> &str {
                $cmd_id
            }

            fn serialize_params(&self) -> Vec<Vec<u8>> {
                let mut params = Vec::new();
                $(
                    params.push($crate::serialize::AbecsSerialize::serialize_abecs(&self.$req_field));
                )*
                params
            }

            fn is_blocking(&self) -> bool {
                $blocking
            }
        }

        paste::paste! {
            impl $crate::serialize::AbecsDeserialize for [<$name Response>] {
                fn deserialize_abecs(data: &[u8]) -> Result<Self, String> {
                    // Parse dos blocos da resposta
                    let mut blocks = Vec::new();
                    let mut pos = 0;

                    while pos < data.len() {
                        if pos + 3 > data.len() {
                            break;
                        }

                        let len_str = String::from_utf8_lossy(&data[pos..pos + 3]);
                        let block_len = match len_str.parse::<usize>() {
                            Ok(len) => len,
                            Err(_) => break,
                        };

                        pos += 3;

                        if pos + block_len > data.len() {
                            return Err(format!("Bloco incompleto na resposta"));
                        }

                        blocks.push(data[pos..pos + block_len].to_vec());
                        pos += block_len;
                    }

                    // Desserializa cada campo
                    let mut block_idx = 0;
                    $(
                        let $resp_field = if block_idx < blocks.len() {
                            <$resp_type as $crate::serialize::AbecsDeserialize>::deserialize_abecs(&blocks[block_idx])?
                        } else {
                            return Err(format!("Campo {} não encontrado na resposta", stringify!($resp_field)));
                        };
                        block_idx += 1;
                    )*

                    Ok(Self {
                        $($resp_field,)*
                    })
                }
            }
        }
    };
}
