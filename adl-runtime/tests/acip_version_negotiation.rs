use adl_runtime::acip::{
    negotiate_version, AcipNegotiationOffer, CSM_ACIP_PROTOCOL_FAMILY, CSM_ACIP_VERSION_MAJOR,
    CSM_ACIP_VERSION_MINOR,
};

fn offer(minimum_minor: u32, maximum_minor: u32) -> AcipNegotiationOffer {
    AcipNegotiationOffer {
        protocol_family: CSM_ACIP_PROTOCOL_FAMILY.to_string(),
        supported_major: CSM_ACIP_VERSION_MAJOR,
        minimum_minor,
        maximum_minor,
        required_features: Vec::new(),
    }
}

#[test]
fn accepts_minor_ranges_containing_the_supported_version() {
    for candidate in [offer(0, 0), offer(0, 1)] {
        let negotiated = negotiate_version(&candidate).expect("offer must contain ACIP 1.0");
        assert_eq!(negotiated.version_major, CSM_ACIP_VERSION_MAJOR);
        assert_eq!(negotiated.version_minor, CSM_ACIP_VERSION_MINOR);
    }
}

#[test]
fn rejects_minor_ranges_that_cannot_negotiate_the_supported_version() {
    for candidate in [offer(1, 1), offer(1, 0)] {
        let error = negotiate_version(&candidate).expect_err("offer must not contain ACIP 1.0");
        assert_eq!(error, "no compatible protocol minor version");
    }
}
