#!/bin/bash
cd /home/kaindume/26/kong
cargo build -p kong_js_builder
cd /home/kaindume/26/frosh/client/
../kong/target/debug/kong_js_builder kong-kontrollers.toml
cargo run

rm -rf /home/kaindume/26/sme/global-links/www/portal
cp -R ./.www /home/kaindume/26/sme/global-links/www/portal
