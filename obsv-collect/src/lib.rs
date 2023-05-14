//! Collector
//!
//! The collector is the server responsible for receiving, processing, and exporting metrics, traces, logs, and other data.
//!
//! The collector provides:
//! - a HTTP API, which mimics the OpenTelemetry API
//! - a gRPC API, which mimics the OpenTelemetry API
//!  
//! # Features
//!
//! - **http**: HTTP server and client

use expt::Exporter;
use obsv_core::Data;
use proc::Processor;
use recv::Receiver;

pub mod expt;
pub mod proc;
pub mod recv;

// Server
#[derive(Default)]
pub struct Server {
    /// Receivers
    receivers: Vec<Box<dyn Receiver>>,
    /// Processors
    processors: Vec<Box<dyn Processor>>,
    /// Exporters
    exporters: Vec<Box<dyn Exporter>>,
}

impl Server {
    /// Instantiates a new server
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a receiver
    pub fn receiver(mut self, receiver: impl Receiver + 'static) -> Self {
        self.receivers.push(Box::new(receiver));
        self
    }

    /// Adds a processor
    pub fn processor(mut self, processor: impl Processor + 'static) -> Self {
        self.processors.push(Box::new(processor));
        self
    }

    /// Adds an exporter
    pub fn exporter(mut self, exporter: impl Exporter + 'static) -> Self {
        self.exporters.push(Box::new(exporter));
        self
    }
}

impl Server {
    /// Starts the server
    pub async fn start(self) {
        let mut tasks = vec![];

        let (recv_tx, mut recv_rx) = tokio::sync::mpsc::unbounded_channel::<Data>();
        for receiver in self.receivers {
            // NB: each receiver runs in its own task
            let task = tokio::spawn({
                let tx = recv_tx.clone();
                async move {
                    receiver.start(tx).await;
                }
            });
            tasks.push(task);
        }

        // processing
        // there is a unique task to process all the data received from the receiver
        let (proc_tx, _proc_rx) = tokio::sync::broadcast::channel(100);
        let task = tokio::spawn({
            let proc_tx = proc_tx.clone();
            async move {
                // => we receive data from the receiver channel
                loop {
                    let mut data = match recv_rx.recv().await {
                        Some(d) => {
                            log::trace!("Processing data");
                            d
                        }
                        None => {
                            log::error!("Closed receiver channel");
                            panic!("Closed receiver channel")
                        }
                    };
                    for processor in &self.processors {
                        data = processor.process(data).await;
                    }
                    match proc_tx.send(data) {
                        Ok(_ok) => {}
                        Err(_err) => {
                            log::error!("Closed processing channel");
                            panic!("Closed processing channel")
                        }
                    };
                }
            }
        });
        tasks.push(task);

        // exporting
        // each exporter runs in its own task
        for exporter in self.exporters {
            let mut proc_rx = proc_tx.subscribe();
            let task = tokio::spawn({
                let data = match proc_rx.recv().await {
                    Ok(d) => d,
                    Err(_err) => {
                        log::error!("Closed processing channel");
                        panic!("Closed processing channel")
                    }
                };
                async move {
                    exporter.export(data).await;
                }
            });
            tasks.push(task);
        }

        // wait for all tasks
        for task in tasks {
            task.await.unwrap();
        }
    }
}
