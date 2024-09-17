enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    Terabytes(f64)
}

#[derive(Debug)]
struct Sizes {
    bytes: f64,
    kilobytes: f64,
    megabytes: f64,
    gigabytes: f64,
    terabytes: f64
}

impl Sizes {
    fn new(value: f64) -> Sizes {
        Sizes {
            bytes: value,
            kilobytes: value / 1024.0,
            megabytes: value / 1024.0 / 1024.0,
            gigabytes: value / 1024.0 / 1024.0 / 1024.0,
            terabytes: value / 1024.0 / 1024.0 / 1024.0 / 1024.0
        }
    }

    fn print_sizes(&self) {
        println!("bytes: {}", self.bytes);
        println!("kilobytes: {}", self.kilobytes);
        println!("megabytes: {}", self.megabytes);
        println!("gigabytes: {}", self.gigabytes);
        println!("terabytes: {}", self.terabytes);
    }
}

fn main() {
    // get first argument
    let arg: Vec<String> = std::env::args().collect();
    let size: u64 = match arg.get(1) {
        Some(string_value) => match string_value.parse::<u64>() {
            Ok(size) => size,
            Err(_) => {
                println!("Please provide a valid number as the first argument");
                return;
            }
        },
        None => {
            println!("Please provide the first argument as a number");
            return;
        }
    };  

    println!("Size: {}", size);

    let unit = match arg.get(2) {
        Some(value) => value.trim(),
        None => {
            println!("Please provide the second argument as a unit");
            return;
        }
    };

    //println!("Unit: {}", unit);

    let size_in_bytes = match unit.to_uppercase().as_str() {
        "B" => size,
        "KB" => size * 1024,
        "MB" => size * 1024 * 1024,
        "GB" => size * 1024 * 1024 * 1024,
        "TB" => size * 1024 * 1024 * 1024 * 1024,
        _ => {
            println!("Invalid unit provided. Please provide one of the following: B, KB, MB, GB, TB");
            return;
        }
    };

    //println!("Size in bytes: {}", size_in_bytes);

    let sizes = Sizes::new(size_in_bytes as f64);
    sizes.print_sizes();

}