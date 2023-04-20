use clap::{Arg, Command};
use commands::{encode, decode};

mod chunk;
mod chunk_type;
mod commands;
mod png;

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
                panic!("failed to encode file {}: {}", path, e);
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
                Err(e) => panic!("failed to decode file {}: {}", path, e),
            }
        }
        _ => panic!("oh shieet"),
    }
}
