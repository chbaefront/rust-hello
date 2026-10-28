# orm.md

---

## SeaORM

- seaORM은 러스트에서 사용하는 인기있는 ORM 라이브러리

https://www.sea-ql.org/SeaORM/

### 의존성 설치
```
cargo add sea-orm

cargo add sea-orm --features sqlx-postgres,runtime-tokio-rustls,macros
```

```
sea-orm = { version = "2.0.4", features = ["sqlx-postgres", "runtime-tokio-native-tls", "macros"]}
```

- `sqlx-postgres`는 sqlx를 통해 postgress에 연결
- `runtime-tokio-native-tls`는 TLS를 사용
- `macros` #[derive(DeriveEntityModel)] 등 Sea-ORM의 핵심 정적 매크로들을 사용할 수 있게 하여, 구조체를 데이터베이스 모델로 쉽게 변환

### Docker

- 도커 설치 후, 공식 이미지 pull   
  https://hub.docker.com/_/postgres

```
# 1. pgdata라는 이름의 볼륨 생성
docker volume create pgdata

# 2. 볼륨을 연결하여 컨테이너 실행
docker run -d -p 5432:5432 --name local-postgres -e TZ=Asia/Seoul -e POSTGRES_PASSWORD=your_password -v pgdata:/var/lib/postgresql postgres

# 3. 별도 DB 프로그램 없이 도커 내부로 접속하여 확인
docker exec -it local-postgres psql -U postgres

SELECT version();
\q # psql 종료하고 나가기
```
- PostgreSQL 18 버전 이상의 공식 도커 이미지에서 데이터 저장 구조에 주의

```
CREATE SCHEMA IF NOT EXISTS "local";
```

### DB 스키마 정의

- Users, Category, Product 총 3개의 테이블을 정의

```
CREATE TABLE IF NOT EXISTS local.users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL
);

INSERT INTO local.users(username, password)
VALUES
('홍길동', '1234'),
('GD', '2222'),
('아리랑', '3333'),
('카카로트', '4444'),
('김반장', '5555');

CREATE TABLE IF NOT EXISTS local.category (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS local.product (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    price INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    FOREIGN KEY (category_id) REFERENCES local.category(id)
);

INSERT INTO local.category (name)
VALUES
    ('전자제품'),
    ('의류'),
    ('식품'),
    ('도서'),
    ('생활용품');

INSERT INTO local.product (name, price, category_id)
VALUES
    -- 전자제품 (1)
    ('맥북 프로 14인치', 2590000, 1),
    ('아이폰 16', 1250000, 1),
    ('로지텍 MX Master 3S', 129000, 1),
    ('삼성 갤럭시 버즈', 189000, 1),

    -- 의류 (2)
    ('나이키 반팔 티셔츠', 39000, 2),
    ('리바이스 청바지', 89000, 2),
    ('아디다스 후드티', 79000, 2),
    ('뉴발란스 운동화', 139000, 2),

    -- 식품 (3)
    ('제주 감귤 3kg', 19900, 3),
    ('서울우유 1L', 3200, 3),
    ('신라면 5개입', 4500, 3),
    ('스타벅스 원두 250g', 18000, 3),

    -- 도서 (4)
    ('클린 코드', 33000, 4),
    ('리팩터링 2판', 35000, 4),
    ('모던 자바스크립트 Deep Dive', 45000, 4),
    ('러스트 프로그래밍 공식 가이드', 38000, 4),

    -- 생활용품 (5)
    ('무인양품 수납박스', 15900, 5),
    ('다이슨 청소기', 699000, 5),
    ('필립스 전동칫솔', 89000, 5),
    ('테팔 프라이팬', 39000, 5);
```

![img.png](img.png)

만약, 마이그레이션을 원한다면 아래 명령어를 실행(또는 Entity 생성에 사용)
```
cargo install sea-orm-cli
```

```
sea-orm-cli migrate init
```

![img_1.png](img_1.png)
- migration/ 은 독립적인 Rust 크레이트이므로 Cargo.toml을 지니고 있음.
- migration 폴더의 Cargo.toml에 추가 의존성을 명시(runtime-tokio-restls, sqlx-postgres)
- m20220101 머시기 tables.rs 는 샘플 파일이므로 삭제해도 됨.

```
sea-orm-cli migrate generate [TABLE_NAME]
```
위 명령어를 사용하여 새로운 마이그레이션 파일을 생성

```
use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261028_191527_users"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();
    }
}
```
- `up함수`와 `down함수`를 정의(up함수는 마이그레이션이 실행될 때 호출되는 함수, down은 마이그레이션을 롤백할 때 호출되는 함수)
- `manager` 객체를 이용해 새로운 테이블을 생성하고 컬럼을 정의

```
sea-orm-cli migrate up
```

루트 경로에 .env 생성하여 DB 연결을 맺을 수 있게 해줘야한다.
```
SeaORM CLI->DATABASE_URL 환경변수->마이그레이션 실행

DATABASE_URL=postgres://postgres:1234@localhost:5432/rust_db
```
![img_2.png](img_2.png)

test 테이블이 생성되었다..!


DB에 생성된 스키마를 기준으로 쿼리 모델을 생성하는 명령어는 아래와 같다.
```
# 현재 스키마를 기반으로 Entity 생성
sea-orm-cli generate entity \
  -u postgres://postgres:password@localhost:5432/mydb \
  -s local \
  -o src/entities
```