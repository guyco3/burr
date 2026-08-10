wit_bindgen::generate!({
    world: "fuzzer-world",
    path: "wit",
    generate_all
});

use crate::wasi::filesystem::preopens::get_directories;
use crate::wasi::filesystem::types::{Descriptor, DescriptorFlags, OpenFlags, PathFlags};
use exports::local::adversary_fuzzer::fuzzer::Guest;
use std::env;

struct FuzzerComponent;

impl Guest for FuzzerComponent {
    async fn run_fuzzer() {
        println!("[FUZZER] Starting adversary fuzzer...");

        let depth = env::var("FUZZ_DEPTH")
            .unwrap_or_else(|_| "3".to_string())
            .parse::<usize>()
            .unwrap_or(3);
        println!("[FUZZER] Depth: {}", depth);

        let dirs = get_directories();
        if dirs.is_empty() {
            println!("[FUZZER] No preopens available.");
            return;
        }

        let root_dir = &dirs[0].0;
        let alphabet = vec![".burr", "..", "/", "target", "symlink"];

        let mut path_stack = vec![];

        // Very simple fuzzer: try to open paths formed by combining alphabet segments up to depth
        Self::explore_paths(root_dir, &alphabet, &mut path_stack, depth).await;

        println!("[FUZZER] Fuzzer completed with no successful breakouts.");
    }
}

impl FuzzerComponent {
    async fn explore_paths(
        root_dir: &Descriptor,
        alphabet: &[&str],
        current_path: &mut Vec<String>,
        max_depth: usize,
    ) {
        if current_path.len() >= max_depth {
            return;
        }

        for &segment in alphabet {
            current_path.push(segment.to_string());
            let path_str = current_path.join("/");

            // Try to open it
            let res = root_dir
                .open_at(
                    PathFlags::empty(),
                    path_str.clone(),
                    OpenFlags::empty(),
                    DescriptorFlags::empty(),
                )
                .await;

            if res.is_ok() {
                println!("[FUZZER] SUCCESSFULLY ACCESSED: {}", path_str);

                // If we successfully accessed something containing .burr or .., that's a breakout
                if path_str.contains(".burr") || path_str.contains("..") {
                    println!(
                        "[FUZZER] CRITICAL VULNERABILITY! Sandbox breakout via path: {}",
                        path_str
                    );
                    std::process::exit(1); // Breakout!
                }
            }

            // Recurse
            Box::pin(Self::explore_paths(
                root_dir,
                alphabet,
                current_path,
                max_depth,
            ))
            .await;

            current_path.pop();
        }
    }
}

export!(FuzzerComponent);
