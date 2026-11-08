// 이 파일은 `storage` 모듈의 루트다.
// main.rs의 `mod storage;` 선언은 `storage.rs`가 없으면 `storage/mod.rs`를 찾는다.

// 자식 모듈은 같은 storage 디렉터리의 memory.rs와 disk.rs에 정의한다.
pub mod disk;
pub mod memory;

// 이 함수는 storage 모듈 자체에 속하므로 `storage::print_summary()`로 호출한다.
pub fn print_summary() {
    println!("저장소 모듈은 storage/mod.rs 방식을 사용합니다.");
}
