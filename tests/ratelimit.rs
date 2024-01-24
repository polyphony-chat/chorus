use chorus::errors::ChorusError;

mod common;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn hit_ratelimit() {
    let mut bundle = common::setup().await;
    let mut _count = 0;
    let guild = bundle.guild.read().unwrap().clone();
    while _count < 1000 {
        _count += 1;
        match guild.channels(&mut bundle.user).await {
            Err(ChorusError::RateLimited { bucket: _ }) => {
                return;
            }
            Err(_) => panic!("Hit different rate limit"),
            _ => continue,
        }
    }
    common::teardown(bundle).await;
    panic!("Ratelimit never triggered");
}
