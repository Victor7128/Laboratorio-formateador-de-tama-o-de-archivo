use std::env;

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

#[allow(dead_code)]
fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}

impl FileSize {
    fn parse(input: &str) -> Result<FileSize, String> {
        let (number, unit) = match input.split_once(' ') {
            Some(parts) => parts,
            None => return Err(format!("Error: '{}'", input)),
        };

        let n: f64 = match number.parse() {
            Ok(value) => value,
            Err(_) => return Err(format!("'{}' no es un número válido", number)),
        };

        match unit.to_lowercase().as_str() {
            "b" | "bytes" => Ok(FileSize::Bytes(n as u64)),
            "kb" => Ok(FileSize::Kilobytes(n)),
            "mb" => Ok(FileSize::Megabytes(n)),
            "gb" => Ok(FileSize::Gigabytes(n)),
            other => Err(format!("Unidad desconocida: '{}'", other)),
        }
    }

    fn to_bytes(&self) -> u64 {
        match self {
            FileSize::Bytes(b) => *b,
            FileSize::Kilobytes(kb) => (kb * 1_000.0) as u64,
            FileSize::Megabytes(mb) => (mb * 1_000_000.0) as u64,
            FileSize::Gigabytes(gb) => (gb * 1_000_000_000.0) as u64,
        }
    }
}

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl Sizes {
    fn new(bytes: u64) -> Sizes {
        Sizes {
            bytes: format!("{} bytes", bytes),
            kilobytes: format!("{} kilobytes", bytes / 1_000),
            megabytes: format!("{} megabytes", bytes / 1_000_000),
            gigabytes: format!("{} gigabytes", bytes / 1_000_000_000),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match FileSize::parse(&args[1]) {
        Ok(filesize) => println!("{:?}", Sizes::new(filesize.to_bytes())),
        Err(e) => println!("Error: {}", e),
    }
}