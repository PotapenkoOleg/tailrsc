use std::process;

use tailrsc::{get_args, input::open_files};

#[tokio::main]
async fn main() {
    let args = get_args();

    if args.files.is_empty() {
        return;
    }

    let (_files, errors) = open_files(&args.files).await;

    if !errors.is_empty() {
        for err in &errors {
            eprintln!("{err}");
        }
        process::exit(1);
    }
}
