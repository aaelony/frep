use glob::glob;
use std::{env, fs, path::PathBuf, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <find> <replace> <file_pattern>", args[0]);
        eprintln!("{:?}", args);
        process::exit(1);
    }

    let find = &args[1];
    let replace = &args[2];
    let paths: Box<dyn Iterator<Item = PathBuf>> =
        if args.len() == 4 && (args[3].contains('*') || args[3].contains('?')) {
            Box::new(
                glob(&args[3])
                    .unwrap_or_else(|err| {
                        eprintln!("Invalid pattern {}: {}", args[3], err);
                        process::exit(1);
                    })
                    .filter_map(Result::ok),
            )
        } else {
            // Handle shell-expanded arguments as individual paths
            Box::new(args[3..].iter().map(PathBuf::from))
        };

    paths
        .filter(|path| path.exists())
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
