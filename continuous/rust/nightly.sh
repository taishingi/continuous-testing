echo nightly

rustup default nightly 
sudo apt-get install git -y
git clone https://github.com/taishingi/continuous-testing
cd continuous-testing
cargo build
