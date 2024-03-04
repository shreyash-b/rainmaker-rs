pub(crate) trait SecurityTrait{
    type SecParams;

    fn security_handler(ep: String, data: Vec<u8>) -> Vec<u8>;
    fn encrypt(indata: Vec<u8>, sec_params: Self::SecParams) -> Vec<u8>;
    fn decrypt(indata: Vec<u8>, sec_params: Self::SecParams) -> Vec<u8>;
}

pub(crate) mod sec0;