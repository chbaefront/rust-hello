// protocol 모듈 자체가 비공개이므로 이 상수도 크레이트 외부에 노출되지 않는다.
// 부모인 network와 형제 모듈에서 사용할 수 있도록 부모 범위까지만 공개한다.
pub(super) const DEFAULT_PORT: u16 = 8080;
