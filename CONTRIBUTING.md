# CONTRIBUTING

## Setup Guide

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Install Bruno & bruno cli

- Bruno 는 [여기](https://www.usebruno.com/downloads) 서 가이드에 따라 설치합니다.
  - Bruno를 실행하고, `./tests/bruno` 폴더를 collection 으로 엽니다.
- Bruno CLI 는 `npm install -g @usebruno/cli` 로 설치합니다.

<br/>

## Run

- `cargo run`
- 또는 `cargo build` 후 `./target/debug/waffle-judge`

## Test

### E2E Test

서버가 실행되고 있어야 합니다.

```bash
cd tests/bruno
bru run
```

## Rules for Contribution

- `main` 브랜치에서 분기하여 PR을 생성하고, 다시 `main` 브랜치로 병합합니다.
  - 기본 설정에 따라, 병합 시 `Squash and Merge` 를 사용해야 하며, commit 은 `Pull Request Title and Description` 으로 해야 합니다.
- 아키텍처에 영향을 주는 변경사항의 경우 [ARCHITECTURE.md](./ARCHITECTURE.md)를 업데이트해야 합니다.
  - `ARCHITECTURE.md` 가 변경되는 PR의 경우 PR Description 에 [ADR](https://github.blog/engineering/why-write-adrs/)을 작성해야 합니다.
