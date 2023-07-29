use serde_json::from_str;

use crate::errors::{ChorusError, ChorusResult};
use crate::instance::Instance;
use crate::types::GeneralConfiguration;

impl Instance {
    /// Gets the instance policies schema.
    pub async fn general_configuration_schema(&self) -> ChorusResult<GeneralConfiguration> {
        let endpoint_url = self.urls.api.clone() + "/policies/instance/";
        let request = match self.client.get(&endpoint_url).send().await {
            Ok(result) => result,
            Err(e) => {
                return Err(ChorusError::RequestFailed {
                    url: endpoint_url,
                    error: e,
                });
            }
        };

        if !request.status().as_str().starts_with('2') {
            return Err(ChorusError::ReceivedErrorCode {
                error_code: request.status().as_u16(),
                error: request.text().await.unwrap(),
            });
        }

        let body = request.text().await.unwrap();
        Ok(from_str::<GeneralConfiguration>(&body).unwrap())
    }
}
