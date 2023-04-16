use crate::api::schemas::schemas::InstancePoliciesSchema;
use crate::gateway::Gateway;
use crate::limit::LimitedRequester;
use crate::URLBundle;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Instance {
    main_url: String,
    urls: URLBundle,
    instance_info: InstancePoliciesSchema,
    requester: LimitedRequester,
}
