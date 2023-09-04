//! Reporter

use obsv_api::ApiServer;

#[tokio::main]
async fn main() {
    let mut tasks = vec![];

    let task = tokio::spawn(async move {
        let api_server = ApiServer::new("0.0.0.0:5001".parse().unwrap());
        eprintln!("API server listening on :5001 ...");
        api_server.start().await;
    });
    tasks.push(task);

    // join all the tasks
    for task in tasks {
        task.await.unwrap();
    }
}
