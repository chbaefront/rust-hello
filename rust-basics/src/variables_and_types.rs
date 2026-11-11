// 상수는 반드시 타입을 명시해야 하며 컴파일 시간에 계산 가능한 값만 사용할 수 있다.
// 이름은 관례적으로 대문자와 밑줄을 사용한다.
const MAX_POINTS: u32 = 100_000;
const SECONDS_PER_HOUR: u32 = 60 * 60;

pub fn run() {
    println!("\n--- 변수와 기본 타입 예제 ---");

    // let으로 선언한 변수는 기본적으로 불변이다.
    let language = "Rust";
    println!("불변 변수: {language}");

    // 값을 변경해야 한다면 변수 이름 앞에 mut를 붙인다.
    let mut score = 10;
    score += 5;
    println!("가변 변수: {score}, 최대 점수: {MAX_POINTS}");

    // 섀도잉은 같은 이름으로 새로운 변수를 선언한다.
    // 새 변수이므로 값뿐 아니라 타입도 바꿀 수 있다.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("공백 개수: {spaces}");

    scalar_types();
    compound_types();

    println!("한 시간은 {SECONDS_PER_HOUR}초입니다.");
}

fn scalar_types() {
    // 정수: 부호 있는 타입(i)과 부호 없는 타입(u)이 있다.
    let signed: i32 = -42;
    let unsigned: u32 = 42;
    let architecture_bits = usize::BITS;

    // 정수 리터럴은 진법 표기, 자릿수 구분 밑줄, 타입 접미사를 지원한다.
    let decimal = 98_222;
    let hexadecimal = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    println!(
        "정수: {signed}, {unsigned}, usize={architecture_bits}비트, {decimal}, {hexadecimal}, {octal}, {binary}, {byte}"
    );

    // 타입을 생략한 부동소수점 값은 기본적으로 f64로 추론된다.
    let double_precision = 2.5;
    let single_precision: f32 = 3.5;
    println!("부동소수점: f64={double_precision}, f32={single_precision}");

    let is_learning = true;
    let explicit_boolean: bool = false;
    println!("불리언: {is_learning}, {explicit_boolean}");

    // char는 작은따옴표로 표현하며 유니코드 스칼라 값 하나를 저장한다.
    let latin = 'R';
    let korean = '러';
    let emoji = '🦀';
    println!("문자: {latin}, {korean}, {emoji}");

    // 문자열 리터럴은 프로그램 바이너리에 저장된 UTF-8 데이터의 뷰이므로 &str이다.
    let borrowed: &str = "Rust";

    // String은 힙의 UTF-8 데이터를 소유하며 실행 중에 내용을 늘이거나 줄일 수 있다.
    let mut owned = String::from("Rust");
    owned.push_str(" language");

    // &String은 역참조 강제 변환을 통해 &str을 받는 함수에 전달할 수 있다.
    print_text(borrowed);
    print_text(&owned);

    // String의 일부를 빌려서 &str 슬라이스로 사용할 수도 있다.
    let first_word: &str = &owned[..4];
    println!("문자열: &str={borrowed}, String={owned}, 슬라이스={first_word}");
}

fn print_text(text: &str) {
    println!("문자열 길이: {}바이트", text.len());
}

fn compound_types() {
    // 튜플은 서로 다른 타입의 값을 고정된 개수로 묶는다.
    let person: (&str, u32, bool) = ("Ferris", 8, true);
    let (name, age, active) = person;
    println!("튜플 분해: 이름={name}, 나이={age}, 활성화={active}");
    println!("튜플 인덱스 접근: {}", person.0);

    // 배열은 같은 타입의 값을 고정된 길이로 저장한다.
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let repeated = [3; 4];
    println!("배열 첫 값: {}, 반복 배열: {repeated:?}", numbers[0]);
}

#[cfg(test)]
mod tests {
    use super::{MAX_POINTS, SECONDS_PER_HOUR};

    #[test]
    fn constants_have_expected_values() {
        assert_eq!(MAX_POINTS, 100_000);
        assert_eq!(SECONDS_PER_HOUR, 3_600);
    }

    #[test]
    fn shadowing_can_change_the_type() {
        let value = "123";
        let value = value.len();
        assert_eq!(value, 3);
    }
}
