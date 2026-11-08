// 같은 이름의 함수가 memory 모듈에도 있지만 모듈 경로가 달라 충돌하지 않는다.
pub fn save(file_name: &str) {
    println!("디스크에 저장: {file_name}");
}
