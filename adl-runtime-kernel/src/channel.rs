use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::mpsc;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelFullPolicy {
    Block,
    Reject,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum SendError {
    #[error("bounded channel is full")]
    Full,
    #[error("bounded channel is closed")]
    Closed,
}

#[derive(Clone, Debug)]
pub struct ChannelMetrics {
    capacity: usize,
    state: Arc<ChannelMetricState>,
}

#[derive(Debug, Default)]
struct ChannelMetricState {
    generation: AtomicU64,
    sent: AtomicU64,
    rejected: AtomicU64,
    depth: AtomicU64,
    high_water: AtomicU64,
}

impl ChannelMetrics {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            state: Arc::new(ChannelMetricState::default()),
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn sent(&self) -> u64 {
        self.state.sent.load(Ordering::Relaxed)
    }

    pub fn rejected(&self) -> u64 {
        self.state.rejected.load(Ordering::Relaxed)
    }

    pub fn depth(&self) -> u64 {
        self.state.depth.load(Ordering::Relaxed)
    }

    pub fn high_water(&self) -> u64 {
        self.state.high_water.load(Ordering::Relaxed)
    }

    pub fn snapshot(&self) -> (u64, usize, u64, u64, u64, u64) {
        (
            self.state.generation.load(Ordering::Acquire),
            self.capacity,
            self.state.depth.load(Ordering::Acquire),
            self.state.high_water.load(Ordering::Acquire),
            self.state.sent.load(Ordering::Acquire),
            self.state.rejected.load(Ordering::Acquire),
        )
    }

    fn record_rejected(&self) {
        self.state.rejected.fetch_add(1, Ordering::Relaxed);
        self.state.generation.fetch_add(1, Ordering::Release);
    }

    fn record_enqueue(&self) {
        self.state.sent.fetch_add(1, Ordering::Relaxed);
        let depth = self.state.depth.fetch_add(1, Ordering::AcqRel) + 1;
        self.state.high_water.fetch_max(depth, Ordering::Relaxed);
        self.state.generation.fetch_add(1, Ordering::Release);
    }

    fn record_dequeue(&self) {
        let _ = self
            .state
            .depth
            .fetch_update(Ordering::AcqRel, Ordering::Relaxed, |depth| {
                Some(depth.saturating_sub(1))
            });
        self.state.generation.fetch_add(1, Ordering::Release);
    }
}

#[derive(Debug)]
pub struct BoundedSender<T> {
    tx: mpsc::Sender<T>,
    policy: ChannelFullPolicy,
    metrics: ChannelMetrics,
}

impl<T> Clone for BoundedSender<T> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            policy: self.policy,
            metrics: self.metrics.clone(),
        }
    }
}

impl<T> BoundedSender<T> {
    pub async fn send(&self, value: T) -> Result<(), SendError> {
        let result = match self.policy {
            ChannelFullPolicy::Block => {
                self.tx
                    .reserve()
                    .await
                    .map_err(|_| SendError::Closed)
                    .map(|permit| {
                        self.record_enqueue();
                        permit.send(value);
                    })
            }
            ChannelFullPolicy::Reject => self
                .tx
                .try_reserve()
                .map_err(|error| match error {
                    mpsc::error::TrySendError::Full(_) => SendError::Full,
                    mpsc::error::TrySendError::Closed(_) => SendError::Closed,
                })
                .map(|permit| {
                    self.record_enqueue();
                    permit.send(value);
                }),
        };
        match result {
            Ok(()) => Ok(()),
            Err(error) => {
                if error == SendError::Full {
                    self.metrics.record_rejected();
                }
                Err(error)
            }
        }
    }

    pub fn metrics(&self) -> ChannelMetrics {
        self.metrics.clone()
    }

    fn record_enqueue(&self) {
        self.metrics.record_enqueue();
    }
}

#[derive(Debug)]
pub struct BoundedReceiver<T> {
    rx: mpsc::Receiver<T>,
    metrics: ChannelMetrics,
}

impl<T> BoundedReceiver<T> {
    pub async fn recv(&mut self) -> Option<T> {
        let value = self.rx.recv().await;
        if value.is_some() {
            self.metrics.record_dequeue();
        }
        value
    }

    pub fn try_recv(&mut self) -> Result<T, mpsc::error::TryRecvError> {
        let value = self.rx.try_recv()?;
        self.metrics.record_dequeue();
        Ok(value)
    }
}

pub fn channel<T>(
    capacity: usize,
    policy: ChannelFullPolicy,
) -> (BoundedSender<T>, BoundedReceiver<T>) {
    assert!(capacity > 0, "bounded channel capacity must be non-zero");
    let (tx, rx) = mpsc::channel(capacity);
    let metrics = ChannelMetrics::new(capacity);
    (
        BoundedSender {
            tx,
            policy,
            metrics: metrics.clone(),
        },
        BoundedReceiver { rx, metrics },
    )
}
