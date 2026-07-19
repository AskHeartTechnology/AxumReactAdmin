#[tokio::main]
async fn main() {
    ara::run("0.0.0.0:8088").await.unwrap();
}
