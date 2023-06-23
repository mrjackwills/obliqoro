use std::time::SystemTime;
fn main() {
    // Inject BUILD_DATE into cargo envs
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("This error shouln't happen")
        .as_secs();
    println!("cargo:rustc-env=BUILD_DATE={now}");
    tauri_build::build()
}
