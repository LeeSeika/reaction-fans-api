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
                    .col(
                        timestamp(Users::CreatedAt)
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".to_string())),
                    )
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
                    .col(string_uniq(Authors::OriginalId))
                    .col(string(Authors::Platform))
                    .col(string_null(Authors::SpaceUrl))
                    .col(
                        timestamp(Authors::CreatedAt)
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".to_string())),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .col(pk_uuid(Categories::Id))
                    .col(string(Categories::Name).unique_key())
                    .col(
                        timestamp(Categories::CreatedAt)
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".to_string())),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Topics::Table)
                    .col(pk_uuid(Topics::Id))
                    .col(string(Topics::Name).unique_key())
                    .col(
                        timestamp(Topics::CreatedAt)
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".to_string())),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Videos::Table)
                    .col(pk_uuid(Videos::Id))
                    .col(uuid(Videos::AuthorId))
                    .col(string_null(Videos::OriginalUrl))
                    .col(uuid(Videos::TopicId))
                    .col(uuid(Videos::CategoryId))
                    .col(timestamp(Videos::PublishedAt))
                    .col(string(Videos::Platform))
                    .col(uuid(Videos::MetaId))
                    .col(
                        timestamp(Videos::CreatedAt)
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".to_string())),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(BilibiliMeta::Table)
                    .col(pk_uuid(BilibiliMeta::Id))
                    .col(string_uniq(BilibiliMeta::Bvid))
                    .col(integer_uniq(BilibiliMeta::Aid))
                    .col(string(BilibiliMeta::Iframe))
                    .col(integer(BilibiliMeta::Videos))
                    .col(json(BilibiliMeta::Pages))
                    .col(string(BilibiliMeta::Pic))
                    .col(string(BilibiliMeta::Title))
                    .col(timestamp(BilibiliMeta::Pubdate))
                    .col(integer(BilibiliMeta::Duration))
                    .col(integer(BilibiliMeta::View))
                    .col(integer(BilibiliMeta::Danmaku))
                    .col(integer(BilibiliMeta::Reply))
                    .col(integer(BilibiliMeta::Favorite))
                    .col(integer(BilibiliMeta::Coin))
                    .col(integer(BilibiliMeta::Share))
                    .col(integer(BilibiliMeta::Like))
                    .col(
                        timestamp(BilibiliMeta::CreatedAt)
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".to_string())),
                    )
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
    OriginalId,
    Platform,
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
    AuthorId,
    OriginalUrl,
    TopicId,
    CategoryId,
    PublishedAt,
    Platform,
    MetaId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum BilibiliMeta {
    Table,
    Id,
    Bvid,
    Aid,
    Iframe,
    Videos,
    Pages,
    Pic,
    Title,
    Pubdate,
    Duration,
    View,
    Danmaku,
    Reply,
    Favorite,
    Coin,
    Share,
    Like,
    CreatedAt,
}
