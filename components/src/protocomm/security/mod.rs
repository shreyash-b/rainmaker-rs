pub(crate) mod sec0;
pub(crate) mod sec1;
pub(crate) trait SecurityTrait: Default {
    fn security_handler(&self, ep: &str, data: Vec<u8>) -> Vec<u8>;
    fn encrypt(&self, indata: &mut [u8]); // encryption in-place
    fn decrypt(&self, indata: &mut [u8]); // decryption in-place
}

/// Encrypting data in Transit
///
/// [ProtocommSecurity] is used for signifying the level of encryption to provide while utilizing
/// [Protocomm]   
/// It currently provides 2 levels of security:   
///     - Sec0: It provides plaintext communication. Similar to `NO ENCRYPTION`   
///     - Sec1: It provides transport encryption using AES128-CTR cipher with ECDH using Curve25519 for key
///     exchange. Additionally it is also possible to have a Proof-Of-Possession(PoP) key for added
///     security. PoP is need to be shared by both devices beforehand and is not shared on the
///     communication channel
///
/// [Protocomm]: crate::protocomm::Protocomm
pub enum ProtocommSecurity {
    Sec0(sec0::Sec0),
    // wrap sec1 in box due to difference in runtime sizes of sec0 and sec1
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
    fn security_handler(&self, ep: &str, data: Vec<u8>) -> Vec<u8> {
        match self {
            ProtocommSecurity::Sec0(sec_inner) => sec_inner.security_handler(ep, data),
            ProtocommSecurity::Sec1(sec_inner) => sec_inner.security_handler(ep, data),
        }
    }

    fn encrypt(&self, indata: &mut [u8]) {
        match self {
            ProtocommSecurity::Sec0(sec_inner) => sec_inner.encrypt(indata),
            ProtocommSecurity::Sec1(sec_inner) => sec_inner.encrypt(indata),
        }
    }

    fn decrypt(&self, indata: &mut [u8]) {
        match self {
            ProtocommSecurity::Sec0(sec_inner) => sec_inner.decrypt(indata),
            ProtocommSecurity::Sec1(sec_inner) => sec_inner.decrypt(indata),
        }
    }
}
