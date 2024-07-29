# 설계

## 동작

- 실행 시 `http://127.0.0.1:8080` 에 서버를 띄웁니다.
- `POST /submit` 경로에 `{ language: String, code: String }` 형태의 JSON 데이터를 받습니다.
- 받은 데이터를 language (C, JAVA 를 지원합니다.) 에 따라 컴파일하고 실행합니다.
- 실행 결과를 JSON 형태로 반환합니다.

## 구조

- `waffle-judge-server/src/main.rs`가 진입점이며 서버를 띄우는 역할을 합니다.
  - 서버를 띄울 때에는 `actix-web`을 이용하며, 직렬화를 위해 `serde`를 사용합니다.
- `waffle-judge-core/src/languages/` 에는 각 언어별 실행함수가 들어있습니다.
- 실행하려면 임시로 파일을 저장해둘 경로가 필요한데, `.temp` 경로가 해당 역할을 합니다.
  - 매 실행마다 `.temp/${랜덤 문자열}` 경로에 임시 폴더가 생성됩니다. 이때 `uuid` 를 이용합니다.
  - 실행이 끝나면 임시 폴더가 제거됩니다.
- API 콜 테스트 및 자동화 테스트에는 Bruno 를 활용합니다. Bruno Collection 은 [여기](./tests/bruno) 에 있습니다.

## CI/CD

- CI는 Github Actions 를 사용합니다. [여기](./.github/workflows/ci.yml) 을 참고해 주세요.
- CD는 아직 구축되어 있지 않습니다.
