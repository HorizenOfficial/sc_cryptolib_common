#!/bin/bash

set -euo pipefail

cargo clean

# Test with all features
cargo test --features "tweedle"
cargo test --features "bn_382" 

# ######################### Build with tweedle, test and publish ########################

# cargo build -j$(($(nproc)+1)) --release --features "tweedle" --target=x86_64-pc-windows-gnu
cargo build -j$(($(nproc)+1)) --release --features "tweedle" --target=x86_64-unknown-linux-gnu

# ########################

mkdir -p jni/src/main/resources/native/linux64
cp target/x86_64-unknown-linux-gnu/release/libsc_common.so jni/src/main/resources/native/linux64/libsc_common.so

# mkdir -p jni/src/main/resources/native/windows64
# cp target/x86_64-pc-windows-gnu/release/sc_common.dll jni/src/main/resources/native/windows64/sc_common.dll

cd jni
echo "Building jar"
mvn clean install -P !build-extras -DskipTests=true -Dmaven.javadoc.skip=true -B
echo "Testing jar"
mvn test -P !build-extras -B

# TODO: We need to find a way to publish on maven using different artifact name depending on the curve
# (e.g. sc-common-cryptolib-tweedle, sc-common-cryptolib-bn382)
# if [ "$CONTAINER_PUBLISH" = "true" ]; then
#   echo "Deploying bundle to maven repository"
#   mvn deploy -P sign,build-extras --settings ../ci/mvn_settings.xml -B
# fi

cd ..
######################### Build with bn382 and publish ########################
cargo clean

# cargo build -j$(($(nproc)+1)) --release --features "bn_382" --target=x86_64-pc-windows-gnu
cargo build -j$(($(nproc)+1)) --release --features "bn_382" --target=x86_64-unknown-linux-gnu

########################

mkdir -p jni/src/main/resources/native/linux64
cp target/x86_64-unknown-linux-gnu/release/libsc_common.so jni/src/main/resources/native/linux64/libsc_common.so

# mkdir -p jni/src/main/resources/native/windows64
# cp target/x86_64-pc-windows-gnu/release/sc_common.dll jni/src/main/resources/native/windows64/sc_common.dll

cd jni
echo "Building jar"
mvn clean install -P !build-extras -DskipTests=true -Dmaven.javadoc.skip=true -B

# TODO: We need to find a way to publish on maven using different artifact name depending on the curve
# (e.g. sc-common-cryptolib-tweedle, sc-common-cryptolib-bn382)
# if [ "$CONTAINER_PUBLISH" = "true" ]; then
#   echo "Deploying bundle to maven repository"
#   mvn deploy -P sign,build-extras --settings ../ci/mvn_settings.xml -B
# fi
