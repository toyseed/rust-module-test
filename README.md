# How to work Rust module

> Rust requires users to build an explicit module graph

러스트는 사용자가 모듈 그래프를 명시해야한다. 모듈 그래프는 crate root 로 부터 시작된다. cargo project 에서는 src/lib.rs 혹은 src/main.rs 파일이 crate root 역할을 한다.
crate rood 에 `mod` 키워드로 명시된 모듈만 crate 에 포함되어 상호 참조할 수 있다.
# rust-module-test
