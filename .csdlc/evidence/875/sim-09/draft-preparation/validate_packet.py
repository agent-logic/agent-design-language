"""PVF tooling: deterministic packet integrity/accounting; local CPU/files only.
Required for this preparation draft; not a pilot or release qualification gate.
"""
import json
from pathlib import Path

def validate(d):
    assert d["schema"] == "adl.sim09.preparation_packet.v1"
    assert d["issue"] == 875
    assert d["pilot"] == {"journeys": 0, "resume_receipt": None,
        "activation_performed": False, "acceptance": "not_proven", "required_journeys": 5}
    assert len(d["remaining_acceptance"]) == 7
    assert d["cost"] == {"incremental_paid_budget_usd": 0, "provider_calls": False}
    assert d["candidate"]["source"] == "067cb99bf5c6220f64c9faadd7da6abdca34bcc4"
    assert d["candidate"]["sha256"] == "6d30fcc7aa17c417c444145809968d0c286ad15b3338303963c42761a0620fa9"
    assert d["transition"]["accepted_source"] == "439b5ca0e0bca9bce78f0bde73e8e9c6dabe2be4"
    assert d["transition"]["source_hash"] == "beaff3d1635c705aa545e6fd737cdce7dbf580ce14fe3f70bd8828f6b0a87f4e"
    assert d["blocker"]["tracked_card_files"] == 12
    assert not d["blocker"]["index_present"] and not d["blocker"]["binding_present"]
    assert all(x["exit_code"] == 2 for x in d["read_only_outcomes"])
    assert "not controlled pilot evidence" in d["shadow_boundary"]
    assert "Full issue acceptance unchanged" in d["publication_boundary"]

def main():
    d = json.loads(Path(__file__).with_name("packet.json").read_text())
    validate(d)
    # Ensure the guard rejects false completion, invented authority and sample substitution.
    for key, value in [("journeys", 5), ("resume_receipt", "invented"),
                       ("activation_performed", True), ("acceptance", "pass"),
                       ("required_journeys", 30)]:
        altered = json.loads(json.dumps(d))
        altered["pilot"][key] = value
        try:
            validate(altered)
        except AssertionError:
            continue
        raise AssertionError("false pilot claim accepted: " + key)
    print("preparation packet integrity passed; pilot remains not_proven")

if __name__ == "__main__":
    main()
