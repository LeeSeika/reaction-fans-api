use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(string(Users::Email))
                    .col(string(Users::OauthId))
                    .col(string(Users::Username))
                    .col(string(Users::Avatar))
                    .col(string(Users::Password))
                    .col(timestamp(Users::CreatedAt))
                    .col(timestamp(Users::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    // async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    //     // Replace the sample below with your own migration scripts
    //     todo!();

    //     manager
    //         .drop_table(Table::drop().table(Post::Table).to_owned())
    //         .await
    // }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Email,
    OauthId,
    Username,
    Avatar,
    Password,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Authors {
    Table,
    Id,
    Name,
    SpaceUrl,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Categories {
    Table,
    Id,
    Name,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Topics {
    Table,
    Id,
    Name,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Videos {
    Table,
    Id,
    Name,
    AuthorId,
    OriginalUrl,
    TopicId,
    CategoryId,
    PostedAt,
    CreatedAt,
}
