use std::process::Command;

fn main() {
    // Inject git commit version information.
    let output = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
    let commit_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=COMMIT_HASH={}", commit_hash[0..8].trim_end())
}
