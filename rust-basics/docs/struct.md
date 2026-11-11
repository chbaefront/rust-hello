# Rust 구조체

구조체(struct)는 서로 관련된 여러 값을 하나의 사용자 정의 타입으로 묶는 기능입니다. 각 값에 의미 있는 이름을 붙일 수 있어 튜플보다 데이터의 의미를 명확하게 표현할 수 있습니다.

실행 가능한 전체 예제는 [`src/structures.rs`](../src/structures.rs)에 있습니다.

## 일반 구조체

`struct` 키워드로 타입을 정의하고, 중괄호 안에 필드 이름과 타입을 작성합니다.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

인스턴스를 생성할 때는 모든 필드에 값을 지정해야 합니다. 인스턴스 전체가 가변일 때만 필드 값을 변경할 수 있으며, Rust에서는 특정 필드만 가변으로 선언할 수 없습니다.

```rust
let mut user = User {
    active: true,
    username: String::from("rustacean"),
    email: String::from("rust@example.com"),
    sign_in_count: 1,
};

user.email = String::from("new@example.com");
```

## 필드 초기화 단축 문법

함수 매개변수나 지역 변수의 이름이 구조체 필드와 같으면 `username: username` 대신 `username`만 작성할 수 있습니다.

```rust
fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

## 구조체 갱신 문법

`..인스턴스` 문법을 사용하면 일부 필드만 새 값으로 지정하고 나머지는 기존 인스턴스에서 가져올 수 있습니다. `..` 표현식은 반드시 마지막에 작성합니다.

```rust
let user2 = User {
    email: String::from("new@example.com"),
    ..user1
};
```

갱신 문법도 대입이므로 소유권 규칙이 적용됩니다. 위 예제의 `String` 필드는 `Copy`가 아니기 때문에 `user1.username`의 소유권이 `user2`로 이동합니다. 반면 `bool`, `u64` 같은 `Copy` 필드는 복사됩니다.

## 튜플 구조체

튜플 구조체는 필드 이름 없이 필드의 타입만 선언합니다. 튜플과 비슷하지만 고유한 구조체 타입을 갖습니다.

```rust
struct Color(u8, u8, u8);
struct Point(u8, u8, u8);

let black = Color(0, 0, 0);
```

`Color`와 `Point`의 필드 타입이 같아도 서로 다른 타입이므로 호환되지 않습니다. 필드는 `black.0`처럼 인덱스로 접근하거나 패턴 분해로 꺼낼 수 있습니다.

## 유닛 구조체

유닛 구조체는 필드가 없는 구조체입니다.

```rust
struct AlwaysEqual;

let marker = AlwaysEqual;
```

저장할 데이터는 없지만 특정 동작이나 트레이트를 구현하는 타입 자체가 필요할 때 사용할 수 있습니다.

## 메서드

`impl` 블록을 사용해 구조체와 관련된 함수를 정의합니다. 첫 번째 매개변수로 `self`, `&self`, `&mut self`를 받는 함수는 메서드이며 인스턴스에서 점 문법으로 호출합니다.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

let rectangle = Rectangle { width: 30, height: 50 };
println!("{}", rectangle.area());
```

메서드의 수신 방식은 일반 함수 매개변수와 동일한 소유권 규칙을 따릅니다.

| 수신자 | 의미 |
| --- | --- |
| `self` | 인스턴스의 소유권을 가져옴 |
| `&self` | 인스턴스를 불변으로 빌림 |
| `&mut self` | 인스턴스를 가변으로 빌림 |

Rust는 메서드 호출 시 필요한 참조와 역참조를 자동으로 적용하므로 `rectangle.area()`처럼 호출할 수 있습니다.

## 연관 함수

`impl` 블록 안에 있지만 `self`를 매개변수로 받지 않는 함수는 연관 함수입니다. 인스턴스가 아니라 `타입::함수()` 형태로 호출합니다.

```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(size: u32) -> Self {
        Self::new(size, size)
    }
}

let rectangle = Rectangle::new(30, 50);
let square = Rectangle::square(20);
```

`Self`는 현재 `impl` 대상 타입을 의미합니다. `new`는 언어가 강제하는 특별한 이름은 아니지만 생성자 역할의 연관 함수에 관례적으로 사용합니다.

## 출력과 파생 트레이트

구조체는 기본적으로 `Display`나 `Debug`를 구현하지 않습니다. 개발 중 값을 확인하려면 `Debug` 트레이트를 파생할 수 있습니다.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

println!("{rectangle:?}");  // 한 줄 출력
println!("{rectangle:#?}"); // 보기 좋게 여러 줄 출력
```

`dbg!` 매크로도 파일과 줄 번호를 포함해 표현식의 값과 결과를 출력합니다. `println!`과 달리 표현식의 소유권을 받아 평가한 뒤 값을 다시 반환합니다.

## 구조체 필드와 가시성

구조체를 `pub`으로 공개해도 각 필드는 자동으로 공개되지 않습니다. 다른 모듈에서 직접 읽거나 생성해야 하는 필드에는 별도로 `pub`을 지정하거나, 생성자와 메서드를 통해 접근하도록 설계합니다.

```rust
pub struct Account {
    pub username: String,
    password: String,
}
```

이 경우 외부 모듈은 `username`에는 접근할 수 있지만 `password`에는 직접 접근할 수 없습니다.

## 참조를 필드로 저장할 때

구조체가 참조를 필드로 저장하면 참조가 구조체보다 먼저 무효화되지 않도록 수명(lifetime)을 명시해야 합니다.

```rust
struct Profile<'a> {
    name: &'a str,
}
```

`'a`는 `Profile` 인스턴스가 내부의 `name` 참조보다 오래 살아남을 수 없다는 관계를 표현합니다. 수명은 별도의 개념이므로 구조체 기초 단계에서는 소유 타입인 `String`을 사용하는 예제부터 익히는 편이 좋습니다.
