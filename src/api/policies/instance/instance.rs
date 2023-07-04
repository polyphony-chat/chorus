use reqwest::Client;
use serde_json::from_str;

use crate::errors::{ChorusError, ChorusResult};
use crate::instance::Instance;
use crate::types::GeneralConfiguration;

impl Instance {
    /// Gets the instance policies schema.
    /// # Errors
    /// [`ChorusLibError`] - If the request fails.
    pub async fn general_configuration_schema(&self) -> ChorusResult<GeneralConfiguration> {
        let endpoint_url = self.urls.api.clone() + "/policies/instance/";
        let request = match self.client.get(&endpoint_url).send().await {
            Ok(result) => result,
            Err(e) => {
                return Err(ChorusError::RequestErrorError {
                    url: endpoint_url,
                    error: e.to_string(),
                });
            }
        };

        if !request.status().as_str().starts_with('2') {
            return Err(ChorusError::ReceivedErrorCodeError {
                error_code: request.status().to_string(),
            });
        }

        let body = request.text().await.unwrap();
        Ok(from_str::<GeneralConfiguration>(&body).unwrap())
    }
}
