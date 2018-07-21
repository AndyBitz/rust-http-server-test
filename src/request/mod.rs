use http1;
use std::fmt;
use std::io::BufReader;
use std::net::{TcpStream};
use std::collections::HashMap;


pub struct Request {
    is_tls: bool,
    method: Option<String>,
    path: Option<String>,
    version: Option<String>,
    headers: Option<HashMap<String, String>>,
    body: Option<BufReader<TcpStream>>,
}


impl Request {
    pub fn from_http1(buffer: &mut Vec<u8>, stream: TcpStream ) -> Self {
        // the buffer must contain the first line
        let (method, path, version) = http1::parse_first_line(&buffer).unwrap();

        // parse headers
        let headers = http1::parse_headers(buffer, stream.try_clone().unwrap());

        // body
        let body = BufReader::new(stream);

        Request{
            is_tls: false,
            method: Some(method),
            path: Some(path),
            version: Some(version),
            headers: Some(headers),
            body: Some(body),
        }
    }

    pub fn from_tls(buffer: &mut Vec<u8>, stream: TcpStream) -> Self {
        Request{
            is_tls: true,
            method: None,
            version: None,
            path: None,
            headers: None,
            body: None,
        }
    }
}


impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let method = match self.method.as_ref() {
            Some(s) => s,
            None => "<NO_METHOD>"
        };

        let path = match self.path.as_ref() {
            Some(s) => s,
            None => "<NO_PATH>",
        };

        let version = match self.version.as_ref() {
            Some(s) => s,
            None => "<NO_VERSION>",
        };

        let headers = match self.headers.as_ref() {
            Some(h) => {
                let mut headers = String::from("");
                for (key, value) in h {
                    headers.push_str(&key);
                    headers.push_str(": ");
                    headers.push_str(&value);
                    headers.push_str("\n");
                }

                headers
            },
            None => String::from("<NO_HEADERS>\n"),
        };

        write!(
            f,
            "{method} {path} {version}\n{headers}<Body...>\n",
            method=method,
            path=path,
            version=version,
            headers=headers
        )
    }
}
