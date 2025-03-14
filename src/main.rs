use glob::glob;
use std::{env, fs, path::PathBuf, process};

/// Processes file rename operations based on a search pattern.
///
/// # Arguments
///
/// * `paths` - An iterator of PathBuf objects representing the files to process
/// * `find` - The string pattern to find in filenames
/// * `replace` - The string to replace the found pattern with
///
/// # Example
///
/// ```no_run
/// use std::path::PathBuf;
///
/// let paths = Box::new(vec![PathBuf::from("test-bar.txt")].into_iter());
/// rename_files(paths, "bar", "foo");
/// // Will rename "test-bar.txt" to "test-foo.txt"
/// ```
fn rename_files(paths: Box<dyn Iterator<Item = PathBuf>>, find: &str, replace: &str) {
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

/// A file renaming utility that searches for patterns in filenames and replaces them.
///
/// # Usage
///
/// ```text
/// frep <find> <replace> <file_pattern>
/// ```
///
/// # Arguments
///
/// * `find` - The string pattern to find in filenames
/// * `replace` - The string to replace the found pattern with
/// * `file_pattern` - A file pattern (can include glob patterns like * and ?)
///
/// # Examples
///
/// ```bash
/// # Rename all files containing "bar" to "foo" in that match *.txt
/// frep bar foo *.txt
///
/// # Rename specific files containing "old" to contain "new"
/// frep old new test/old-specific-file.txt
///
/// # Use with glob patterns
/// frep test prod "src/**/*"
/// ```
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!(
            "{} version {}\nA utility that will rename file parts that match the <file_pattern>.\nUsage: {} <find> <replace> <file_pattern>",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            args[0]
        );
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
            // Handle shell-expanded paths
            let owned_paths: Vec<PathBuf> = args[3..].iter().map(PathBuf::from).collect();
            Box::new(owned_paths.into_iter())
        };

    rename_files(paths, find, replace);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn test_rename_match_single_file() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test-bar.txt");
        File::create(&test_file).unwrap();

        let paths = Box::new(std::iter::once(test_file.clone()));
        rename_files(paths, "bar", "foo");

        let expected_file = temp_dir.path().join("test-foo.txt");
        assert!(expected_file.exists());
        assert!(!test_file.exists());
        drop(temp_dir);
    }

    #[test]
    fn test_rename_match_multiple_files() {
        let temp_dir = TempDir::new().unwrap();
        let test_files = vec![
            temp_dir.path().join("test-bar.txt"),
            temp_dir.path().join("other-bar.txt"),
            temp_dir.path().join("no-match.txt"),
        ];

        for file in &test_files {
            File::create(file).unwrap();
        }

        let paths = Box::new(test_files.clone().into_iter());
        rename_files(paths, "bar", "foo");

        assert!(temp_dir.path().join("test-foo.txt").exists());
        assert!(temp_dir.path().join("other-foo.txt").exists());
        assert!(temp_dir.path().join("no-match.txt").exists());
        assert!(!temp_dir.path().join("test-bar.txt").exists());
        assert!(!temp_dir.path().join("other-bar.txt").exists());
        drop(temp_dir);
    }

    #[test]
    fn test_rename_no_matches() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        File::create(&test_file).unwrap();

        let paths = Box::new(std::iter::once(test_file.clone()));
        rename_files(paths, "bar", "foo");

        assert!(test_file.exists());
        drop(temp_dir);
    }
}
