//! Collector
//!
//! The collector is the service responsible for receiving, processing, and exporting metrics, traces, logs, and other data.
//!  
//! # Features
//!
//! - **http**: HTTP server
//! - **grpc**: GRPC server

use expt::Exporter;
use proc::Processor;
use recv::Receiver;

pub mod expt;
pub mod proc;
pub mod recv;

// Collector service
#[derive(Default)]
pub struct CollService {
    /// Receivers
    receivers: Vec<Box<dyn Receiver>>,
    /// Processors
    processors: Vec<Box<dyn Processor>>,
    /// Exporters
    exporters: Vec<Box<dyn Exporter>>,
}

impl CollService {
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

/// A piece of collection data
#[derive(Debug, Clone)]
pub enum Data {
    ToDo,
}

impl Default for Data {
    fn default() -> Self {
        Self::ToDo
    }
}

impl CollService {
    /// Starts the service
    pub async fn start(mut self) {
        let mut tasks = vec![];

        // NB: each receiver runs in its own task, which sends each received data for processing
        let (recv_tx, mut recv_rx) = tokio::sync::mpsc::unbounded_channel::<Data>();
        for receiver in self.receivers {
            tasks.push(tokio::spawn({
                let tx = recv_tx.clone();
                async move {
                    receiver.start(tx).await;
                }
            }));
        }

        // NB: there is a unique task waiting to receive a signal
        // once received, a task is spawned to process the data and then export it in its own thread
        tasks.push(tokio::spawn({
            async move {
                // => we receive data from the receiver channel
                loop {
                    let data_recv = match recv_rx.recv().await {
                        Some(d) => {
                            log::trace!("received data");
                            d
                        }
                        None => {
                            log::error!("closed receiver channel");
                            panic!("closed receiver channel")
                        }
                    };

                    let mut processors = self.processors.clone();
                    let exporters = self.exporters.clone();
                    tokio::spawn(async move {
                        let mut data = vec![data_recv];
                        for processor in &mut processors {
                            data = match processor.process(data).await {
                                Some(d) => d,
                                None => {
                                    // NB: nothing is returned, so we stop the processing chain
                                    return;
                                }
                            }
                        }

                        // NB: each exporter runs in its own task to export in parallel
                        for exporter in exporters {
                            tokio::spawn({
                                let data = data.clone();
                                async move {
                                    exporter.export(&data).await;
                                }
                            });
                        }
                    });
                }
            }
        }));

        // wait for all top-level tasks
        for task in tasks {
            task.await.unwrap();
        }
    }
}
