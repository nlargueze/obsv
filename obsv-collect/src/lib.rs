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
        let (recv_tx, mut recv_rx) = tokio::sync::mpsc::unbounded_channel::<Data>();
        for receiver in self.receivers {
            // NB: each receiver runs in its own task
            tokio::spawn({
                let tx = recv_tx.clone();
                async move {
                    receiver.start(tx).await;
                }
            });
        }

        // processing
        // there is a unique task to process all the data
        let (proc_tx, mut proc_rx) = tokio::sync::broadcast::channel(100);
        tokio::spawn({
            let mut data = match recv_rx.recv().await {
                Some(d) => d,
                None => {
                    log::error!("Closed channel");
                    panic!("Closed channel")
                }
            };
            async move {
                log::trace!("Processing data: {:?}", data);
                for processor in &self.processors {
                    data = processor.process(data).await;
                }
                match proc_tx.send(data) {
                    Ok(_ok) => {}
                    Err(_err) => {
                        log::error!("Closed channel");
                        panic!("Closed channel")
                    }
                };
            }
        });

        // exporting
        for exporter in self.exporters {
            // NB: each receiver runs in its own task
            tokio::spawn({
                let data = match proc_rx.recv().await {
                    Ok(d) => d,
                    Err(_err) => {
                        log::error!("Closed channel");
                        panic!("Closed channel")
                    }
                };
                async move {
                    exporter.export(data).await;
                }
            });
        }

        loop {
            std::hint::spin_loop();
        }
    }
}
