use chorus::instance::Instance;
use chorus::types::LoginSchema;
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
    // Assume, you already have an account created on this instance. Registering an account works
    // the same way, but you'd use the Register-specific Structs and methods instead.
    let login_schema = LoginSchema {
        login: "user@example.com".to_string(),
        password: "Correct-Horse-Battery-Staple".to_string(),
        ..Default::default()
    };
    // Each user connects to the Gateway. The Gateway connection lives on a seperate thread. Depending on
    // the runtime feature you choose, this can potentially take advantage of all of your computers' threads.
    let user = instance
        .login_account(login_schema)
        .await
        .expect("An error occurred during the login process");
    dbg!(user.belongs_to);
    dbg!(&user.object.read().unwrap().username);
}
