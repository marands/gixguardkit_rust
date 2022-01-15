#!/bin/sh

##cargo lipo --targets aarch64-apple-ios,x86_64-apple-ios --release
#cargo build --release --target aarch64-apple-ios
#cargo build --release --target x86_64-apple-ios
#lipo -create $(PWD)/target/aarch64-apple-ios/release/libgix_guard_kit.a  $(PWD)/target/x86_64-apple-ios/release/libgix_guard_kit.a -output $(PWD)/build/lib/libgix_guard_kit_ios.a

set -e

BUILD_TYPE=release
library_file_name="libgix_guard_kit.a"
lib_out_file_name="libgix_guard_kit_ios.a"
declare -a BUILD_ARCHS=("aarch64-apple-ios" "x86_64-apple-ios")

PWD=$(pwd)
LIPO_ITEMS_STRING=""
declare -a LIPO_LIB_PATHS=()

mkdir -p $PWD/build/lib
mkdir -p $PWD/build/include

COUNT=0
for BUILD_ARCH in "${BUILD_ARCHS[@]}" ; do
    cargo build --release --target $BUILD_ARCH
    LIPO_LIB_PATHS[$COUNT]="$PWD/target/$BUILD_ARCH/$BUILD_TYPE/$library_file_name"
    COUNT=$((COUNT+1))
done

/usr/bin/lipo -create ${LIPO_LIB_PATHS[@]} -output "$PWD/build/lib/$lib_out_file_name"
cp -R $PWD/bindings/c/include $PWD/build/
