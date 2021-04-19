// https://docs.gitlab.com/ee/api/api_resources.html#project-resources

pub enum Access {
    NoAccess = 0,
    MinimalAccess = 5,
    Guest = 10,
    Reporter = 20,
    Developer = 30,
    Maintainer = 40,
    Owner = 50
}