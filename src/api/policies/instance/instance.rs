pub mod instance {
    use std::fmt;

    use reqwest::Client;
    use serde_json::from_str;

    use crate::{api::schemas::schemas::InstancePoliciesSchema, instance::Instance};

    #[derive(Debug, PartialEq, Eq)]
    pub struct InstancePoliciesError {
        pub message: String,
    }

    impl InstancePoliciesError {
        fn new(message: String) -> Self {
            InstancePoliciesError { message }
        }
    }

    impl fmt::Display for InstancePoliciesError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl std::error::Error for InstancePoliciesError {}
    impl Instance {
        /// Gets the instance policies schema.
        /// # Errors
        /// * [`InstancePoliciesError`] - If the request fails.
        pub async fn instance_policies_schema(
            &self,
        ) -> Result<InstancePoliciesSchema, InstancePoliciesError> {
            let client = Client::new();
            let endpoint_url = self.urls.get_api().to_string() + "/policies/instance/";
            let request = match client.get(&endpoint_url).send().await {
                Ok(result) => result,
                Err(e) => {
                    return Err(InstancePoliciesError {
                        message: format!(
                            "An error occured while trying to GET from {}: {}",
                            endpoint_url, e
                        ),
                    });
                }
            };

            if request.status().as_str().chars().next().unwrap() != '2' {
                return Err(InstancePoliciesError {
                    message: format!(
                        "Received the following error code while requesting from the route: {}",
                        request.status().as_str()
                    ),
                });
            }

            let body = request.text().await.unwrap();
            let instance_policies_schema: InstancePoliciesSchema = from_str(&body).unwrap();
            Ok(instance_policies_schema)
        }
    }
}

#[cfg(test)]
mod instance_policies_schema_test {
    use crate::{instance::Instance, limit::LimitedRequester, URLBundle};

    #[tokio::test]
    async fn generate_instance_policies_schema() {
        let urls = URLBundle::new(
            "http://localhost:3001/api".to_string(),
            "http://localhost:3001".to_string(),
            "http://localhost:3001".to_string(),
        );
        let limited_requester = LimitedRequester::new(urls.get_api().to_string()).await;
        let test_instance = Instance::new(urls.clone(), limited_requester).unwrap();

        let schema = test_instance.instance_policies_schema().await.unwrap();
        println!("{}", schema);
    }
}
