pub trait TransportTrait<'a> {
    fn add_endpoint<T>(&self, ep_name: &str, cb: T)
    where
        T: Fn(String, Vec<u8>) -> Vec<u8> + Send + Sync + 'a;
}

pub mod httpd;
