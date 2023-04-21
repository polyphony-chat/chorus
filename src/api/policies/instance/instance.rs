pub mod instance {
    use reqwest::Client;
    use serde_json::from_str;

    use crate::errors::InstancePoliciesError;
    use crate::{api::schemas::schemas::InstancePoliciesSchema, instance::Instance};

    impl Instance {
        /**
        Gets the instance policies schema.
        # Errors
        [`InstancePoliciesError`] - If the request fails.
        */
        pub async fn instance_policies_schema(
            &self,
        ) -> Result<InstancePoliciesSchema, InstancePoliciesError> {
            let client = Client::new();
            let endpoint_url = self.urls.get_api().to_string() + "/policies/instance/";
            let request = match client.get(&endpoint_url).send().await {
                Ok(result) => result,
                Err(e) => {
                    return Err(InstancePoliciesError::RequestErrorError {
                        url: endpoint_url,
                        error: e.to_string(),
                    });
                }
            };

            if request.status().as_str().chars().next().unwrap() != '2' {
                return Err(InstancePoliciesError::ReceivedErrorCodeError {
                    error_code: request.status().to_string(),
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
        let test_instance = Instance::new(urls.clone(), limited_requester)
            .await
            .unwrap();

        let schema = test_instance.instance_policies_schema().await.unwrap();
        println!("{}", schema);
    }
}
