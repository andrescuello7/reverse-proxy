pub enum Errs {
    Err400(&'static str),
    Err500(&'static str),
    Err503(&'static str),
    Err502(&'static str),
}

impl Errs {
    pub fn message(&self) -> &'static str {
        match self {
            Errs::Err400(_) => "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\n\r\nBad request",
            Errs::Err500(_) => "HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/plain\r\n\r\nInternal Server",
            Errs::Err503(_) => "HTTP/1.1 503 Service Unavailable\r\nContent-Type: text/plain\r\n\r\nMalformed request",
            Errs::Err502(_) => "HTTP/1.1 502 Bad Gateway\r\nContent-Type: text/plain\r\n\r\nMalformed request",
        }
    }
}
