use clap::Parser;
use common::arg_cache::update_string_cache;
use common::jump::jump_on_term;
use common::log::{log_blue, log_green};
use std::io::BufRead;
use std::process::{exit, Command};

/// Wrapper for cbuild that jumps VSCode to the source location of the first compilation error.
#[derive(clap_derive::Parser, Debug)]
struct Args {
    /// Features that require persistent state between executions creates files in <PATH>/.cb/
    #[clap(long = "cache_dir", value_name = "PATH", default_value = ".devlocal")]
    cache_dir: String,

    /// Clean out the build directory before building
    #[clap(short = 'c', long = "clean")]
    clean: bool,

    /// Treat warnings as errors
    #[clap(short = 'w', long = "warnings-as-errors")]
    warn_as_error: bool,

    /// Skip the first N detected issues
    #[clap(short = 's', long = "skip", value_name = "N", default_value = "0")]
    skip_issues: usize,

    /// Path to the project file to pass to cbuild.
    /// If not specified, use the last project file as determined by the cache.
    csolution_yml: Option<String>,
}

fn main() {
    let args = Args::parse();

    let jump_term = if args.warn_as_error {
        common::jump::JumpTerm::Warning
    } else {
        common::jump::JumpTerm::Error
    };

    let csolution_yml = update_string_cache(
        &args.cache_dir,
        ".cb",
        "last_csolution_yml",
        args.csolution_yml,
    );

    if args.clean {
        log_blue(format!("cbuild {} --clean", csolution_yml));
        let _ = Command::new("cbuild")
            .arg(&csolution_yml)
            .arg("--clean")
            .status()
            .expect("Failed to execute cbuild clean command");
    }

    log_blue(format!("cbuild {} --context-set --packs", csolution_yml));
    let start_time = std::time::Instant::now();

    let mut cbuild = Command::new("cbuild")
        .arg(csolution_yml)
        .arg("--context-set")
        .arg("--packs")
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute cbuild command");

    let output = cbuild.stdout.take().expect("Failed to capture stdout");

    let reader = std::io::BufReader::new(output);

    let mut issue_count = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        println!("{}", line);
        if jump_on_term(&line, &jump_term).is_some() {
            issue_count += 1;
            if issue_count < args.skip_issues {
                log_blue("Skipping issue".to_string());
            } else {
                cbuild.kill().expect("Failed to terminate cbuild process");
                exit(1);
            }
        }
    }

    let elapsed = start_time.elapsed();

    let _ = cbuild.wait().expect("Failed to wait on cbuild process");

    log_green(format!("Finished in {:?}", elapsed));
}
