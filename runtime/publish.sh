#!/bin/bash
set -ex
cd ../crates
cd core
cargo publish
cd ..
cd macros
cargo publish
cd ..
cd image
cargo publish
cd ..
cd text
cargo publish
cd ..
cd path
cargo publish
cd ..
cd winit
cargo publish
cd ..
cargo publish
echo "published successfully."