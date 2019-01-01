use convert::{ToMessagePack};

// Struct describe error dto object
#[derive(Debug, Deserialize, Serialize, ToMessagePack)]
pub struct ErrorDTO {
    error: String
}


impl ErrorDTO {
    // Method convert created data structure to message pack
    pub fn to_mspack(error_message: &String) -> std::vec::Vec<u8> {
        let error = Self {
            error: error_message.to_string()
        };
        error.convert()
    }
}