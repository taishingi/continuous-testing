echo beta

rustup default beta
sudo apt-get install git -y
git clone https://github.com/taishingi/continuous-testing && cd continuous-testing || exit 1
cargo build || exit 1
