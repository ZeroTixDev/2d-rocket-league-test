cargo.exe build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/macroquad_sample_project.wasm www/static/macroquad_sample_project.wasm
basic-http-server www/static/