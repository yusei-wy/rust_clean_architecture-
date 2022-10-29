use anyhow::Context;
use entity::{User, UserName};
use error::AppError;
use repository::{ProvideUserRepository, UserRepository};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct CreateUserCommand {
    name: UserName,
}

pub async fn create_user<T>(ctx: &T, cmd: CreateUserCommand) -> anyhow::Result<User>
where
    T: ProvideUserRepository,
{
    let user = User::new(cmd.name);

    let user_repository = ProvideUserRepository::provide(ctx);
    user_repository
        .save(&user)
        .await
        .with_context(|| AppError::Internal("failed to create_user".to_string()))?;

    Ok(user)
}