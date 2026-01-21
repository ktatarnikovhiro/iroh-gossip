//! Metrics for iroh-gossip
#[cfg(not(feature = "metrics"))]
use std::marker::PhantomData;

use iroh_metrics::{
    core::{Counter, Metric},
    struct_iterable::Iterable,
};
#[cfg(feature = "metrics")]
use prometheus_client::{encoding::EncodeLabelSet, metrics::family::Family};

/// Enum of metrics for the module
#[allow(missing_docs)]
#[derive(Debug, Clone, Iterable)]
pub struct Metrics {
    pub msgs_ctrl_sent: Counter,
    pub msgs_ctrl_recv: Counter,
    pub msgs_data_sent: Counter,
    pub msgs_data_recv: Counter,
    pub msgs_data_sent_size: Counter,
    pub msgs_data_recv_size: Counter,
    pub msgs_ctrl_sent_size: Counter,
    pub msgs_ctrl_recv_size: Counter,
    pub neighbor_up: Counter,
    pub neighbor_down: Counter,
    pub actor_tick_main: Counter,
    pub actor_tick_rx: Counter,
    pub actor_tick_endpoint: Counter,
    pub actor_tick_dialer: Counter,
    pub actor_tick_dialer_success: Counter,
    pub actor_tick_dialer_failure: Counter,
    pub actor_tick_in_event_rx: Counter,
    pub actor_tick_timers: Counter,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            msgs_ctrl_sent: Counter::new("Number of control messages sent"),
            msgs_ctrl_recv: Counter::new("Number of control messages received"),
            msgs_data_sent: Counter::new("Number of data messages sent"),
            msgs_data_recv: Counter::new("Number of data messages received"),
            msgs_data_sent_size: Counter::new("Total size of all data messages sent"),
            msgs_data_recv_size: Counter::new("Total size of all data messages received"),
            msgs_ctrl_sent_size: Counter::new("Total size of all control messages sent"),
            msgs_ctrl_recv_size: Counter::new("Total size of all control messages received"),
            neighbor_up: Counter::new("Number of times we connected to a peer"),
            neighbor_down: Counter::new("Number of times we disconnected from a peer"),
            actor_tick_main: Counter::new("Number of times the main actor loop ticked"),
            actor_tick_rx: Counter::new("Number of times the actor ticked for a message received"),
            actor_tick_endpoint: Counter::new(
                "Number of times the actor ticked for an endpoint event",
            ),
            actor_tick_dialer: Counter::new("Number of times the actor ticked for a dialer event"),
            actor_tick_dialer_success: Counter::new(
                "Number of times the actor ticked for a successful dialer event",
            ),
            actor_tick_dialer_failure: Counter::new(
                "Number of times the actor ticked for a failed dialer event",
            ),
            actor_tick_in_event_rx: Counter::new(
                "Number of times the actor ticked for an incoming event",
            ),
            actor_tick_timers: Counter::new("Number of times the actor ticked for a timer event"),
        }
    }
}

impl Metric for Metrics {
    fn name() -> &'static str {
        "gossip"
    }
}

