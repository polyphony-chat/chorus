use chorus::instance::Instance;
use chorus::types::LoginSchema;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut instance = Instance::new("https://example.com/")
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
