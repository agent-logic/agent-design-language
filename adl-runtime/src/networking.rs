//! CSM runtime networking contract.

pub const CSM_NETWORKING_SCHEMA: &str = "adl.csm.networking.v1";
pub const CSM_POOLING_PLAN_SCHEMA: &str = "adl.csm.pooling_plan.v1";
pub const CSM_POOL_STATUS_SCHEMA: &str = "adl.csm.connection_pool_status.v1";
pub const CSM_LOCAL_PORT_RANGE_START: u16 = 19950;
pub const CSM_LOCAL_PORT_RANGE_END: u16 = 19999;
pub const CSM_MAIN_API_PORT: u16 = 19997;
pub const CSM_LOOPBACK_HOST: &str = "127.0.0.1";
pub const CSM_MAIN_API_BIND: &str = "127.0.0.1:19997";
pub const CSM_DEADPOOL_CRATE: &str = "deadpool";
pub const CSM_DEADPOOL_MODEL: &str = "deadpool::unmanaged";
pub const CSM_DEFAULT_POOL_CAPACITY: usize = 4;

pub fn csm_reserved_range_label() -> String {
    format!("{CSM_LOCAL_PORT_RANGE_START}-{CSM_LOCAL_PORT_RANGE_END}")
}

pub fn is_csm_reserved_local_port(port: u16) -> bool {
    (CSM_LOCAL_PORT_RANGE_START..=CSM_LOCAL_PORT_RANGE_END).contains(&port)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_csm_port_is_inside_reserved_range() {
        assert_eq!(CSM_MAIN_API_PORT, 19997);
        assert_eq!(CSM_MAIN_API_BIND, "127.0.0.1:19997");
        assert!(is_csm_reserved_local_port(CSM_MAIN_API_PORT));
        assert_eq!(csm_reserved_range_label(), "19950-19999");
    }
}
