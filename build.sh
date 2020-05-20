version=$(git describe --tags)

cargo build --release --bin passwordmanager
cargo build --release --bin passwordmanager-gui
zip -j ~/passwordmanager-linux-$version target/release/passwordmanager target/release/passwordmanager-gui install/*