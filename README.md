# Installation

* [**Template of the project**](https://github.com/taishingi/continuous-template)
  * [**Docker**](https://docs.docker.com/engine/install/), [**Packer**](https://developer.hashicorp.com/packer/docs), [**Git**](https://git-scm.com) must be installed on your system.

[**Docker root less**](https://linuxhandbook.com/rootless-docker/)

```bash
cargo install continuous-testing
```

## Arch user installation

```bash
yay -S continuous-testing
```

## Init a repository

```bash
cd project
```

```bash
git init
```

```bash
again init
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

All providers scripts are based on [**archlinux**](https://archlinux.org) from my [**docker hub**](https://hub.docker.com/u/taishingi) to simplify testing.

[**D**](https://hub.docker.com/r/taishingi/dlang/tags) [**Rust**](https://hub.docker.com/r/taishingi/rlang/tags) [**Go**](https://hub.docker.com/r/taishingi/glang/tags) [**Bash**](https://hub.docker.com/r/taishingi/shlang/tags)

> Manually swith to release

```bash
cd continuous && git checkout -b new-branch $version || exit 1
```
