extern crate embed_resource;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        // on windows we will set our game icon as icon for the executable
        match embed_resource::compile(
            "build/windows/icon.rc",
            embed_resource::NONE,
        ) {
            embed_resource::CompilationResult::NotWindows => {
                println!("Not running on Windows, skipping icon embedding")
            }
            embed_resource::CompilationResult::Ok => {
                println!("Successfully embedded icon resource")
            }
            embed_resource::CompilationResult::NotAttempted(reason) => {
                println!("Icon embedding not attempted: {}", reason)
            }
            embed_resource::CompilationResult::Failed(error) => {
                panic!("Failed to embed icon resource: {}", error)
            }
        }
    }
}
