// The only remaining source consumer of this excluded compatibility module
// is adl_review.rs. Public ADL compiles public_dispatch.rs directly.
// Original dispatcher and its tests are preserved in Git and retirement custody.
pub fn run_review_main() {
    eprintln!("The historical adl-review source-tree wrapper was retired by #1223. Use the separately qualified CodeFriend owner command; no historical validation was executed.");
    std::process::exit(2);
}
