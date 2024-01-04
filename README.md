# A continuous integration testing project

[Template of the project](https://github.com/taishingi/continuous-template)

[Docker](https://docs.docker.com/engine/install/), [Packer](https://developer.hashicorp.com/packer/docs) and [Git](https://git-scm.com) must be installed on your system.


## Install docker

```bash
curl -fsSL https://get.docker.com/rootless | sh
```

## Configure packer

```bash
packer plugins install github.com/hashicorp/docker
```

## Install continuous testing

```bash
cargo install continuous-testing fd-find
```

![demonstration](https://raw.githubusercontent.com/taishingi/continuous-testing/master/again.gif)

## Initialize tracking

```bash
again init
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

## Usage

```bash
git add .
```

```bash
git commit -m "msg"
```

## The continuous directory

The continuous directory is now a git repository.

The latest release branch take the name of your environment variable $USER.

You can back to an old release if you want or create your branch.

All providers scripts are based on [archlinux](https://archlinux.org) from my [docker hub](https://hub.docker.com/u/taishingi) to simplify testing.

[D](https://hub.docker.com/r/taishingi/dlang/tags) [Rust](https://hub.docker.com/r/taishingi/rlang/tags) [Go](https://hub.docker.com/r/taishingi/glang/tags) [Bash](https://hub.docker.com/r/taishingi/shlang/tags)

> Manually switch to release

```bash
cd continuous && git checkout -b new-branch-name $tag || exit 1
```
## File structure
