#![recursion_limit="128"]

use syn::{parse_macro_input, DeriveInput};
use quote::{quote};

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(ToMessagePack)]
pub fn derive_to_message_pack(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    // TODO add convert_from method from deserialize from message pack to Struct
    let expanded = quote!{
        use serde::{Serialize, Deserialize};
         use convert::CommonConvertError;
        impl ToMessagePack for #name {
            // Convert method should return vector with message pack payload in <u8>
            fn convert(&self) -> std::vec::Vec<u8> {
                let to_json = serde_json::to_string(&self).unwrap();
                let mut buffer = std::vec::Vec::new();
                to_json.serialize(&mut rmps::Serializer::new(&mut buffer)).unwrap();
                buffer
            }

            // Convert from u8 vector to chosen struct
            fn from_message_pack(data: &std::vec::Vec<u8>) -> Result<Box<Self>, CommonConvertError> {
                let mut bytes = rmps::Deserializer::new(&data[..]);
                let raw_json_string: String = match Deserialize::deserialize(&mut bytes) {
                    Ok(data) => data,
                    Err(_) => return Err(CommonConvertError::convert_error())
                };
                match serde_json::from_str::<Self>(&raw_json_string) {
                    Ok(dto) => Ok(Box::new(dto)),
                    Err(_) => Err(CommonConvertError::convert_error())
                }
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}
