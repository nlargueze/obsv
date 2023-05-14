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

type Data = u8;

impl Server {
    /// Starts the server
    pub async fn start(self) {
        let (recv_tx, mut recv_rx) = tokio::sync::mpsc::unbounded_channel::<Data>();

        let mut tasks = vec![];
        for receiver in self.receivers {
            let task_recv = tokio::spawn({
                let tx = recv_tx.clone();
                async move {
                    receiver.start().await;
                }
            });
            tasks.push(task_recv);
        }

        // processors + exporters
        let task_proc = tokio::spawn(async move {
            let data = recv_rx.recv().await;
            // NB: we spawn a new task to process and export each received data
            tokio::spawn(async {
                for processor in self.processors {
                    processor.process().await;
                }

                // NB: we spawn a new tasks for each exporter
                for exporter in self.exporters {
                    tokio::spawn(async {
                        exporter.export(data).await;
                    });
                }
                // send to exporters
                proc_tx.clone().send(0);
            });
        });
        tasks.push(task_proc);

        // exporters
        for exporter in self.exporters {
            tasks.push(tokio::spawn({
                let mut proc_rx = proc_tx.subscribe();
                async move {
                    let data = proc_rx.recv().await.unwrap();
                }
            }));
        }

        //join all tasks
        for task in tasks {
            task.await.unwrap();
        }
    }
}
