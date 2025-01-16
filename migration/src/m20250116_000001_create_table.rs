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
                    // .if_not_exists()
                    .col(pk_uuid(Users::Id))
                    .col(string_null(Users::Email).unique_key())
                    .col(string_null(Users::OauthId).unique_key())
                    .col(string(Users::Username).default("default user"))
                    .col(string(Users::Avatar).default("default avatar"))
                    .col(string_null(Users::Password))
                    .col(timestamp(Users::CreatedAt))
                    .col(timestamp(Users::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Authors::Table)
                    .col(pk_uuid(Authors::Id))
                    .col(string(Authors::Name))
                    .col(string_null(Authors::SpaceUrl))
                    .col(timestamp(Authors::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .col(pk_uuid(Categories::Id))
                    .col(string(Categories::Name).unique_key())
                    .col(timestamp(Categories::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Topics::Table)
                    .col(pk_uuid(Topics::Id))
                    .col(string(Topics::Name).unique_key())
                    .col(timestamp(Topics::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Videos::Table)
                    .col(pk_uuid(Videos::Id))
                    .col(string(Videos::Name))
                    .col(uuid(Videos::AuthorId))
                    .col(string(Videos::OriginalUrl))
                    .col(uuid(Videos::TopicId))
                    .col(uuid(Videos::CategoryId))
                    .col(timestamp_null(Videos::PostedAt))
                    .col(timestamp(Videos::CreatedAt))
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
