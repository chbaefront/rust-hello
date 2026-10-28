use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

// DeriveIden 트레이트 사용
#[derive(DeriveIden)]
enum Test {
    Table,
    Id,
    Name,
}
// Table 은 테이블 자체를 나타내고, 나머지 항목은 각 컬럼을 나타냄

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261028_191527_test"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();
        // manager 객체를 이용해 새로운 테이블 정의
        _manager
            .create_table(
                Table::create()
                    .table((Alias::new("local"), Test::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Test::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Test::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();
        _manager
            .drop_table(
                Table::drop()
                    .table((Alias::new("local"), Test::Table))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
