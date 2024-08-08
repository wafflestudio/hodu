# CONTRIBUTING

## Setup Guide

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Install Docker

운영체제에 맞게 설치해 주세요.

### Install Bruno & bruno cli

- Bruno 는 [여기](https://www.usebruno.com/downloads) 서 가이드에 따라 설치합니다.
  - Bruno를 실행하고, `./tests/bruno` 폴더를 collection 으로 엽니다.
- Bruno CLI 는 `npm install -g @usebruno/cli` 로 설치합니다.

<br/><br/>

## Run

로컬에 isolate 가 설치되어 있다면 아래와 같이 실행할 수 있습니다.

```sh
cargo run
```

로컬에 isolate 가 설치되어 있다면 아래와 같이 Docker 를 활용할 수밖에 없습니다.

```sh
docker build -t hodu .
docker run --privileged -dp 8080:8080 hodu
```

## Test

### E2E Test

서버가 실행되고 있어야 합니다.

```bash
cd hodu-server/tests/bruno
bru run --env local
```

<br/><br/>

## Rules for Contribution

- 브랜치 컨벤션
  - 기본적으로 `github flow` 를 따릅니다.
  - `main` 브랜치에서 분기하여 PR을 생성하고, 다시 `main` 브랜치로 병합합니다.
  - 기본 설정에 따라, 병합 시 `Squash and Merge` 를 사용해야 하며, commit 은 `Pull Request Title and Description` 으로 해야 합니다.
  - 머지되면 아무 의미 없기 때문에, 브랜치 이름과 커밋 이름은 자유롭게 작성해도 됩니다.
- 코드 리뷰
  - PR은 최소 1명 이상의 code owner의 Approve 를 받아야 머지할 수 있습니다.
  - code owner들은 bypass 할 수 있으나, 역시 Approve를 받길 권장합니다.
  - 0시~12시에 올린 PR은 당일, 12시~24시에 올린 PR은 다음날까지 code owner의 리뷰가 완료될 것입니다.
  - 각 코드 리뷰 코멘트 맨 앞에는 `P1` ~ `P3` 이 붙을 것입니다. 각각 아래 의미입니다.
    - `P1`: 꼭 해 달라
    - `P2`: 해 주면 좋겠다 or 코멘트
    - `P3`: 사소한 거 or 질문
- 원활한 리뷰와 적은 컨플릭을 위해, PR은 작게 올려 주세요.
  - `.rs` 파일 (서비스 로직) 변경사항 300줄 이내로 제한합니다.
- 아키텍처에 영향을 주는 변경사항의 경우 [ARCHITECTURE.md](./ARCHITECTURE.md)를 업데이트해야 합니다.
  - `ARCHITECTURE.md` 가 변경되는 PR의 경우 PR Description 에 [ADR](https://github.blog/engineering/why-write-adrs/)을 작성해야 합니다.
