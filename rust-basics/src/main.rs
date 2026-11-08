mod greeting {
    pub fn say_hello(name: &str) {
        println!("안녕하세요, {name}!");
    }
}

fn append_role(name: &mut String) {
    name.push_str(" developer");
}

fn main() {
    // String은 대입할 때 소유권이 이동한다.
    let owner = String::from("Rust");
    let moved_owner = owner;
    println!("새 소유자: {moved_owner}");

    // 참조로 빌리면 소유권을 넘기지 않고 값을 사용할 수 있다.
    let mut name = String::from("backend");
    append_role(&mut name);
    println!("가변 빌림 결과: {name}");

    // 클로저는 사용하는 방식에 따라 주변 값을 빌리거나 이동한다.
    let numbers = vec![1, 2, 3];
    let print_numbers = || println!("불변 빌림: {numbers:?}");
    print_numbers();
    println!("클로저 실행 후에도 사용 가능: {numbers:?}");

    greeting::say_hello(&name);
}
