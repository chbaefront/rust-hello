# rust-axum-orm

Axum 애플리케이션에서 PostgreSQL과 SeaORM을 연결하고, 엔티티 기반 조회와 마이그레이션을 학습하는 프로젝트입니다. 순수 HTTP 기능은 [`rust-axum`](../rust-axum/README.md)으로 분리했습니다.

## 구성

```text
rust-axum-orm/
├── src/
│   ├── entities/       # DB 스키마에서 생성한 SeaORM 엔티티
│   ├── routes/users.rs # 엔티티 조회 API
│   ├── lib.rs          # DB 연결을 담은 AppState
│   └── main.rs         # DB 연결 및 Axum 서버 시작
└── migration/          # 독립적인 SeaORM Migration 크레이트
```

## 사전 준비

PostgreSQL을 준비하고 데이터베이스와 `local` 스키마를 생성합니다. Docker를 사용한다면 다음처럼 시작할 수 있습니다.

```bash
docker volume create pgdata
docker run -d -p 5432:5432 --name local-postgres \
  -e TZ=Asia/Seoul \
  -e POSTGRES_PASSWORD=your_password \
  -v pgdata:/var/lib/postgresql \
  postgres
```

PostgreSQL 18 이상 공식 이미지에서는 데이터 디렉터리 구조가 이전 버전과 다를 수 있으므로 볼륨 경로를 이미지 문서에서 확인합니다.

```sql
CREATE DATABASE rust_db;
CREATE SCHEMA IF NOT EXISTS local;
```

`.env.example`을 참고하여 워크스페이스 루트 또는 이 프로젝트 디렉터리에 `.env`를 만듭니다.

```dotenv
DATABASE_URL=postgres://postgres:your_password@localhost:5432/rust_db
```

`.env`에는 비밀번호가 포함되므로 Git에서 제외됩니다.

## 실행

워크스페이스 루트에서 다음 명령을 실행하면 루트의 `.env`를 읽습니다.

```bash
cargo run -p rust-axum-orm
```

서버는 `http://localhost:3001`에서 실행됩니다.

| Method | 경로 | 설명 |
| --- | --- | --- |
| GET | `/` | 서버 상태 확인 |
| GET | `/users` | 모든 사용자 조회 |
| GET | `/users/{user_id}` | ID로 사용자 한 명 조회 |

## SeaORM 핵심 개념

SeaORM은 SQLx 위에서 동작하는 비동기 ORM입니다. 이 프로젝트는 PostgreSQL 드라이버, Tokio 런타임, derive 매크로와 직렬화 기능을 활성화합니다.

```toml
sea-orm = { version = "2", features = [
  "sqlx-postgres",
  "runtime-tokio-native-tls",
  "macros",
  "serde"
] }
```

서버 시작 시 한 번 연결한 `DatabaseConnection`을 `AppState`에 담고 핸들러에서 `State<AppState>`로 추출합니다. 연결 핸들은 복제해도 같은 풀을 공유합니다.

```rust
let db = Database::connect(database_url).await?;
let state = AppState { db };

let users = Entity::find().all(&state.db).await?;
let user = Entity::find()
    .filter(users::Column::Id.eq(user_id))
    .one(&state.db)
    .await?;
```

- `all`은 `Vec<Model>`을 반환합니다.
- `one`은 레코드가 없을 수 있으므로 `Option<Model>`을 반환합니다.
- `EntityTrait`, `QueryFilter`, `ColumnTrait`을 가져와야 쿼리 메서드를 사용할 수 있습니다.

## SeaQuery와 식별자

SeaORM의 SQL 생성 기반은 SeaQuery입니다. `DeriveIden`을 사용하면 테이블과 컬럼 식별자를 문자열 대신 타입으로 표현할 수 있습니다.

```rust
#[derive(DeriveIden)]
enum Test {
    Table,
    Id,
    Name,
}
```

레거시 이름이나 대소문자를 그대로 사용해야 할 때는 `#[sea_orm(iden = "...")]`로 실제 식별자를 지정할 수 있습니다.

## 마이그레이션

`migration`은 워크스페이스에 포함된 별도 크레이트입니다. 현재 마이그레이션은 `local.test` 테이블의 `id`, `name` 컬럼을 생성하고 롤백 시 테이블을 제거합니다.

```bash
# 최신 마이그레이션 적용
cargo run -p migration -- up

# 한 단계 롤백
cargo run -p migration -- down

# 상태 확인
cargo run -p migration -- status
```

새 마이그레이션은 SeaORM CLI로 생성할 수 있습니다.

```bash
cargo install sea-orm-cli
sea-orm-cli migrate generate create_users
```

## 엔티티 생성

기존 DB 스키마에서 엔티티를 다시 생성할 때는 출력 경로를 이 프로젝트의 `src/entities`로 지정합니다.

```bash
sea-orm-cli generate entity \
  -u postgres://postgres:your_password@localhost:5432/rust_db \
  -s local \
  -o rust-axum-orm/src/entities
```

생성된 엔티티를 JSON 응답에 포함하려면 모델에 `Serialize`가 구현되어 있어야 합니다. 이 프로젝트의 `users::Model`에는 이를 추가했습니다.

## 상세 학습 노트

- [SeaORM과 PostgreSQL](docs/orm.md)
- [SeaQuery와 CRUD](docs/sea-query.md)
