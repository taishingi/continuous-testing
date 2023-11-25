echo nightly
rustup default nightly 
sudo apt-get install git -y
git clone https://github.com/taishingi/continuous-testing && cd continuous-testing || exit 1
git log -1 
sleep 10
cargo build || exit 1
