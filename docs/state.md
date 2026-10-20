# state.md

---

## state

---
Axum에서는 State를 사용해 앱 전체에서 공유되는 상태를 관리할 수 있음

`with_state()` 함수를 사용해 핸들러간 상태를 공유할 수 있음

```
let app = Router::new()
        .route("/", get(callback))
        .nest("/users", users())
        .nest("/state", state())
        .with_state(data);
```

위 처럼 사용하면 되는데, 타입 에러가 발생한다.
```
Type mismatch [E0308]

Expected:()
Found:Vec<i32>
```

nest()로 체이닝 된 라우터 반환 타입을 아래처럼 변경
```
Router<Vec<i32>>
```
`Router<Vec<i32>>`: `Vec<i32>` 상태를 제공받아야 하는 Router

```
Router::new() 는 Router<()> 로 추론
```


```rust
use axum::Router;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;

pub fn state() -> Router<Vec<i32>> {
    Router::new().route("/", get(state_hello))
}

async fn state_hello(State(mut data): State<Vec<i32>>) -> impl IntoResponse {
    data[0] += 1;
    println!("{:?}", data);
    
    format!("Hello, world! {}!", data[0])
}
```

`extract::State` 를 통해 공유된 상태를 추출

[1, 0, 0] 만 계속 동일하게 출력이 된다. 값이 늘어나지 않는다.

이유는 State를 통해 공유할 수 있는 값은 반드시 Clone 트레이트를 구현하고 있어야하고 State를 통해 공유될 때는 항상 클론된 값이 전달되기 때문이다.   
만약, 정말로 값을 변경하고 싶다면 스레드 안전성을 위해 `Arc<Mutex>`를 사용해야 한다.   
(원본을 복제한걸 계속 수정하니 변경되지 않음 = 독립적)

요청마다 Vec<i32>가 복제되어 전달되므로 매번 [1, 0, 0]이 출력되는 거야

`Arc<Mutex<T>>`

```rust
use std::sync::{Arc, Mutex};

// Arc::new() 
// Mutex::new()
let data = Arc::new(Mutex::new(vec![0, 3]));
```

Arc는 모든 스레드가 데이터에 대한 레퍼런스를 갖도록 보장   
Mutex는 한 번에 하나의 스레드만 데이터를 변경할 수 있도록 해줌

```rust
use axum::Router;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use std::sync::{Arc, Mutex};

pub fn state() -> Router<Arc<Mutex<Vec<i32>>>> {
    Router::new().route("/", get(state_hello))
}

async fn state_hello(State(data): State<Arc<Mutex<Vec<i32>>>>) -> impl IntoResponse {
    // data[0] += 1;
    let mut target_data = data.lock().unwrap(); // 데이터에 접근하려면 명시적으로 lock을 해줘야함.
    target_data[0] += 1;
    println!("{:?}", target_data);

    format!("Hello, world! {}!", target_data[0])
}
```

타입변경 및 Arc<Mutex<T>>에 접근하여 처리

예제는 벡터형태로 했으나, 실제 애플리케이션 개발에서는 구조체를 통해 저장하고 공유를 함

```
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub cache: Arc<RwLock<HashMap<String, String>>>,
    pub config: Arc<AppConfig>,
}
```
`#[derive(Clone)]` 트레이트가 중요

ppState가 복제될 때 Arc 내부의 데이터가 복제되는 게 아니라, 참조 카운트만 증가하면서 동일한 데이터에 접근

```
async fn get_user(
    State(state): State<AppState>,
) {
    // state.db
    // state.cache
    // state.config
}
```


Axum의 State는 요청 간 공유되는 애플리케이션 상태이고, 요청마다 새롭게 생성되는 데이터와는 다름.

보통 서버 시작 시, 상태를 생성하여 각 요청의 핸들러에서 필요한 필드에 접근하는 구조로 설계.

## extension

---