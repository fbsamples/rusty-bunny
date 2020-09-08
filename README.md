# Rusty Bunny

<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![License][license-shield]][license-url]

<br />
<p align="center">
  <h3 align="center">rusty-bunny</h3>

  <p align="center">
    rusty-bunny is a mini-clone of <a href="http://www.bunny1.org/">bunny1  </a>
    <br />
    "a tool that lets you write smart bookmarks in [rust] and then share them across all your browsers..."
    <br />
    <a href="https://github.com/fbsamples/rusty-bunny"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/fbsamples/rusty-bunny#demo">View Demo</a>
    ·
    <a href="https://github.com/fbsamples/rusty-bunny/issues">Report Bug</a>
  </p>
</p>

<!-- TABLE OF CONTENTS -->
## Table of Contents

- [Rusty Bunny](#rusty-bunny)
  - [Table of Contents](#table-of-contents)
  - [About the Project](#about-the-project)
  - [Demo](#demo)
    - [Built With](#built-with)
  - [Getting Started](#getting-started)
    - [Manual Setup](#manual-setup)
    - [VSCode Dev Container Setup](#vscode-dev-container-setup)
    - [Running](#running)
    - [Testing](#testing)
  - [Usage](#usage)
  - [Contributing](#contributing)
  - [License](#license)
  - [Contact](#contact)
  - [Acknowledgements](#acknowledgements)

<!-- ABOUT THE PROJECT -->
## About the Project

The idea for this project came after learning about `bunny1` and using it at work. I really enjoyed it and thought, "I wonder if I could build my own from scratch?" so this is it!

## Demo

![rusty-bunny demo][product-screenshot]

This is what `rusty-bunny` looks like in action.

### Built With

* [Rust](https://www.rust-lang.org/)
* [Rocket](https://rocket.rs/)

<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow the simple steps under either of the following sections:
- [Manual Setup](#manual-setup) – follow this if you prefer to install all dependencies locally.
- [VSCode Dev Container Setup](#vscode-dev-container-setup) – follow this to run the project in an isolated development environment inside a Docker container, pre-installed with all dependencies.

### Manual Setup

#### Prerequisites

Make sure you have Rust installed.

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Rocket uses the nightly version of Rust so make sure you use that. If you'd like to only use nightly for this project, you can run this from the root of the project after cloning.

```sh
# from the root of the project
rustup override set nightly
```

#### Installation

1. Clone the rusty-bunny
```sh
git clone https://github.com/fbsamples/rusty-bunny.git
```
2. Make sure you're using nightly
```sh
cargo --version
```
3. Build the project
```sh
cargo build
```
4. Follow the instructions in the [Running](#running) section.

### VSCode Dev Container Setup

#### Prerequisites

This requires VSCode, Docker and the Remote Development extension pack. For more details see [the official docs](https://code.visualstudio.com/docs/remote/containers#_system-requirements).

#### Spinning Up The Environment

- Follow [the official guide](https://code.visualstudio.com/docs/remote/containers#_quick-start-open-a-git-repository-or-github-pr-in-an-isolated-container-volume) to open this repository inside a dev container. VSCode will read the [config file](.devcontainer/devcontainer.json) provided to auto-install relevant dependencies and extensions.
- To run terminal commands, use the [integrated terminal](https://code.visualstudio.com/docs/editor/integrated-terminal) functionality.

### Running

1. Run the project
```sh
cargo run
```
2. Visit [localhost:8000](http://localhost:8000/)
3. To test a command, go to [localhost:8000/search?cmd=tw](http://localhost:8000/search?cmd=tw) and you should be redirected to Twitter

### Testing

Run the following command
```sh
cargo test
```

<!-- USAGE EXAMPLES -->
## Usage

To test out a command, type in http://localhost:8000/search?cmd= followed by your command.

The following commands are supported by `rusty-bunny`:
- "tw" -> redirects to twitter.com
- "tw @username" -> redirects to twitter.com/username
- "gh" -> redirects to github.com
- "gh username" -> redirects to github.com/username
- "gh username/repo" -> redirects to github.com/username/repo

Everything else redirects to a google search with your query.

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**. See [`CONTRIBUTING`](CONTRIBUTING.md) for more information.

<!-- LICENSE -->
## License

Distributed under the MIT License. See [`LICENSE`](LICENSE) for more information.

<!-- CONTACT -->
## Contact

If you have questions or thoughts on this project, feel free to send them my way by @'ing me on Twitter or shooting me a DM.

Joe Previte - [@jsjoeio](https://twitter.com/jsjoeio)

<!-- ACKNOWLEDGEMENTS -->
## Acknowledgements

* [The Rust Community](https://www.rust-lang.org/community)
* [Rocket.rs](https://rocket.rs/)
* [@othneildrew](https://github.com/othneildrew) - for the [README template](https://github.com/othneildrew/Best-README-Template)

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/fbsamples/rusty-bunny.svg?style=flat-square
[contributors-url]: https://github.com/fbsamples/rusty-bunny/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/fbsamples/rusty-bunny.svg?style=flat-square
[forks-url]: https://github.com/fbsamples/rusty-bunny/network/members
[stars-shield]: https://img.shields.io/github/stars/fbsamples/rusty-bunny.svg?style=flat-square
[stars-url]: https://github.com/fbsamples/rusty-bunny/stargazers
[issues-shield]: https://img.shields.io/github/issues/fbsamples/rusty-bunny.svg?style=flat-square
[issues-url]: https://github.com/fbsamples/rusty-bunny/issues
[license-shield]: https://img.shields.io/github/license/fbsamples/rusty-bunny?style=flat-square
[license-url]: https://github.com/fbsamples/rusty-bunny/blob/master/LICENSE
[product-screenshot]: demo.gif