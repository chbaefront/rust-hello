# module.md

---

### Rust Module

- **모듈 정의**   
`mod` 키워드를 사용하여 정의
```
mod my_module {
  pub fn say_hello() {
    println!("my module!");
  }
}
```

my_module은 say_hello 함수를 포함하는 모듈   
`pub` 키워드는 모듈 외부에서 접근 가능하도록 해줍니다.   
모듈 내에서 정의된 항목은 기본적으로 비공개(private)   

- **모듈 사용**   
```
mod my_module {
    pub fn say_hello() {
        println!("my module!");
    }
}

fn main() {
    // 다른 모듈에 있는 pub 함수를 호출할 때는 :: 심볼을 사용합니다.
    my_module::say_hello(); 
}
```
`::` 연산자는 모듈 경로를 나타내며, 모듈 내의 항목에 접근하는데 사용

- **중첩 모듈**   
  하나의 모듈 안에 또 다른 모듈을 넣는 방식
```
mod network {
    // network의 자식 모듈인 server
    pub mod server {
        pub fn connect() {
            println!("서버에 연결되었습니다.");
        }
    }

    // network의 자식 모듈인 client
    pub mod client {
        pub fn send_data() {
            println!("데이터를 전송합니다.");
        }
    }
}

fn main() {
    // 경로를 ::로 이어서 호출합니다.
    network::server::connect();
    network::client::send_data();
}
```

- **파일과 디렉토리를 사용한 모듈 정의**   
파일 이름은 모듈 이름과 동일해야합니다.   
서브 모듈은 모듈 디렉토리 내의 별도 파일로 정의합니다.   
mod 모듈명; 뒤에 중괄호가 없다면 실제 파일이나 폴더 경로를 탐색   

1. main.rs에 mod route_module; 라고 작성
2. 같은 폴더 안의 파일을 찾음(route_module.rs)
3. 이름이 같은 폴더 안의 mod.rs를 찾음(구버전 스타일)
4. route_module.rs도 모듈이 선언되어 있으면, 모듈 트리는 아래와 같이 구성
```
rust-hello
└── route_module
    └── routes
        └── users
```

- **lib.rs 로 정의하여 use 키워드로 모듈 가져오기**
```
src/
├── main.rs
├── lib.rs
└── routes/
    └── users.rs
```
```rust
// lib.rs
pub mod routes {
    pub mod users;
}
```
```rust
use my_app::routes::users::users_router;

fn main() {
    users_router();
}
```

lib.rs라서 use를 사용하는 것이 아니라, lib.rs에서 선언한 모듈이 별도의 라이브러리 크레이트에 속하기 때문에 main.rs에서 그 크레이트를 참조하는 것   
정확히는 `mod`는 모듈을 정의, `use`는 모듈을 참조하는 키워드   
lib.rs는 독립된 패키지로 격상시켜주는 특별한 파일이며, 이 파일이 있어야만 컴파일러가 프로젝트 이름을 인식하여 외부 모듈을 use로 가져와 사용 가능
