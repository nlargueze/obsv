//! OTLP metrics

/// Metric kind
pub enum MetricKind {
    /// A value that accumulates over time – you can think of this like an odometer on a car; it only ever goes up.
    Counter,
    /// Same as the Counter, but is collected once for each export. Could be used if you don’t have access to the continuous increments, but only to the aggregated value.
    AsyncCounter,
    /// A value that accumulates over time, but can also go down again. An example could be a queue length, it will increase and decrease with the number of work items in the queue.
    UpDownCounter,
    /// Same as the UpDownCounter, but is collected once for each export. Could be used if you don’t have access to the continuous changes, but only to the aggregated value (e.g., current queue size).
    AsyncUpDownCounter,
    /// Measures a current value at the time it is read. An example would be the fuel gauge in a vehicle. Gauges are always asynchronous.
    Gauge,
    /// A histogram is a client-side aggregation of values, e.g., request latencies. A histogram is likely a good choice if you have a lot of values, and are not interested in every individual value, but a statistic about these values (e.g., How many requests take fewer than 1s?)
    Histogram,
}
