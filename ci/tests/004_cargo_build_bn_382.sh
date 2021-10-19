#!/bin/bash
# shellcheck disable=SC2086

sed -i 's!<artifactId>sc-common-cryptolib</artifactId>!<artifactId>sc-common-cryptolib-bn_382</artifactId>!g' jni/pom.xml

set -xeo pipefail

retval=0

cargo $CARGOARGS build --release --features "bn_382" --target=x86_64-pc-windows-gnu || retval="$?"
cargo $CARGOARGS build --release --features "bn_382" --target=x86_64-unknown-linux-gnu || retval="$?"
cargo $CARGOARGS build --features "bn_382" --tests || retval="$?"

mkdir -p jni/src/main/resources/native/linux64
cp target/x86_64-unknown-linux-gnu/release/libsc_common.so jni/src/main/resources/native/linux64/libsc_common.so

mkdir -p jni/src/main/resources/native/windows64
cp target/x86_64-pc-windows-gnu/release/sc_common.dll jni/src/main/resources/native/windows64/sc_common.dll

cd jni
mvn clean install -P !build-extras -DskipTests=true -Dmaven.javadoc.skip=true -B
mvn test -P !build-extras -B

exit "$retval"
