# Rustt Language Compiler

## 소개

Rusttc는 Rust 언어의 라이트 버전 언어인 Rustt을 위한 컴파일러 구현 프로젝트로 Rust의 기능을 간소화하여, 간단하고 가벼운 버전의 Rustt 언어를 제공하는 것입니다.
Rustt는 Rust 언어의 기본 구문과 특징을 유지하면서, 컴파일 과정과 실행 효율성에 초점을 맞추고 있습니다.

## 프로젝트 목표

- **간소화된 Rust 언어 지원**: Rustt는 기본적인 데이터 타입과 연산, 소유권 개념 등 Rust의 핵심 기능을 지원하면서, 복잡한 특징들은 제외합니다.
- **LLVM 백엔드 사용**: Rustt 컴파일러는 LLVM을 백엔드로 사용하여, 효율적인 기계어 코드 생성을 목표로 합니다. 이를 통해 Rustt는 높은 성능과 최적화된 실행 파일을 제공합니다.
- **학습 및 교육용 도구로의 활용**: Rustt는 Rust 언어의 핵심 개념을 배우고자 하는 학습자들에게 이상적인 도구로 활용될 수 있습니다. 또한, 컴파일러 구조 및 설계에 대한 학습에도 유용합니다.

## 개발 상태

Rustt는 현재 개발 초기 단계에 있으며, 기본적인 렉서(lexer)와 파서(parser) 구현에 집중하고 있습니다. 컴파일러의 전체 아키텍처 및 모듈 구조는 Rust의 구현을 참조하여 라이트하게 구현할 예정입니다.