use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use sha256::try_digest;

fn main() {
    let args = parse_args();
    if args.alg.len() == 0 {
        println!("Please input algorithm parameters");
    } else {
        println!("{}", hash(&args.alg, &args.file));
    };
}

#[derive(Debug, Clone)]
struct Args {
    alg: String,
    file: String,
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    // env::args().into_iter().for_each(|x| println!("arg: {}", x));
    args.iter().fold(Args { alg: String::from(""), file: String::from("") }, |acc, val| {
        let idx = args.iter().position(|x| x == val).unwrap();
        match val.as_str() {
            "-alg" => Args { alg: args[idx + 1].to_string(), file: args[idx + 2].to_string() },
            _ => acc,
        }
    })
}

fn hash(alg: &String, file: &String) -> String {
    return match alg.as_str() {
        "sha256" => sha256(file).to_uppercase(),
        "blake3" => blake3(file).to_uppercase(),
        "autov2" => sha256(file)[0..10].to_uppercase(),
        "crc32" => crc32(file).to_uppercase(),
        _ => format!("{} is not supported", alg),
    };
}

fn sha256(file: &String) -> String {
    let input = Path::new(file);
    return try_digest(input).unwrap();
}

fn blake3(file: &String) -> String {
    let file = File::open(file).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1024];
    let mut hasher = blake3::Hasher::new();
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let t = &buffer[0..n];
                hasher.update(&t);
            }
            Err(e) => panic!("error: {}", e),
        };
    }
    return hasher.finalize().to_string();
}

fn crc32(file: &String) -> String {
    let file = File::open(file).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1024];
    let mut hasher = crc32fast::Hasher::new();
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let t = &buffer[0..n];
                hasher.update(&t);
            }
            Err(e) => panic!("error: {}", e),
        };
    }
    let result = hasher.finalize();
    let mut hex = String::new();
    // civitai crc32 is reverse
    for x in result.to_be_bytes().iter().rev() {
        hex.push_str(&format!("{:02x}", x));
    }
    return hex;
}

#[cfg(test)]
mod tests {
    use std::env;

    #[test]
    fn test_parse() {
        env::args().into_iter().for_each(|x| println!("{}", x));
        let args = super::parse_args();
        assert_eq!(args.alg, "sha256");
        assert_eq!(args.file, "Cargo.toml");
    }
}


