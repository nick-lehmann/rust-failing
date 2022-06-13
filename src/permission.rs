use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Resource {
    User,
}

impl Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resource::User => write!(f, "user"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    Read,
    Create,
    Update,
    Delete,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Read => write!(f, "read"),
            Action::Create => write!(f, "create"),
            Action::Update => write!(f, "update"),
            Action::Delete => write!(f, "delete"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Permission {
    pub resource: Resource,
    pub action: Action,
    pub identifier: Option<String>,
}

impl Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}.{}", self.resource, self.action))?;

        if let Some(identifer) = &self.identifier {
            f.write_fmt(format_args!("(id = {})", identifer))?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct PermissionError {
    missing_permission: Permission,
}

impl PermissionError {
    pub fn new(resource: Resource, action: Action, identifier: Option<String>) -> Self {
        Self {
            missing_permission: Permission {
                resource,
                action,
                identifier,
            },
        }
    }
}

impl Display for PermissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Missing permission: {}", self.missing_permission)
    }
}

impl std::error::Error for PermissionError {}
