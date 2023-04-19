use clap::{ Command, Arg};

mod chunk;
mod chunk_type;
mod png;

fn main() {
    let matches = Command::new("pngme")
        .subcommand_required(true)
        .subcommand(
            Command::new("encode")
                .about("encodes data in the chunk")
                .arg(Arg::new("path").required(true).help("path to png file"))
                .arg(Arg::new("type").required(true).help("valid chunk type e.g ruSt"))
                .arg(Arg::new("data").required(false).help("data to be encoded"))
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

            println!("command: {} {} {}", path, chunk_type, chunk_data);
        }
        _ => panic!("oh shieet"),
    }
}
