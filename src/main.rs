use std::env;

mod extractor;

fn main() {
    println!("Extracting file...");
    let cwd = env::current_dir().unwrap();

    let tar_file_path = cwd.join("prune_darwin_amd64.tar");

    extractor::extract(&tar_file_path, "prune", &cwd).unwrap();
}
