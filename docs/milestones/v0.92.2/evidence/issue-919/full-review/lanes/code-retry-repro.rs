use std::path::Path;
#[derive(Clone)] enum OperatorReviewStatus { Incomplete, Complete, Failed, Cancelled, WithheldPublication }
struct OperatorAttempt {attempt:u64, review_out:String,status:OperatorReviewStatus}
struct OperatorReviewState {active_attempt:u64, attempts:Vec<OperatorAttempt>}
fn attempt_settlement_ref(attempt:u64)->String {format!("attempts/{attempt}/settlement.json")}
fn active_attempt_settled(out: &Path, state: &OperatorReviewState) -> bool {
    let Some(active) = state
        .attempts
        .iter()
        .find(|attempt| attempt.attempt == state.active_attempt)
    else {
        return true;
    };
    match active.status {
        OperatorReviewStatus::Cancelled | OperatorReviewStatus::Failed => {
            out.join(&active.review_out).join("run.json").exists()
                || out.join(attempt_settlement_ref(active.attempt)).exists()
        }
        _ => true,
    }
}

fn main(){
 let out=Path::new("/nonexistent-919-state-fixture");
 let state=OperatorReviewState{active_attempt:1,attempts:vec![OperatorAttempt{attempt:1, review_out:"attempts/1/review".into(),status:OperatorReviewStatus::Incomplete}]};
 let settled=active_attempt_settled(out,&state);
 println!("Incomplete active attempt with no run or settlement receipt: settled={settled}");
 assert!(settled);
 println!("REPRODUCED: running attempt is admitted by retry settlement predicate");
}