/// A wrapper for a Prometheus gauge family, with a description for what the gauge tracks.
#[derive(Debug, Clone)]
pub struct GaugeFamily<L>
where
    L: Clone,
{
    /// The actual prometheus family.
    #[cfg(feature = "metrics")]
    pub family: Family<L, prometheus_client::metrics::gauge::Gauge>,
    /// The fake data
    #[cfg(not(feature = "metrics"))]
    pub _phantom: PhantomData<L>,

    /// What this gauge tracks.
    pub description: &'static str,
}
impl<L> GaugeFamily<L>
where
    L: Clone + std::hash::Hash + std::cmp::Eq,
{
    /// Constructs a new gauge, based on the given `description`.
    pub fn new(description: &'static str) -> Self {
        Self {
            #[cfg(feature = "metrics")]
            family: Family::<L, prometheus_client::metrics::gauge::Gauge>::new_with_constructor(
                || prometheus_client::metrics::gauge::Gauge::default(),
            ),
            #[cfg(not(feature = "metrics"))]
            _phantom: PhantomData::default(),
            description,
        }
    }

    /// Increase the [`Gauge`] by 1, returning the previous value.
    pub fn inc(&self, _labels: &L) -> i64 {
        #[cfg(feature = "metrics")]
        {
            self.family.get_or_create(_labels).inc()
        }
        #[cfg(not(feature = "metrics"))]
        0
    }
    /// Increase the [`Gauge`] by `i64`, returning the previous value.
    #[cfg(feature = "metrics")]
    pub fn inc_by(&self, labels: &L, v: i64) -> i64 {
        self.family.get_or_create(labels).inc_by(v)
    }
    /// Increase the [`Gauge`] by `i64`, returning the previous value.
    #[cfg(not(feature = "metrics"))]
    pub fn inc_by(&self, _labels: &L, _v: u64) -> u64 {
        0
    }

    /// Decrease the [`Gauge`] by 1, returning the previous value.
    pub fn dec(&self, _labels: &L) -> i64 {
        #[cfg(feature = "metrics")]
        {
            self.family.get_or_create(_labels).dec()
        }
        #[cfg(not(feature = "metrics"))]
        0
    }
    /// Decrease the [`Gauge`] by `i64`, returning the previous value.
    #[cfg(feature = "metrics")]
    pub fn dec_by(&self, labels: &L, v: i64) -> i64 {
        self.family.get_or_create(labels).dec_by(v)
    }
    /// Decrease the [`Gauge`] by `i64`, returning the previous value.
    #[cfg(not(feature = "metrics"))]
    pub fn dec_by(&self, _labels: &L, _v: u64) -> u64 {
        0
    }

    /// Set the [`Gauge`] value.
    #[cfg(feature = "metrics")]
    pub fn set(&self, labels: &L, v: i64) -> i64 {
        self.family
            .get_or_create(labels)
            .inner()
            .store(v, std::sync::atomic::Ordering::Relaxed);
        v
    }
    /// Set the [`Gauge`] value.
    #[cfg(not(feature = "metrics"))]
    pub fn set(&self, _labels: &L, _v: i64) -> i64 {
        0
    }

    /// Get the [`Gauge`] value.
    #[cfg(feature = "metrics")]
    pub fn get(&self, labels: &L) -> i64 {
        self.family
            .get_or_create(labels)
            .inner()
            .load(std::sync::atomic::Ordering::Relaxed)
    }
    /// Get the [`Gauge`] value.
    #[cfg(not(feature = "metrics"))]
    pub fn get(&self, _labels: &L) -> i64 {
        0
    }
}

#[cfg(feature = "metrics")]
/// Labels for peer metrics.
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {
    /// The peer ID associated with these metrics.
    pub peer_id: String,
}

#[cfg(not(feature = "metrics"))]
/// Labels for peer metrics.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Labels {
    /// The peer ID associated with these metrics.
    pub peer_id: String,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Iterable)]
pub struct PeerMetrics {
    pub queue_size: GaugeFamily<Labels>,
}

impl Default for PeerMetrics {
    fn default() -> Self {
        PeerMetrics {
            queue_size: GaugeFamily::new(""),
        }
    }
}

impl Metric for PeerMetrics {
    #[cfg(feature = "metrics")]
    fn new(registry: &mut prometheus_client::registry::Registry) -> Self {
        let sub_registry = registry.sub_registry_with_prefix(Self::name());

        let this = Self::default();
        for (metric, counter) in this.iter() {
            if let Some(counter) = counter.downcast_ref::<Counter>() {
                sub_registry.register(metric, counter.description, counter.counter.clone());
            }
            if let Some(family) = counter.downcast_ref::<GaugeFamily<Labels>>() {
                sub_registry.register(metric, family.description, family.family.clone());
            }
        }
        this
    }
    #[cfg(not(feature = "metrics"))]
    fn new(_: &mut ()) -> Self {
        Self::default()
    }

    fn name() -> &'static str {
        "gossip_peer"
    }
}
