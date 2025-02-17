use clap::Parser;
use common::arg_cache::update_string_cache;
use common::jump::jump_on_term;
use common::log::{log_blue, log_green};
use std::io::BufRead;
use std::process::{exit, Command};

/// Wrapper for ninja that jumps VSCode to the source location of the first compilation error.
#[derive(clap_derive::Parser, Debug)]
struct Args {
    /// Features that require persistent state between executions creates files in <PATH>/.nin/.
    /// If not specified, use the ninja build directory.
    #[clap(long = "cache_dir", value_name = "DIR")]
    cache_dir: Option<String>,

    /// Directory where ninja build files are located
    #[clap(
        short = 'b',
        long = "build-dir",
        default_value = "build",
        value_name = "DIR"
    )]
    build_dir: String,

    /// Clean out the build directory before building
    #[clap(short = 'c', long = "clean")]
    clean: bool,

    /// Treat warnings as errors
    #[clap(short = 'w', long = "warnings-as-errors")]
    warn_as_error: bool,

    /// Target to build. If not specified, use the last target as determined by the cache.
    target: Option<String>,
}

fn main() {
    let args = Args::parse();
    let cache_dir_root = args.cache_dir.as_deref().unwrap_or(&args.build_dir);

    let target = update_string_cache(cache_dir_root, ".nin", "last_target", args.target);
    let jump_term = if args.warn_as_error {
        common::jump::JumpTerm::Warning
    } else {
        common::jump::JumpTerm::Error
    };

    if args.clean {
        log_blue(format!("ninja -C {} clean", args.build_dir));
        let _ = Command::new("ninja")
            .arg("-C")
            .arg(&args.build_dir)
            .arg("clean")
            .status()
            .expect("Failed to execute ninja clean command");
    }

    log_blue(format!("ninja -C {} {}", args.build_dir, target));
    let start_time = std::time::Instant::now();

    let mut ninja = Command::new("ninja")
        .arg("-C")
        .arg(args.build_dir)
        .arg(target)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute ninja command");

    let output = ninja.stdout.take().expect("Failed to capture stdout");

    let reader = std::io::BufReader::new(output);

    for line in reader.lines() {
        let line = line.unwrap();

        println!("{}", line);
        if jump_on_term(&line, &jump_term).is_some() {
            ninja.kill().expect("Failed to terminate ninja process");
            exit(1);
        }
    }

    let elapsed = start_time.elapsed();

    let _ = ninja.wait().expect("Failed to wait on ninja process");

    log_green(format!("Finished in {:?}", elapsed));
}
