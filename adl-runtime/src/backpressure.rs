//! CSM runtime backpressure contract.

pub const CSM_BACKPRESSURE_REPORT_SCHEMA: &str = "adl.csm.backpressure_report.v1";
pub const CSM_BACKPRESSURE_STATE_SCHEMA: &str = "adl.csm.backpressure_state.v1";
pub const CSM_BACKPRESSURE_COMMAND_RESULT_SCHEMA: &str = "adl.csm.backpressure_command_result.v1";

pub const REQUIRED_STATE_LOSS_POLICY: &str = "never_silent_drop";
pub const NONCRITICAL_LOSS_POLICY: &str = "explicit_defer_or_shed";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backpressure_contract_keeps_required_state_fail_closed() {
        assert_eq!(REQUIRED_STATE_LOSS_POLICY, "never_silent_drop");
        assert_eq!(
            CSM_BACKPRESSURE_STATE_SCHEMA,
            "adl.csm.backpressure_state.v1"
        );
    }
}
