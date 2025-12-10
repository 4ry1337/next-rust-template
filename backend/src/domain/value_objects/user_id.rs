use std::ops::Deref;

use uuid::Uuid;

#[derive(Copy, Clone, Debug)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl Deref for UserId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
