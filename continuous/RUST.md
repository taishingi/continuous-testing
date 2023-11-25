# Rust template

> For rust user

```bash
git clone https://github.com/taishingi/continuous-template continuous
```

```bash
cd continuous/rust 
```

> Edit providers scripts

```bash
vim stable.sh
```

```bash
vim beta.sh
```

```bash
vim nightly.sh 
```

```bash
packer validate . 
```

```bash
packer build .
```
> .git/hooks/post-commit

```bash
#!/bin/bash

unset GIT_DIR
git push origin --all
git push origin --tags
cd continuous/rust
packer validate .
packer build .
```
