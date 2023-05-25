use reqwest::Client;
use serde_json::from_str;

use crate::errors::InstanceServerError;
use crate::instance::Instance;
use crate::types::GeneralConfiguration;

impl Instance {
    /**
    Gets the instance policies schema.
    # Errors
    [`InstanceServerError`] - If the request fails.
    */
    pub async fn general_configuration_schema(
        &self,
    ) -> Result<GeneralConfiguration, InstanceServerError> {
        let client = Client::new();
        let endpoint_url = self.urls.get_api().to_string() + "/policies/instance/";
        let request = match client.get(&endpoint_url).send().await {
            Ok(result) => result,
            Err(e) => {
                return Err(InstanceServerError::RequestErrorError {
                    url: endpoint_url,
                    error: e.to_string(),
                });
            }
        };

        if !request.status().as_str().starts_with('2') {
            return Err(InstanceServerError::ReceivedErrorCodeError {
                error_code: request.status().to_string(),
            });
        }

        let body = request.text().await.unwrap();
        let instance_policies_schema: GeneralConfiguration = from_str(&body).unwrap();
        Ok(instance_policies_schema)
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
        let limited_requester = LimitedRequester::new().await;
        let test_instance = Instance::new(urls.clone()).await.unwrap();

        let _schema = test_instance.general_configuration_schema().await.unwrap();
    }
}
