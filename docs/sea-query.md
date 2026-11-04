# sea-query.md

---

### SeaQuery

- SeqORM을 사용해 SQL 명령을 수행하는 방법으로는 쿼리 빌더인 SeaQuery를 사용
- SeaORM에서 모든 SQL을 만들고 실행하는 기능은 SeaQuery를 통해서 이뤄짐

---

- SeaQuery에서 테이블 이름과 컬럼 이름을 다루기 위해 `Iden` 매크로를 주로 사용

```rust
use sea_query::Iden;

#[derive(Iden)]
enum BakeryIden {
    Table, // 'bakery' 테이블
    Id,
    Name,
}
```

- `DeriveIden` 은 `Iden` 매크로 구현을 간소화
```rust
#[derive(DeriveIden)]
enum Test {
    Table,
    Id,
    Name,
}

// 열거형은 PascalCase 로 작성
```
DeriveIden 매크로 덕분에 열거형 변수들의 이름이 id, name, employee_name와 같이 자동으로 변환

```rust
use sea_orm::DeriveIden; // SeaORM에서 제공하는 트레이트 매크로

#[derive(DeriveIden)]
pub enum CustomUser {
    #[sea_orm(iden = "TB_USER_INFO")] // 실제 DB 테이블 이름 명시
    Table,

    #[sea_orm(iden = "user_idx")]     // 실제 DB 컬럼 이름 명시
    Id,

    #[sea_orm(iden = "USER_EMAIL")]   // 대문자도 문제 없음!
    Email,

    #[sea_orm(iden = "phone-number")] // 하이픈이 들어간 레거시 이름도 가능!
    Phone,
}
```
SQL 식별자를 문자열로 사용해서 명시적으로 표현도 가능

### DB 연결

---

CRUD 연산을 하는 SQL을 실행하려면 DB에 연결을 해야한다.

```rust
use std::env; // cargo add dotenvy
use sea_orm::{Database, DbErr};

if let Err(e) = dotenvy::dotenv() {
eprintln!(".env 파일을 로드하지 못했습니다: {}", e);
}

let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
let conn = Database::connect(database_url).await?;
```

await?의 뒷부분에 붙은 물음표(?) 연산자 때문에 main 함수의 반환 타입을 변경 -> `Result<(), DbErr>`

### SELECT, INSERT, UPDATE, DELETE

---

- 쿼리 빌더를 사용하려면 src/entities에 생성한 각 테이블의 엔티티를 가져와야함
- lib.rs 에 entities 모듈 추가

```rust
// query
// 생성된 엔터티 모듈들을 가져옵니다. (경로는 프로젝트 구조에 맞게 조절)
use crate::entities::users::{Entity, Model};
// 엔티티를 테이블과 연결하는 트레이트
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
```

```rust

.route("/", get(|State(data): State<AppState>| async move {
// SELECT
// user table
let users: Vec<Model> = Entity::find().all(&data.db).await.unwrap();
println!("{:#?}", users);

Json(json!({ "users": users })) // #[derive(Serialize)] 직접 추가하거나, sea-orm 피처 플래그에 serde 추가

}))
.route(
"/{user_id}",
get(|Path(user_id): Path<String>, State(data): State<AppState>| async move {
println!("user_id: {}", user_id);

let user: Model = Entity::find()
.filter(users::Column::Id.eq(user_id.parse::<i32>().unwrap()))
.one(&data.db).await
.unwrap()
.unwrap();

println!("{:#?}", user);

Json(json!({ "user": user }))

}),
```

all: Vec<Model>을 반환   
one: Model을 반환