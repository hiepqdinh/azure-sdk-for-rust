use super::*;
use crate::prelude::*;
use crate::resources::user::UserResponse;
use crate::resources::ResourceType;
use crate::{requests, ReadonlyString};
use azure_core::{Context, HttpClient, Pipeline, Request};

/// A client for Cosmos user resources.
#[derive(Debug, Clone)]
pub struct UserClient {
    database_client: DatabaseClient,
    user_name: ReadonlyString,
}

impl UserClient {
    pub(crate) fn new<S: Into<ReadonlyString>>(
        database_client: DatabaseClient,
        user_name: S,
    ) -> Self {
        Self {
            database_client,
            user_name: user_name.into(),
        }
    }

    /// Get a [`CosmosClient`]
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.database_client().cosmos_client()
    }

    /// Get a [`DatabaseClient`]
    pub fn database_client(&self) -> &DatabaseClient {
        &self.database_client
    }

    /// Get the user name
    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    /// Create the user
    pub fn create_user(&self) -> CreateUserBuilder {
        CreateUserBuilder::new(self.clone())
    }

    /// Get the user
    pub fn get_user(&self) -> GetUserBuilder {
        GetUserBuilder::new(self.clone())
    }

    /// Replace the user
    pub async fn replace_user<S: AsRef<str>>(
        &self,
        ctx: Context,
        user_name: S,
        options: ReplaceUserOptions,
    ) -> crate::Result<UserResponse> {
        let mut request = self.prepare_request_with_user_name(http::Method::PUT);

        options.decorate_request(&mut request, user_name.as_ref())?;
        let response = self
            .pipeline()
            .send(ctx.clone().insert(ResourceType::Users), &mut request)
            .await?;

        Ok(UserResponse::try_from(response).await?)
    }

    /// Delete the user
    pub fn delete_user(&self) -> DeleteUserBuilder {
        DeleteUserBuilder::new(self.clone())
    }

    /// List the user's permissions
    pub fn list_permissions(&self) -> requests::ListPermissionsBuilder<'_, '_> {
        requests::ListPermissionsBuilder::new(self)
    }

    /// Convert into a [`PermissionClient`]
    pub fn into_permission_client<S: Into<ReadonlyString>>(
        self,
        permission_name: S,
    ) -> PermissionClient {
        PermissionClient::new(self, permission_name)
    }

    pub(crate) fn prepare_request_with_user_name(&self, method: http::Method) -> Request {
        self.cosmos_client().prepare_request_pipeline(
            &format!(
                "dbs/{}/users/{}",
                self.database_client().database_name(),
                self.user_name()
            ),
            method,
        )
    }

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.cosmos_client().http_client()
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        self.cosmos_client().pipeline()
    }
}