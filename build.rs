use std::process::Command;

fn main() {
    // run pnpm webpack
    let status = Command::new("pnpm")
        .args(["webpack"])
        .status()
        .expect("Failed to run pnpm webpack");

    if !status.success() {
        panic!("pnpm webpack failed with status: {:?}", status.code());
    }

}
