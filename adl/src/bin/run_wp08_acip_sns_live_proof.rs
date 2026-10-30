use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    adl::wp08_acip_sns_proof::run_wp08_acip_sns_live_proof(&args)
}
