use std::net::{TcpStream};
use std::collections::HashMap;
use std::io::{BufReader, BufRead};


// returns Result with tuple of (method, path, version) e.g. GET, /, HTTP/1.1
pub fn parse_first_line(first_line: &Vec<u8>) -> Result<(String, String, String), String> {
    let length = first_line.len();

    let start = first_line.iter().position(|&c| c == b' ').unwrap();
    let end = first_line.iter().rposition(|&c| c == b' ').unwrap();

    let method = String::from_utf8_lossy(&first_line[..start]);
    let path = String::from_utf8_lossy(&first_line[start+1..end]);
    let version = String::from_utf8_lossy(&first_line[end+1..length-2]);

    Ok((
        method.to_string(),
        path.to_string(),
        version.to_string(),
    ))
}


pub fn parse_headers( buffer: &mut Vec<u8>, stream: TcpStream) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    let reader = BufReader::new(stream);

    // read line for line
    for line in reader.lines() {
        let line = line.unwrap();
        buffer.append(&mut line.clone().into_bytes());
        // append removed line endings
        buffer.push(0xd);
        buffer.push(0xa);

        // end header parsing
        if buffer.ends_with(&[0xd, 0xa, 0xd, 0xa]) {
            break;
        }

        // skip empty lines
        if line == "" {
            continue;
        }

        // split at ':'
        let sep = line.find(':').unwrap();
        let key = (&line[..sep]).trim().to_string();
        let value = (&line[sep+1..]).trim().to_string();

        headers.insert(key, value);
    }

    headers
}
