# MAHF Contribution Guidelines

Thank you for your interest in contributing to the MAHF ecosystem! We welcome contributions from the community to help
make this project better.

Before you get started, please take a moment to read through these contribution guidelines to ensure a smooth and
collaborative experience.

## Table of Contents

* [Contributing](#contributing)
    * [Reporting Bugs](#reporting-bugs)
    * [Suggesting Enhancements](#suggesting-enhancements)
    * [Documentation](#documentation)
    * [Code Contributions](#code-contributions)
* [Pull Requests](#pull-requests)
* [Code Guidelines](#code-guidelines)
    * [Code Style](#code-style)
    * [Testing](#testing)
    * [Documenting](#documenting)
* [License](#license)

## Contributing

We welcome contributions in various forms, including bug reports, feature requests, code contributions, and improvements
to documentation.

In general, if you find an issue that addresses the problem you're having, please add your own reproduction information
to the existing issue rather than creating a new one. Adding
a [reaction](https://github.blog/2016-03-10-add-reactions-to-pull-requests-issues-and-comments/) can also help be
indicating to our maintainers that a particular problem is affecting more than just the reporter.

### Reporting Bugs

If you encounter a bug, please [open an issue](https://github.com/mahf-opt/mahf/issues) with a detailed description of
the problem, steps to reproduce it, and any relevant error messages.
Additionally, please annotate your issue with the [bug](https://github.com/mahf-opt/mahf/issues/labels) label.

### Suggesting Enhancements

If you have an idea for an enhancement or a new feature, please [open an issue](https://github.com/mahf-opt/mahf/issues)
with a clear description of the proposed change and its potential benefits.
Additionally, please annotate your issue with the [enhancement](https://github.com/mahf-opt/mahf/issues/labels) label.

### Documentation

Improvements to documentation are always appreciated. If you find any errors or areas that need clarification, please
submit a pull request with your changes.

### Code Contributions

In general, we follow the ["fork-and-pull" Git workflow](https://github.com/susam/gitpr)

1. Fork the repository to your own GitHub account.
2. Clone the project to your machine.
3. Create a branch locally with a succinct but descriptive name.
4. Commit changes to the branch
   following [commit message guidelines](https://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html).
5. Ensure your code follows our [code guidelines](#code-guidelines).
6. Push changes to your fork.
7. Open a PR in our repository and describe the changes you've made.

## Pull Requests

PRs to our projects are always welcome and can be a quick way to get your fix or improvement slated for the next
release.
When submitting a PR, please:

1. Provide a clear and concise title and description.
2. Reference any related issues in your pull request description.
3. Ensure that your code passes the [CI](#code-guidelines).
4. Be prepared to address feedback and make necessary changes.

For changes that address core functionality or would require breaking changes (e.g. a major release), it's best to open
an issue to discuss your proposal first. This is not required but can save time creating and reviewing changes.

## Code Guidelines

Our projects have strict guidelines for styling, testing, and documenting code.

Note that the [CI](./.github/workflows) checks `master` and PRs into `master` for compliance to the guidelines.
If the CI fails on your PR, please check and fix the errors.

### Code Style

#### Clippy

Clippy with the `-D warnings` flag is used to perform linting checks.

<details>
  <summary>Run Clippy</summary>

  ```shell
  $ cargo clippy --workspace --all-targets --all-features -- -D warnings
  ```

</details>

#### Rustfmt

Rustfmt is used to enforce consistent code formatting with the options specified in [`rustfmt.toml`](rustfmt.toml).

Some options require running rustfmt with
the [nightly toolchain](https://rust-lang.github.io/rustup/concepts/channels.html).
Note that the nightly toolchain is only required for rustfmt, and **not** for building.

<details>
  <summary>Run Rustfmt</summary>

  ```shell
  $ cargo +nightly fmt --all -- --check --verbose
  ```

</details>

### Testing

Testing is a critical aspect of maintaining code quality.

If your changes affect existing tests, please update them to accommodate for the new behaviour.
If you add a new feature that can be tested, add unit tests to check for correct behaviour.

<details>
  <summary>Run tests</summary>

  ```shell
  $ cargo test --workspace --all-features --lib --bins --tests --examples --verbose
  ```

</details>

### Documenting

Please add or update the documentation for all public items affected by your changes.

#### Docs

The documentation should build without any errors or warnings.

<details>
  <summary>Build docs</summary>

  ```shell
  $ export RUSTDOCFLAGS = -D warnings
  $ cargo doc --no-deps --verbose
  ```

</details>

#### Doctests

Please add examples for all public methods.

<details>
  <summary>Run doctests</summary>

  ```shell
  $ cargo test --workspace --workspace --all-features --doc --verbose
  ```

</details>

## License

By contributing to this project, you agree to license your contributions under
the [GNU General Public License v3.0](https://github.com/mahf-opt/mahf/blob/master/LICENSE).
