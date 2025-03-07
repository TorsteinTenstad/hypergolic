use clap::Parser;
use common::arg_cache::update_string_cache;
use common::jump::jump_on_term;
use common::log::{log_blue, log_green, log_red};
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
    #[clap(short, long, value_name = "DIR", default_value = "build")]
    build_dir: String,

    /// Target to build. If not specified, use the last target as determined by the cache.
    #[clap(
        short,
        long,
        value_name = "TARGET",
        conflicts_with = "interactive_target_selection"
    )]
    target: Option<String>,

    /// Select a target to build from the list of available targets
    #[clap(short, long)]
    interactive_target_selection: bool,

    /// Clean out the build directory before building
    #[clap(short = 'c', long = "clean")]
    clean: bool,

    /// Treat warnings as errors
    #[clap(short = 'w', long = "warnings-as-errors")]
    warn_as_error: bool,
}

fn main() {
    let args = Args::parse();
    let cache_dir_root = args.cache_dir.as_deref().unwrap_or(&args.build_dir);

    let target = if args.interactive_target_selection {
        let available: Vec<_> = Command::new("ninja")
            .args(["-C", &args.build_dir])
            .args(["-t", "targets"])
            .output()
            .expect("Failed to execute ninja")
            .stdout
            .lines()
            .map_while(Result::ok)
            .filter_map(|l| l.split(':').next().map(|l| l.to_owned()))
            .collect();
        if available.is_empty() {
            log_red(&format!(
                "ninja found no targets in directory {}",
                args.build_dir
            ));
            exit(0);
        }
        match inquire::Select::new("Select a target to build", available).prompt() {
            Ok(target) => Some(target),
            Err(_) => {
                log_red("No target selected");
                exit(0);
            }
        }
    } else {
        args.target
    };
    let target =
        update_string_cache(cache_dir_root, ".nin", "last_target", target).unwrap_or_else(|| {
            log_red("No target specified and no cache found. Use -t, --target or -i, --interactive-target-selection");
            exit(1);
        });

    let jump_term = if args.warn_as_error {
        common::jump::JumpTerm::Warning
    } else {
        common::jump::JumpTerm::Error
    };

    if args.clean {
        log_blue(&format!("ninja -C {} clean", args.build_dir));
        let _ = Command::new("ninja")
            .arg("-C")
            .arg(&args.build_dir)
            .arg("clean")
            .status()
            .expect("Failed to execute ninja clean command");
    }

    log_blue(&format!("ninja -C {} {}", args.build_dir, target));
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

    log_green(&format!("Finished in {:?}", elapsed));
}
