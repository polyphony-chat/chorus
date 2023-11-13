use chorus::instance::Instance;
use chorus::UrlBundle;

#[tokio::main]
async fn main() {
    let bundle = UrlBundle::new(
        "https://example.com/api".to_string(),
        "wss://example.com/".to_string(),
        "https://example.com/cdn".to_string(),
    );
    let instance = Instance::new(bundle, true)
        .await
        .expect("Failed to connect to the Spacebar server");
    dbg!(instance.instance_info);
    dbg!(instance.limits_information);
}
