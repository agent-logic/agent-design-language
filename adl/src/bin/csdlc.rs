extern crate adl;

#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;

#[cfg(not(test))]
fn main() {
    cli::run_csdlc_main_named("csdlc");
}

#[cfg(test)]
fn binary_help_probe() -> String {
    cli::csdlc_usage_for("csdlc")
}

#[cfg(test)]
mod tests {
    #[test]
    fn csdlc_cli_binary_links_to_canonical_csdlc_dispatch_surface() {
        let output = super::binary_help_probe();

        assert!(output.contains("csdlc - ADL C-SDLC workflow control-plane binary"));
        assert!(output.contains("csdlc issue run <issue>"));
        assert!(output.contains("adl-csdlc remains a compatibility alias"));
        assert!(output.contains("adl/tools/pr.sh remains the canonical"));
    }
}
