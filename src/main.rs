use clap::{Arg, Command};
use commands::{decode, encode, validate};

pub mod chunk;
pub mod chunk_type;
pub mod commands;
pub mod png;

fn main() {
    let matches = Command::new("pngme")
        .subcommand_required(true)
        .subcommand(
            Command::new("encode")
                .about("encodes data in the chunk")
                .arg(Arg::new("path").required(true).help("path to png file"))
                .arg(
                    Arg::new("type")
                        .required(true)
                        .help("valid chunk type e.g ruSt"),
                )
                .arg(Arg::new("data").required(false).help("data to be encoded")),
        )
        .subcommand(
            Command::new("decode")
                .about("prints data from the given chunk type")
                .arg(Arg::new("path").required(true).help("path to png file"))
                .arg(
                    Arg::new("type")
                        .required(true)
                        .help("valid chunk type e.g ruSt"),
                ),
        )
        .subcommand(
            Command::new("validate").about("validate chunk type").arg(
                Arg::new("type")
                    .required(true)
                    .help("chunk type to check its validity"),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("encode", encode_matches)) => {
            let path = encode_matches
                .get_one::<String>("path")
                .map(|s| s.as_str())
                .expect("path is required");

            let chunk_type = encode_matches
                .get_one::<String>("type")
                .map(|s| s.as_str())
                .expect("type is required");

            let chunk_data = encode_matches
                .get_one::<String>("data")
                .map(|s| s.as_str())
                .unwrap_or("");

            if let Err(e) = encode(path, chunk_type, chunk_data) {
                println!("failed to encode file {}: {}", path, e);
            }
        }
        Some(("decode", encode_matches)) => {
            let path = encode_matches
                .get_one::<String>("path")
                .map(|s| s.as_str())
                .expect("path is required");

            let chunk_type = encode_matches
                .get_one::<String>("type")
                .map(|s| s.as_str())
                .expect("type is required");

            match decode(path, chunk_type) {
                Ok(decoded_message) => println!(
                    "decoded message from chunk {}: {}",
                    chunk_type, decoded_message
                ),
                Err(e) => println!("failed to decode file {}: {}", path, e),
            }
        }
        Some(("validate", validate_matches)) => {
            let chunk_type = validate_matches
                .get_one::<String>("type")
                .map(|s| s.as_str())
                .expect("type is required");

            match validate(chunk_type) {
                Ok(_) => println!("chunk type is valid"),
                Err(e) => println!("chunk type is invalid: {}", e),
            }
        }
        _ => panic!("oh shieet"),
    }
}
