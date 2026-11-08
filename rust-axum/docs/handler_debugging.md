# handler_debugging.md

---

```rust
use axum::{Router, routing::get};
use std::sync::{Arc, Mutex};

pub fn debug() -> Router<Arc<Mutex<Vec<i32>>>> {
    Router::new().route("/", get(get_handler))
}

fn get_handler() -> &'static str {
    "hello world!"
}
```


```text
error[E0277]: the trait bound `fn() -> &'static str {get_handler}: Handler<_, _>` is not satisfied                                                                                                                                
   --> src\routes\debug.rs:5:34
    |
  5 |     Router::new().route("/", get(get_handler))
    |                              --- ^^^^^^^^^^^ the trait `Handler<_, _>` is not implemented for fn item `fn() -> &'static str {get_handler}`
    |                              |
    |                              required by a bound introduced by this call
    |
    = note: Consider using `#[axum::debug_handler]` to improve the error message
note: required by a bound in `axum::routing::get`
   --> C:\Users\zero86\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\axum-0.8.9\src\routing\method_routing.rs:167:16
    |
167 |             H: Handler<T, S>,
    |                ^^^^^^^^^^^^^ required by this bound in `get`
...
441 | top_level_handler_fn!(get, GET);
    | -------------------------------
    | |                     |
    | |                     required by a bound in this function
    | in this macro invocation
    = note: this error originates in the macro `top_level_handler_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.                                                                                                                                                               
error: could not compile `rust-hello` (lib) due to 1 previous error            
```

에러가 이렇게 발생을 하는데, 에러 내용을 보고 바로 파악하기가 어렵습니다.   
Axum의 `macros` 피처에서 제공하는 `#[debug_handler]` 어트리뷰트를 사용

```
axum = { version = "0.8.9", features = ["json", "multipart", "macros"]}
```

위와 같이 피처 플래그에 추가하고, 핸들러에 다음과 같이 추가

```
use axum::{Router, routing::get, debug_handler};
use std::sync::{Arc, Mutex};

pub fn debug() -> Router<Arc<Mutex<Vec<i32>>>> {
    Router::new().route("/", get(get_handler))
}


#[debug_handler]
fn get_handler() -> &'static str {
    "hello world!"
}
```


```text
error[E0277]: `&str` is not a future                                                                                                                                                                                              
  --> src\routes\debug.rs:10:21
   |
10 | fn get_handler() -> &'static str {
   |    -----------------^
   |    |                |
   |    |                `&str` is not a future
   |    this call returns `&str`
   |
   = help: the trait `Future` is not implemented for `&str`
   = note: &str must be a future or must implement `IntoFuture` to be awaited
   = note: required for `&str` to implement `IntoFuture`
help: alternatively, consider making `fn __axum_macros_check_get_handler_into_response_make_value` asynchronous
   |
10 | fn get_handler() -> async &'static str {
   |                     +++++

error[E0277]: the trait bound `fn() -> &'static str {debug::get_handler}: Handler<_, _>` is not satisfied                                                                                                                         
   --> src\routes\debug.rs:5:34
    |
  5 |     Router::new().route("/", get(get_handler))
    |                              --- ^^^^^^^^^^^ the trait `Handler<_, _>` is not implemented for fn item `fn() -> &'static str {debug::get_handler}`
    |                              |
    |                              required by a bound introduced by this call
    |
    = note: Consider using `#[axum::debug_handler]` to improve the error message
note: required by a bound in `axum::routing::get`
   --> C:\Users\zero86\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\axum-0.8.9\src\routing\method_routing.rs:167:16
    |
167 |             H: Handler<T, S>,
    |                ^^^^^^^^^^^^^ required by this bound in `get`
...
441 | top_level_handler_fn!(get, GET);
    | -------------------------------
    | |                     |
    | |                     required by a bound in this function
    | in this macro invocation
    = note: this error originates in the macro `top_level_handler_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `&str` is not a future                                                                                                                                                                                              
 --> src\routes\debug.rs:9:1
  |
9 | #[debug_handler]
  | ^^^^^^^^^^^^^^^^ `&str` is not a future
  |
  = help: the trait `Future` is not implemented for `&str`
note: required by a bound in `__axum_macros_check_get_handler_future::check`
 --> src\routes\debug.rs:9:1
  |
9 | #[debug_handler]
  | ^^^^^^^^^^^^^^^^ required by this bound in `check`
  = note: this error originates in the attribute macro `debug_handler` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.                                                                                                                                                               
error: could not compile `rust-hello` (lib) due to 3 previous errors 
```

the trait `Future` is not implemented for `&str` 가 아니라고 하는데 이 말은 핸들러가 비동기 함수가 아니라는 뜻입니다. `async` 키워드를 추가해주면 정상적으로 컴파일 됩니다.

```rust
#[debug_handler]
async fn get_handler() -> &'static str {
    "hello world!"
}
```