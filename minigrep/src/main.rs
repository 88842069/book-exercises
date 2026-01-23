use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {error}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

// end of main
// another way to handle the Config building
// let config = Config::build(&args);
// match config {
//     Err(error) => {
//         println!("error: {error}");
//     }
//     Ok(config) => {
//         println!(
//             "searching for \"{}\" in file \"{}\"",
//             config.query, config.file_path,
//         );
//
//         println!("with text:\n{contents}");
//     }
// }
