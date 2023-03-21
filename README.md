# OpenAI-Rust

![unit-test-status](https://github.com/itsrainingmani/openai-rust/actions/workflows/test.yml/badge.svg)
![integration-test-status](https://github.com/itsrainingmani/openai-rust/actions/workflows/integration.yml/badge.svg)

An Wrapper library for the OpenAI API written in Rust

## Endpoint Implementation Progress

- [x] List Models
- [x] Retrieve Model
- [ ] Create Completion | _in progress_
- [ ] Create Chat completion
- [ ] Create Edit
- [ ] Create Image

## Priority of Endpoints

Some endpoints are easier than others to implement.
I am going to start out with the `Models` and the `Completions` endpoints. But even before I get to actually implementing those, I have to figure a basic PoC of authenticating with the OpenAI API
