// https://docs.gitlab.com/ee/api/api_resources.html#project-resources

pub struct Rest {
    pub private_token: &'static str,
    pub scope: &'static str,
    pub hostname: &'static str,
}

impl Rest {
    pub fn new(private_token: &'static str, scope: &'static str, hostname: &'static str) -> Self {
        Self {
            private_token,
            scope,
            hostname,
        }
    }
}
