<div align="center">

  <img src="docs/assets/logo.png" alt="logo" width="200" height="auto" />
  <h1>Open Knight</h1>

  <p>
    A cross-platform chess database and analysis application.
  </p>

<sub>⚠️ EARLY ALPHA ⚠️</sub>

<!-- Badges -->
<p>
  <a href="https://github.com/reidpritchard/open-knight/graphs/contributors">
    <img src="https://img.shields.io/github/contributors/reidpritchard/open-knight" alt="contributors" />
  </a>
  <a href="">
    <img src="https://img.shields.io/github/last-commit/reidpritchard/open-knight" alt="last update" />
  </a>
  <a href="https://github.com/reidpritchard/open-knight/network/members">
    <img src="https://img.shields.io/github/forks/reidpritchard/open-knight" alt="forks" />
  </a>
  <a href="https://github.com/reidpritchard/open-knight/stargazers">
    <img src="https://img.shields.io/github/stars/reidpritchard/open-knight" alt="stars" />
  </a>
  <a href="https://github.com/reidpritchard/open-knight/issues/">
    <img src="https://img.shields.io/github/issues/reidpritchard/open-knight" alt="open issues" />
  </a>
</p>

<h4>
    <a href="https://github.com/reidpritchard/open-knight">Documentation</a>
  <span> · </span>
    <a href="https://github.com/reidpritchard/open-knight/issues/">Report Bug</a>
  <span> · </span>
    <a href="https://github.com/reidpritchard/open-knight/issues/">Request Feature</a>
  </h4>
</div>

<br />

<!-- Table of Contents -->

# Table of Contents

<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [Table of Contents](#table-of-contents)
  - [About the Project](#about-the-project)
    - [Screenshots](#screenshots)
    - [Tech Stack](#tech-stack)
    - [Features](#features)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Run the project locally](#run-the-project-locally)
  - [Usage](#usage)
  - [Roadmap](#roadmap)
  - [Contributing](#contributing)
    - [Code of Conduct](#code-of-conduct)
  - [FAQ](#faq)
  - [License](#license)
  - [Contact](#contact)
  - [Acknowledgements](#acknowledgements)

<!-- /code_chunk_output -->

<!-- About the Project -->

## About the Project

<!-- Screenshots -->

### Screenshots

<div align="center">
  <img src="docs/assets/UI-May28-2025.png" alt="Screenshot of the current Open Knight UI" />
</div>

<!-- TechStack -->

### Tech Stack

<details>
  <summary>Client</summary>
  <ul>
    <li><a href="https://www.typescriptlang.org/">Typescript</a></li>
    <li><a href="https://vuejs.org/">Vue.js</a></li>
    <li><a href="https://tailwindcss.com/">TailwindCSS</a></li>
    <li><a href="https://daisyui.com/">DaisyUI</a></li>
  </ul>
</details>

<details>
  <summary>"Backend" (Tauri)</summary>
  <ul>
    <li><a href="https://www.rust-lang.org/">Rust</a></li>
    <li><a href="https://www.sea-ql.org/SeaORM/">SeaORM</a></li>
  </ul>
</details>

<details>
<summary>Database</summary>
  <ul>
    <li><a href="https://www.sqlite.org/">SQLite</a></li>
  </ul>
</details>

<!-- Features -->

### Features

- Import PGN files
- Chess game viewer and editor
- Move tree/list
- Multiple board support
- UCI engine support

<small>
  <i>
    Some features are not fully implemented and are in development, however the core functionality is there.
  </i>
</small>

<!-- Env Variables -->

### Environment Variables

Currently no environment variables are used.

<!-- Getting Started -->

## Getting Started

<!-- Prerequisites -->

### Prerequisites

This project uses `npm` as package manager.

Tauri uses `rust` and `cargo` to build the application. Instructions for installing `rust` can be found [here](https://www.rust-lang.org/tools/install).

<!-- Installation -->

### Run the project locally

Clone the project

```bash
git clone https://github.com/reidpritchard/open-knight.git
```

Go to the project directory

```bash
  cd open-knight
```

Install dependencies

```bash
  npm install
```

Setup Typia and shared types

```bash
  npm run prepare
```

Run the whole project in development mode

```bash
  npm run tauri dev
```

Run only the frontend

```bash
  npm run dev
```

<!-- Usage -->

## Usage

The project is under active development and new features are being added frequently. Currently it's not recommended to use the application in a production environment or with data without backing up first.

<!-- Roadmap (in no particular order) -->

## Roadmap

Listed in no particular order and likely to change (also the checked items are not stable, I keep breaking things):

- [x] PGN Parsing
- [x] Import PGN files
- [x] Chess game viewer
- [x] Basic UI layout customization
- [x] Move tree/list
- [x] UCI support
- [x] Theme support
- [ ] Game/Move annotations (In Progress)
- [ ] Drag and drop imports
- [ ] PGN export
- [ ] Position search
- [ ] Tactics/Motif search
- [ ] Openings support

<!-- Contributing -->

## Contributing

Contributions are always welcome!

See `contributing.md` for ways to get started.

<!-- Code of Conduct -->

### Code of Conduct

Please read the [Code of Conduct](https://github.com/reidpritchard/open-knight/blob/main/docs/CODE_OF_CONDUCT.md)

<!-- FAQ -->

## FAQ

- Does this project support X feature?

  - Likely not. It's still in early alpha, but you can always open an issue or submit a PR!

- How can I help?

  - Any contributions are welcome! See the [Contributing](https://github.com/reidpritchard/open-knight/blob/main/CONTRIBUTING.md) section for more information.

<!-- License -->

## License

The project uses the [PolyForm Noncommercial 1.0.0 License](https://github.com/reidpritchard/open-knight/blob/main/docs/LICENSE.md). See the license file for more information.

<details>
<summary>Reason for license</summary>
I chose this license because I want to keep the project open source, but also want to ensure that it's not used in a way that is harmful to the development of the project. For example, I don't want a company making a profit off of the project without contributing back. I am open to more traditional OSI approved licenses in the future, but for now this one allows me to keep more control over the project. I'd rather move from this license to something more permissive in the future rather than the other way around.
</details>

<!-- Contact -->

## Contact

Reid Pritchard - [@reidpritchard](https://github.com/reidpritchard) - <reidprichard99@gmail.com>

Project Link: [https://github.com/reidpritchard/open-knight](https://github.com/reidpritchard/open-knight)

<!-- Acknowledgments -->

## Acknowledgements

- [Shields.io](https://shields.io/)
- [Awesome README](https://github.com/matiassingers/awesome-readme)
- [Readme Template](https://github.com/othneildrew/Best-README-Template)
- [Tauri](https://github.com/tauri-apps/tauri)
