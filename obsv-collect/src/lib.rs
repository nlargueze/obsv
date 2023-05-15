//! Collector
//!
//! The collector is the service responsible for receiving, processing, and exporting metrics, traces, logs, and other data.
//!  
//! # Features
//!
//! - **http**: HTTP server
//! - **grpc**: GRPC server

use expt::Exporter;
use obsv_core::Data;
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
        let (proc_tx, _proc_rx) = tokio::sync::broadcast::channel::<Vec<Data>>(100);
        tasks.push(tokio::spawn({
            let proc_tx = proc_tx.clone();
            async move {
                // => we receive data from the receiver channel
                loop {
                    let data = match recv_rx.recv().await {
                        Some(d) => {
                            log::trace!("received data");
                            d
                        }
                        None => {
                            log::error!("closed receiver channel");
                            panic!("closed receiver channel")
                        }
                    };
                    let mut processed_data = vec![data];
                    'loop_processors: for processor in &mut self.processors {
                        processed_data = match processor.process(processed_data).await {
                            Some(d) => d,
                            None => {
                                // NB: nothing is returned, so we skip the processing chain
                                processed_data = vec![];
                                break 'loop_processors;
                            }
                        }
                    }
                    if !processed_data.is_empty() {
                        match proc_tx.send(processed_data) {
                            Ok(_ok) => {}
                            Err(_err) => {
                                log::error!("closed processing channel");
                                panic!("closed processing channel")
                            }
                        };
                    }
                }
            }
        }));

        // NB: each exporter runs in its own task
        for exporter in self.exporters {
            let proc_tx = proc_tx.clone();
            tasks.push(tokio::spawn(async move {
                let mut proc_rx = proc_tx.subscribe();
                loop {
                    let data = match proc_rx.recv().await {
                        Ok(d) => {
                            log::trace!("processed data");
                            d
                        }
                        Err(_err) => {
                            log::error!("closed processing channel");
                            panic!("closed processing channel")
                        }
                    };
                    exporter.export(data).await;
                }
            }));
        }

        // wait for all tasks
        for task in tasks {
            task.await.unwrap();
        }
    }
}
