# 설계

## 동작

- 실행 시 `http://127.0.0.1:8080` 에 서버를 띄웁니다.
- `POST /api/v1/submit` 경로에 `{ language: String, code: String }` 형태의 JSON 데이터를 받습니다.
- 받은 데이터를 language 에 따라 비동기적으로 컴파일하고 실행합니다.
- 실행 결과를 JSON 형태로 반환합니다.

## 구조

- `hodu-server/src/main.rs`가 진입점이며 서버를 띄우는 역할을 합니다.
  - 서버를 띄울 때에는 `actix-web`을 이용하며, 직렬화를 위해 `serde`를 사용합니다.
- `hodu-core/src/languages/` 에는 각 언어별 실행함수가 들어있습니다.
- `isolate`를 이용해 실행 환경을 격리합니다.
  - isolate 가 매 실행마다 격리된 box를 생성합니다.
  - 해당 box 안에 코드를 저장하고, `node` `gcc` `python3` 등의 도구를 이용하여 컴파일 및 실행합니다.
  - 실행이 끝나면 isolate 를 통해 격리된 box를 제거합니다.
- API 콜 테스트 및 자동화 테스트에는 Bruno 를 활용합니다. Bruno Collection 은 [여기](./hodu-server/tests/bruno) 에 있습니다.

## 배포

- `Docker` 를 활용하며, debian 12.6 이미지로 빌드됩니다.

## CI/CD

- CI는 Github Actions 를 사용합니다. [여기](./.github/workflows/ci.yml) 을 참고해 주세요.
  - cgroup v1 만 활성화된 호스트에서 수행해야 하는데, 이제 모두 outdate되어서 GitHub Actions 가 기본으로 제공하지 않기 때문에 CI에는 Self Hosted Runner 를 사용합니다. 이때 hodu-dev 가 배포되어 있는 ec2를 이용합니다.
- CD는 아직 구축되어 있지 않습니다.
