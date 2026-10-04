mod log;

fn main() {
    // Argument
    let level_arg = std::env::args().nth(1).expect("No log level given");
    let path_arg = std::env::args().nth(2).expect("No path given");

    let level = match level_arg.parse::<log::LogLevel>() {
        Ok(level) => level,
        Err(_) => {
            eprint!("Wrong log level arg. Just allow: info, warn, error.");
            std::process::exit(1);
        }
    };

    println!("Level: {level:?}");
}
