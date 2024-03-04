# What it's ?

<img src="https://raw.githubusercontent.com/taishingi/continuous-testing/master/.icon/notif.png" alt="continuous" width="250" align="right">

It's a project to run continuous testing to check if the last commit have not broken something.

On every commit run a git clone of your project in the container, print the latest commit, run build and tests.

On your computer you will see notification after the build or on the tracking initialize.

The continuous directory is now a git repository.

The latest release branch take the name of your environment variable $USER.

You can be back to an old release if you want or create your branch.

All providers scripts are based on [archlinux](https://archlinux.org) from my [docker hub](https://hub.docker.com/u/taishingi) to simplify testing.

[D](https://hub.docker.com/r/taishingi/dlang/tags) [Rust](https://hub.docker.com/r/taishingi/rlang/tags) [Go](https://hub.docker.com/r/taishingi/glang/tags) [Bash](https://hub.docker.com/r/taishingi/shlang/tags) [Php](https://hub.docker.com/r/taishingi/plang) [C](https://hub.docker.com/r/taishingi/clang) [Java](https://hub.docker.com/r/taishingi/jlang) [Template](https://github.com/taishingi/continuous-template)

```bash
cd continuous && git checkout -b new-branch-name $tag || exit 1
```

[![continuous](https://github.com/taishingi/continuous-testing/actions/workflows/continuous.yml/badge.svg)](https://github.com/taishingi/continuous-testing/actions/workflows/continuous.yml)
[![zuu](https://github.com/taishingi/continuous-testing/actions/workflows/zuu.yml/badge.svg)](https://github.com/taishingi/continuous-testing/actions/workflows/zuu.yml)

![demonstration](https://raw.githubusercontent.com/taishingi/continuous-testing/master/again-3.0.0.gif)

![Audit](https://raw.githubusercontent.com/taishingi/continuous-testing/master/badges/social/audit.svg)
![Check](https://raw.githubusercontent.com/taishingi/continuous-testing/master/badges/social/check.svg)
![Clippy](https://raw.githubusercontent.com/taishingi/continuous-testing/master/badges/social/clippy.svg)
![Format](https://raw.githubusercontent.com/taishingi/continuous-testing/master/badges/social/fmt.svg)
![Test](https://raw.githubusercontent.com/taishingi/continuous-testing/master/badges/social/test.svg)

## GitHub workflow

```yaml
name: continuous
on:
  push:
    branches: [ "master" , "develop" ]
  pull_request:
    branches: [ "master" , "develop"]
env:
  CARGO_TERM_COLOR: always
jobs:
  continuous:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: deps 
      run: sudo apt-get install -y curl fd-find git docker-ce docker-ce-cli containerd.io docker-buildx-plugin packer && packer plugins install github.com/hashicorp/docker
    - name: continuous
      run: git clone https://github.com/taishingi/continuous-template.git continuous && cd continuous/rust && ./scripts-gen "github.com" "username" "repository" "${GITHUB_REF##*/}" "cpu" && packer validate . && packer build .
```

## Local workflow 

[Docker](https://docs.docker.com/engine/install/), [Packer](https://developer.hashicorp.com/packer/docs) [Git](https://git-scm.com) and 
lib notify must be installed on your system.

### Install docker

```bash
curl -fsSL https://get.docker.com/rootless | sh
```

### Configure packer

```bash
packer plugins install github.com/hashicorp/docker
```

### Configure packer for go

```bash
packer plugins install github.com/hashicorp/googlecompute
````

### Install continuous testing

#### Archlinux

![AUR License](https://img.shields.io/aur/license/continuous-testing?style=social)
![AUR Maintainer](https://img.shields.io/aur/maintainer/continuous-testing?style=social)
![AUR Version](https://img.shields.io/aur/version/continuous-testing?style=social)
![AUR Votes](https://img.shields.io/aur/votes/continuous-testing?style=social)

```bash
paru -S continuous-testing
```

#### Others

```bash
cargo install continuous-testing fd-find
```

### Initialize tracking

```bash
again init
```

### Again configuration

```dotenv
AGAIN_REPOSITORY=continuous-testing
AGAIN_USERNAME=taishingi
AGAIN_BRANCH=master
AGAIN_DOMAIN=github.com
AGAIN_CPU=5
AGAIN_LANGUAGE=rust
AGAIN_REMOTE=origin
```

### Again language valid

- c
- c++
- d
- go
- java
- php
- rust
- sh

### Usage

```bash
git add .
git commit -m "msg"
```

## Structure for rust executable

```bash
.
├── continuous
│  ├── d
│  │  ├── d.pkr.hcl
│  │  ├── README.md
│  │  └── scripts-gen
│  ├── go
│  │  ├── go.pkr.hcl
│  │  ├── main.go
│  │  ├── README.md
│  │  └── scripts-gen
│  ├── rust
│  │  ├── beta
│  │  ├── nightly
│  │  ├── README.md
│  │  ├── rust.pkr.hcl
│  │  ├── scripts-gen
│  │  └── stable
│  ├── sh
│  │  ├── bash.pkr.hcl
│  │  ├── README.md
│  │  └── scripts-gen
│  └── README.md
├── src
│  └── main.rs
└── Cargo.toml
```

## Structure for rust library

```bash
.
├── continuous
│  ├── d
│  │  ├── d.pkr.hcl
│  │  ├── README.md
│  │  └── scripts-gen
│  ├── go
│  │  ├── go.pkr.hcl
│  │  ├── main.go
│  │  ├── README.md
│  │  └── scripts-gen
│  ├── rust
│  │  ├── beta
│  │  ├── nightly
│  │  ├── README.md
│  │  ├── rust.pkr.hcl
│  │  ├── scripts-gen
│  │  └── stable
│  ├── sh
│  │  ├── bash.pkr.hcl
│  │  ├── README.md
│  │  └── scripts-gen
│  └── README.md
├── src
│  └── lib.rs
└── Cargo.toml
```

## Structure for d 

```bash
.
├── continuous
│  ├── d
│  │  ├── beta
│  │  ├── d.pkr.hcl
│  │  ├── nightly
│  │  ├── README.md
│  │  ├── scripts-gen
│  │  └── stable
│  ├── go
│  │  ├── go.pkr.hcl
│  │  ├── main.go
│  │  ├── README.md
│  │  └── scripts-gen
│  ├── rust
│  │  ├── README.md
│  │  ├── rust.pkr.hcl
│  │  └── scripts-gen
│  ├── sh
│  │  ├── bash.pkr.hcl
│  │  ├── README.md
│  │  └── scripts-gen
│  └── README.md
├── source
│  └── app.d
└── dub.json
```
## File structure for go 

```bash
.
├── continuous
│  ├── d
│  │  ├── d.pkr.hcl
│  │  ├── README.md
│  │  └── scripts-gen
│  ├── go
│  │  ├── beta
│  │  ├── go.pkr.hcl
│  │  ├── main.go
│  │  ├── nightly
│  │  ├── README.md
│  │  ├── scripts-gen
│  │  └── stable
│  ├── rust
│  │  ├── README.md
│  │  ├── rust.pkr.hcl
│  │  └── scripts-gen
│  ├── sh
│  │  ├── bash.pkr.hcl
│  │  ├── README.md
│  │  └── scripts-gen
│  └── README.md
└── go.mod
```



