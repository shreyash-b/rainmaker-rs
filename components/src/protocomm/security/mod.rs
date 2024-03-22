pub(crate) mod sec0;
pub(crate) mod sec1;
pub(crate) trait SecurityTrait: Default {
    fn security_handler(&mut self, ep: String, data: Vec<u8>) -> Vec<u8>;
    fn encrypt(&mut self, indata: &mut [u8]); // encryption in-place
    fn decrypt(&mut self, indata: &mut [u8]); // decryption in-place
}

pub enum ProtocommSecurity {
    Sec0(sec0::Sec0),
    Sec1(Box<sec1::Sec1>),
}

impl Default for ProtocommSecurity {
    fn default() -> Self {
        Self::Sec0(sec0::Sec0)
    }
}

impl ProtocommSecurity {
    pub fn new_sec1(pop: Option<String>) -> Self {
        let sec1 = sec1::Sec1 {
            pop,
            ..Default::default()
        };

        Self::Sec1(Box::new(sec1))
    }
}

impl SecurityTrait for ProtocommSecurity {
    fn security_handler(&mut self, ep: String, data: Vec<u8>) -> Vec<u8> {
        match self {
            ProtocommSecurity::Sec0(sec_inner) => sec_inner.security_handler(ep, data),
            ProtocommSecurity::Sec1(sec_inner) => sec_inner.security_handler(ep, data),
        }
    }

    fn encrypt(&mut self, indata: &mut [u8]) {
        match self {
            ProtocommSecurity::Sec0(sec_inner) => sec_inner.encrypt(indata),
            ProtocommSecurity::Sec1(sec_inner) => sec_inner.encrypt(indata),
        }
    }

    fn decrypt(&mut self, indata: &mut [u8]) {
        match self {
            ProtocommSecurity::Sec0(sec_inner) => sec_inner.decrypt(indata),
            ProtocommSecurity::Sec1(sec_inner) => sec_inner.decrypt(indata),
        }
    }
}
