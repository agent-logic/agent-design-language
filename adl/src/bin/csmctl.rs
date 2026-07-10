#[cfg(not(test))]
#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;

#[cfg(not(test))]
fn main() {
    cli::run_csmctl_main();
}
