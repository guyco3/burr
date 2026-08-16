# Contributing to burr

We love your input! We want to make contributing to this project as easy and transparent as possible.

## Pull Requests
1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. Ensure the test suite passes (`make test`).
4. Make sure your code lints and is properly formatted (`make lint`).
5. Issue that pull request!

## Building the Project
To build the CLI and all dependencies:
```bash
make build
```

## Running Tests
Run the unit tests to ensure the Cedar policy engine and core logic is functioning correctly:
```bash
make test-unit
```
To run the end-to-end (E2E) integration tests:
```bash
make test-int
```
To run all tests (both unit and integration):
```bash
make test
```

## License
By contributing, you agree that your contributions will be licensed under its Apache 2.0 License.
