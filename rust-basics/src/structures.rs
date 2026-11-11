// 이름이 있는 필드를 가진 일반 구조체다.
// Debug를 파생하면 `{:?}` 또는 `{:#?}` 형식으로 값을 출력할 수 있다.
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 매개변수와 필드 이름이 같으면 필드 초기화 단축 문법을 사용할 수 있다.
fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// 튜플 구조체는 필드 이름 없이 타입과 순서로 데이터를 구분한다.
// 같은 필드 타입을 가지더라도 서로 다른 튜플 구조체는 서로 다른 타입이다.
#[derive(Debug)]
struct Color(u8, u8, u8);

// 필드가 없는 유닛 구조체다.
// 저장할 데이터는 없지만 특정 트레이트를 구현할 타입이 필요할 때 유용하다.
#[derive(Debug)]
struct AlwaysEqual;

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl 블록에는 구조체와 관련된 동작을 정의한다.
impl Rectangle {
    // 첫 매개변수가 &self인 함수는 인스턴스에서 호출하는 메서드다.
    // 불변 참조를 사용하므로 Rectangle의 소유권을 가져가지 않는다.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // self 매개변수가 없는 함수는 연관 함수다.
    // 생성자 역할의 연관 함수에는 관례적으로 `new`를 많이 사용한다.
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(size: u32) -> Self {
        Self::new(size, size)
    }
}

// main.rs에서는 이 공개 함수만 호출하고, 세부 구조체는 모듈 내부에 감춘다.
pub fn run() {
    println!("\n--- 구조체 예제 ---");

    let user1 = build_user(String::from("rustacean"), String::from("rust@example.com"));

    // 구조체 갱신 문법은 명시하지 않은 필드를 user1에서 가져온다.
    // String 필드는 Copy가 아니므로 username의 소유권이 user2로 이동한다.
    let user2 = User {
        email: String::from("new@example.com"),
        ..user1
    };

    println!(
        "사용자: {}, 이메일: {}, 활성화: {}, 로그인 횟수: {}",
        user2.username, user2.email, user2.active, user2.sign_in_count
    );
    println!("{:?}", user2);

    let black = Color(0, 0, 0);
    // 튜플 구조체의 필드는 인덱스로 접근할 수 있다.
    println!("RGB: ({}, {}, {})", black.0, black.1, black.2);

    let marker = AlwaysEqual;
    println!("유닛 구조체: {marker:?}");

    let rectangle = Rectangle::new(30, 50);
    let square = Rectangle::square(20);

    println!("사각형: {rectangle:?}");
    println!("사각형 넓이: {}", rectangle.area());
    println!("정사각형을 포함하는가: {}", rectangle.can_hold(&square));
}

#[cfg(test)]
mod tests {
    use super::Rectangle;

    #[test]
    fn calculates_rectangle_area() {
        let rectangle = Rectangle::new(10, 20);
        assert_eq!(rectangle.area(), 200);
    }

    #[test]
    fn checks_whether_rectangle_can_hold_another() {
        let outer = Rectangle::new(30, 50);
        let inner = Rectangle::square(20);
        assert!(outer.can_hold(&inner));
    }
}
