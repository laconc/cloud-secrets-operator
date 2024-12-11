use cloudsecrets::CloudSecret;
use kube::Client;

#[tokio::main]
async fn main() {
    let client = Client::try_default()
        .await
        .expect("Failed to create Kubernetes client");
}
