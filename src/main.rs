use wakeup_node::start;

#[forbid(unsafe_code)]
#[tokio::main]
async fn main() {
    if let Err(e) = start().await {
        panic!("{:?}", e)
    }
}
