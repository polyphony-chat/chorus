use chorus::instance::Instance;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let instance = Instance::new("https://example.com/")
        .await
        .expect("Failed to connect to the Spacebar server");
    dbg!(instance.instance_info);
    dbg!(instance.limits_information);
}
