// Isolated reproduction of the original test oracle, not product execution.
#[tokio::test(start_paused = true)]
async fn original_oracle_accepts_a_600_second_shutdown_delay() {
 use std::time::Duration;
 use tokio_util::sync::CancellationToken;
 let shutdown=CancellationToken::new(); let task_shutdown=shutdown.clone();
 let task=tokio::spawn(async move {
  if task_shutdown.is_cancelled(){return;}
  // Counterexample: cancellation is ignored during the probe.
  let _=tokio::time::timeout(Duration::from_secs(600),std::future::pending::<Result<(), &'static str>>()).await;
  if task_shutdown.is_cancelled(){return;}
 });
 tokio::task::yield_now().await;
 let before=tokio::time::Instant::now();
 shutdown.cancel();
 task.await.unwrap(); // This is the complete original test oracle.
 let delay=before.elapsed();
 println!("original oracle passes despite shutdown delay={} seconds",delay.as_secs());
 assert!(delay >= Duration::from_secs(600));
}
