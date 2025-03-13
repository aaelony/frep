use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    // Get the `find` and `replace` arguments from the command line
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <find> <replace>", args[0]);
        process::exit(1);
    }

    let find = &args[1];
    let replace = &args[2];

    // Read the current directory's contents
    let current_dir = "."; // current directory
    let entries = fs::read_dir(current_dir).unwrap_or_else(|err| {
        eprintln!("Error reading directory {}: {}", current_dir, err);
        process::exit(1);
    });

    // Iterate over all files and directories in the current directory
    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                let file_name = path.file_name().unwrap_or_default().to_string_lossy();

                // Only consider files whose names contain the `find` portion
                if file_name.contains(find) {
                    // Create the new file name by replacing `find` with `replace`
                    let new_file_name = file_name.replace(find, replace);
                    let new_path = path.with_file_name(new_file_name);

                    // Rename the file
                    if let Err(err) = fs::rename(&path, &new_path) {
                        eprintln!(
                            "Error renaming {} to {}: {}",
                            path.display(),
                            new_path.display(),
                            err
                        );
                    } else {
                        println!("Renamed: {} -> {}", path.display(), new_path.display());
                    }
                }
            }
            Err(err) => eprintln!("Error reading entry: {}", err),
        }
    }
}
