// src/main.rs

// Logic has been moved to src/app.rs as per Engineering Requirement #6.

mod app;

fn main() {

    if let Err(e) = app::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
