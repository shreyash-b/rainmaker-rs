pub struct HttpClient<T> {
    pub(crate) client: T
}

pub enum HttpMethod {
    GET,
    POST
}