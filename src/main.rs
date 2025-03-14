use glob::glob;
use std::{env, fs, process}; // Add glob = "0.3.1" to Cargo.toml

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <find> <replace> <file_pattern>", args[0]);
        process::exit(1);
    }

    let find = &args[1];
    let replace = &args[2];
    let pattern = &args[3];

    let entries = glob(pattern).unwrap_or_else(|err| {
        eprintln!("Invalid pattern {}: {}", pattern, err);
        process::exit(1);
    });

    entries
        .filter_map(Result::ok)
        .filter(|path| {
            path.file_name()
                .and_then(|n| Some(n.to_string_lossy()))
                .map_or(false, |name| name.contains(find))
        })
        .try_for_each(|path| {
            let file_name = path.file_name().unwrap_or_default().to_string_lossy();
            let new_file_name = file_name.replace(find, replace);
            let new_path = path.with_file_name(new_file_name);

            fs::rename(&path, &new_path).map(|_| {
                println!("Renamed: {} -> {}", path.display(), new_path.display());
            })
        })
        .unwrap_or_else(|err| eprintln!("Error during renaming: {}", err));
}
