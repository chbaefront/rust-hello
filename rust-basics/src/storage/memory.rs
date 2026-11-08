// 공개 함수이므로 `storage::memory::save()` 경로로 호출할 수 있다.
pub fn save(value: &str) {
    println!("메모리에 저장: {value}");
}
