#[macro_use]
extern crate serde_derive;

pub use convert_derive::*;

pub trait ToMessagePack {

    fn convert(&self) -> std::vec::Vec<u8>;
    fn from_message_pack(data: &std::vec::Vec<u8>) -> Self;
}


#[cfg(test)]
mod tests_to_message_pack_derive {
    use super::*;
    use rmp_serde as rmps;

    #[derive(Debug, Deserialize, Serialize, ToMessagePack)]
    struct SomeDTO {
        name: String
    }

    #[test]
    fn test_should_check_convert_to_struct_to_message_pack() {
        let dto = SomeDTO{name: "Andy".to_string()};
        assert_eq!(
            vec![175, 123, 34, 110, 97, 109, 101, 34, 58, 34, 65, 110, 100, 121, 34, 125],
            dto.convert()
        );
    }

    #[test]
    fn test_should_convert_message_pack_to_struct() {
        let income_data = vec![175, 123, 34, 110, 97, 109, 101, 34, 58, 34, 65, 110, 100, 121, 34, 125];
        let some_struct = SomeDTO::from_message_pack(&income_data);
        assert_eq!(some_struct.name, "Andy".to_string());
    }

    // TODO Next step add Error handling
}
