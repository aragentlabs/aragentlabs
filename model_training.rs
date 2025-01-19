use tokio::task;

async fn train_model_on_node(node_id: String) -> Result<(), String> {
    // Simulate model training task
    println!("Training model on node {}", node_id);
    // In a real-world scenario, you could call a machine learning framework here
    Ok(())
}

#[tokio::main]
async fn main() {
    let nodes = vec!["node_001", "node_002", "node_003"];

    let mut tasks = vec![];

    for node in nodes {
        let node_id = node.to_string();
        tasks.push(tokio::spawn(async move {
            if let Err(e) = train_model_on_node(node_id).await {
                eprintln!("Error on node {}: {}", node_id, e);
            }
        }));
    }

    // Await all tasks (model training jobs) to finish
    for task in tasks {
        task.await.unwrap();
    }
}
