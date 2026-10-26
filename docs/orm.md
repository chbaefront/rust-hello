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

### DB 스키마 정의

- Users, Category, Product 총 3개의 테이블을 정의

만약, 마이그레이션을 원한다면 아래 명령어를 실행
```
sea-orm-cli migrate init
```