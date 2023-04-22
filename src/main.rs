use clap::{Arg, ArgAction, Command};
use commands::{decode, encode, get_chunks, validate, remove_chunk};

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
            Command::new("validate").about("validates chunk type").arg(
                Arg::new("type")
                    .required(true)
                    .help("chunk type to check its validity"),
            )
        )
        .subcommand(
            Command::new("chunks").about("prints list of chunk types")
                .arg(Arg::new("path").required(true).help("path to png file"))
                .arg(Arg::new("with-data")
                    .help("prints chunk's data as string if it's valid UTF-8 message, otherwise it prints data as bytes")
                    .short('d')
                    .action(ArgAction::SetTrue)
            )
        )
        .subcommand(
            Command::new("remove").about("removes chunk from png")
                .arg(Arg::new("path").required(true).help("path to png file"))
                .arg(Arg::new("type").required(true).help("chunk type to be removed"))
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
                Ok(decoded_message) => println!("{}", decoded_message),
                Err(e) => println!("failed to decode message: {}", e),
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
        Some(("chunks", chunks_matches)) => {
            let path = chunks_matches
                .get_one::<String>("path")
                .map(|s| s.as_str())
                .expect("path is required");

            match get_chunks(path) {
                Ok(chunks) => {
                    let print_data = chunks_matches.get_flag("with-data");
                    for chunk in chunks {
                        let mut output = format!("{}", chunk.chunk_type());
                        if print_data {
                            match chunk.data_as_string() {
                                Ok(chunk_data_str) => {
                                    output = format!("{}: {}", output, chunk_data_str)
                                }
                                Err(_) => output = format!("{}: {:?}", output, chunk.data()),
                            }
                        }
                        println!("{}", output);
                    }
                }
                Err(e) => println!("failed to get chunk list: {}", e),
            }
        }
        Some(("remove", remove_matches)) => {
            let path = remove_matches
                .get_one::<String>("path")
                .map(|s| s.as_str())
                .expect("path is required");


            let chunk_type = remove_matches
                .get_one::<String>("type")
                .map(|s| s.as_str())
                .expect("type is required");

            if let Err(e) = remove_chunk(path, chunk_type) {
                println!("failed to remove chunk: {}", e)
            } else {
                println!("chunk has been removed")
            }
        }
        _ => panic!("oh shieet"),
    }
}
