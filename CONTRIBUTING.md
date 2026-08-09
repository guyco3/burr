# Contributing to wrdn

We love your input! We want to make contributing to this project as easy and transparent as possible.

## Pull Requests
1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. Ensure the test suite passes.
4. Make sure your code lints.
5. Issue that pull request!

## Building the Project
To build the CLI and all dependencies:
```bash
make build
```

## Running Tests
Run the full automated test suite to ensure the Cedar policy engine and core logic is functioning correctly:
```bash
cargo test --workspace
```
To run the end-to-end (E2E) integration tests:
```bash
make test
```

## License
By contributing, you agree that your contributions will be licensed under its Apache 2.0 License.
