use crate::*;
use std::process::exit;

pub fn update_string_cache(
    cache_dir_root: &str,
    cache_dir: &str,
    cache_filename: &str,
    target: Option<String>,
) -> String {
    let cache_dir = std::path::Path::new(cache_dir_root).join(cache_dir);
    let last_target_path = cache_dir.join(cache_filename);
    match target {
        Some(target) => {
            if std::fs::create_dir_all(&cache_dir).is_err() {
                log::log_red(format!(
                    "Failed to create cache directory {}",
                    cache_dir.to_str().unwrap_or("?")
                ));
            }
            if std::fs::write(&last_target_path, target.as_bytes()).is_err() {
                log::log_red(format!(
                    "Failed to write target cache {}",
                    last_target_path.to_str().unwrap_or("?")
                ));
            }
            target
        }
        None => std::fs::read_to_string(&last_target_path).unwrap_or_else(|_| {
            log::log_red(format!(
                "No target specified and no cache found at {}",
                last_target_path.to_str().unwrap_or("?")
            ));
            exit(1);
        }),
    }
}
