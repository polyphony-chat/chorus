use reqwest::Client;
use serde_json::from_str;

use crate::errors::ChorusLibError;
use crate::instance::Instance;
use crate::types::GeneralConfiguration;

impl Instance {
    /**
    Gets the instance policies schema.
    # Errors
    [`ChorusLibError`] - If the request fails.
    */
    pub async fn general_configuration_schema(
        &self,
    ) -> Result<GeneralConfiguration, ChorusLibError> {
        let client = Client::new();
        let endpoint_url = self.urls.get_api().to_string() + "/policies/instance/";
        let request = match client.get(&endpoint_url).send().await {
            Ok(result) => result,
            Err(e) => {
                return Err(ChorusLibError::RequestErrorError {
                    url: endpoint_url,
                    error: e.to_string(),
                });
            }
        };

        if !request.status().as_str().starts_with('2') {
            return Err(ChorusLibError::ReceivedErrorCodeError {
                error_code: request.status().to_string(),
            });
        }

        let body = request.text().await.unwrap();
        let instance_policies_schema: GeneralConfiguration = from_str(&body).unwrap();
        Ok(instance_policies_schema)
    }
}
