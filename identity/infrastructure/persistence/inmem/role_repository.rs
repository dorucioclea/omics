use async_trait::async_trait;

use common::cache::Cache;
use common::error::Error;
use common::infrastructure::cache::InMemCache;
use common::result::Result;

use crate::domain::role::{Role, RoleId, RoleRepository};

pub struct InMemRoleRepository {
    cache: InMemCache<RoleId, Role>,
}

impl InMemRoleRepository {
    pub fn new() -> Self {
        InMemRoleRepository {
            cache: InMemCache::new(),
        }
    }
}

impl Default for InMemRoleRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RoleRepository for InMemRoleRepository {
    async fn find_all(&self) -> Result<Vec<Role>> {
        Ok(self.cache.all().await)
    }

    async fn find_by_id(&self, id: &RoleId) -> Result<Role> {
        self.cache
            .get(id)
            .await
            .ok_or_else(|| Error::not_found("role"))
    }

    async fn save(&self, role: &mut Role) -> Result<()> {
        self.cache.set(role.base().id().clone(), role.clone()).await
    }
}
