# 🦀 Rust GitHub Actions Workflows 🚀

[![Rust](https://img.shields.io/badge/Language-Rust-blue.svg)](https://www.rust-lang.org)
[![Build Status](https://github.com/tailcallhq/rust-gh-workflows/actions/workflows/ci.yml/badge.svg)](https://github.com/tailcallhq/rust-gh-workflows/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/rust-gh-workflows)](https://crates.io/crates/rust-gh-workflows)
[![Contributors](https://img.shields.io/github/contributors/tailcallhq/rust-gh-workflows)](https://github.com/tailcallhq/rust-gh-workflows/graphs/contributors)
[![GitHub forks](https://img.shields.io/github/forks/tailcallhq/rust-gh-workflows)](https://github.com/tailcallhq/rust-gh-workflows/network/members)
[![Stars](https://img.shields.io/github/stars/tailcallhq/rust-gh-workflows?style=social)](https://github.com/tailcallhq/rust-gh-workflows/stargazers)
[![Issues](https://img.shields.io/github/issues/tailcallhq/rust-gh-workflows)](https://github.com/tailcallhq/rust-gh-workflows/issues)
[![Discord](https://img.shields.io/discord/CHANNEL_ID.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/your-channel)

## 🧑‍💻 What is Rust GitHub Workflows?

**Rust GitHub Workflows** is an awesome library that allows developers to write GitHub Actions in Rust, empowering you to automate, manage, and improve your CI/CD pipelines with a language designed for safety and performance.

GitHub Actions is powerful, but writing workflows can sometimes feel repetitive or tricky. That's where **Rust GitHub Workflows** steps in! 🦾

- 🔥 **Rust-Powered**: Leverage the performance and memory safety of Rust for writing workflows.
- 🧩 **Modular & Reusable**: Build workflows in a reusable, maintainable way.
- 📦 **Crate-friendly**: Seamless integration with your existing Rust projects.
- 🌍 **Cross-platform**: Target multiple operating systems and environments.

## 🚀 Features

- **Rust-based GitHub Actions**: Create workflows and custom actions in Rust.
- **Strong typing**: Eliminate YAML errors with Rust's type safety.
- **Easy setup**: Get started quickly with minimal setup required.
- **Expandability**: Create your own actions or reuse the community's work.

## 📦 Installation

To use **Rust GitHub Workflows** in your project, add it to your `Cargo.toml`:

```toml
[dependencies]
rust-gh-workflows = "0.1.0"
```

Then you can start creating GitHub Actions with Rust like this:

```rust
use rust_gh_workflows::{Workflow, Job};

fn main() {
    let workflow = Workflow::new()
        .job(
            Job::new("build")
                .runs_on("ubuntu-latest")
                .steps(vec![
                    "checkout",
                    "setup-rust",
                    "cargo build --release"
                ]),
        );

    workflow.generate();
}
```

## 🎉 Quick Start

1. Clone the repository:

   ```bash
   git clone https://github.com/tailcallhq/rust-gh-workflows.git
   cd rust-gh-workflows
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run an example:

   ```bash
   cargo run --example simple
   ```

4. See the generated YAML in your GitHub repository!

## 🤝 Contributing

We'd love your contributions! Whether it's bug fixes, features, or improvements, check out our [contributing guide](./CONTRIBUTING.md).

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes (`git commit -am 'Add my feature'`)
4. Push to the branch (`git push origin feature/my-feature`)
5. Create a new Pull Request!

## 🛠️ Roadmap

- [x] Initial Rust workflow generation
- [ ] Custom action library support
- [ ] Documentation improvements
- [ ] Integration testing and examples

## 💡 Why Rust?

Rust provides the perfect combination of speed, safety, and flexibility, making it an ideal choice for writing GitHub Actions. With Rust, you get strong typing, memory safety, and the ability to reuse existing code, which can make your automation scripts more robust and maintainable.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙌 Contributors

A big thank you to all the contributors who helped make this project a success! 🙏

[![Contributors](https://contrib.rocks/image?repo=tailcallhq/rust-gh-workflows)](https://github.com/tailcallhq/rust-gh-workflows/graphs/contributors)

## 🌟 Show Your Support

If you like this project, please consider giving it a ⭐ on [GitHub](https://github.com/tailcallhq/rust-gh-workflows) and share it with the community!

---

Happy automating with Rust! 🦀❤️
