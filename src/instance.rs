use crate::api::schemas::schemas::InstancePoliciesSchema;
use crate::gateway::Gateway;
use crate::limit::LimitedRequester;
use crate::URLBundle;

use std::collections::HashMap;

#[derive(Debug)]
/**
The [`Instance`] what you will be using to perform all sorts of actions on the Spacebar server.
 */
pub struct Instance {
    main_url: String,
    urls: URLBundle,
    instance_info: InstancePoliciesSchema,
    requester: LimitedRequester,
    gateway: Gateway,
}

impl Instance {
    pub fn new() {}
}
