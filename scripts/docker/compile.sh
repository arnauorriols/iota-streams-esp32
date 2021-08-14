docker run -it -v $(pwd):/opt/streams -v cargo-cache:/opt/cargo --workdir /opt/streams --rm --name esp-rust-compiler espressif/idf-rust idf.py build
