#!/bin/zsh

set -e

PWD=$(pwd)
BUILD_TYPE=release

library_file_name="libgix_guard_kit.a"
lib_out_file_name="libgix_guard_kit"
declare -A LIPO_LIB_PATHS=()

mkdir -p $PWD/build/lib
mkdir -p $PWD/build/include

MAC_ARCHS=("x86_64-apple-darwin" "aarch64-apple-darwin")
IOS_ARCHS=("aarch64-apple-ios" "x86_64-apple-ios")
IOS_SIM_ARCHS=("aarch64-apple-ios-sim")

declare -A BUILD_ARCHS=()

if [[ "$PLATFORM_NAME" =~ ^(iphoneos)$ ]]
then
    echo "iphone detected"
    BUILD_ARCHS=("${IOS_ARCHS[@]}")
    lib_out_file_name="${lib_out_file_name}_ios.a"
elif [[ "$PLATFORM_NAME" =~ ^(iphonesimulator)$ ]]
then
    echo "iphone simulator detected"
    BUILD_ARCHS=("${IOS_SIM_ARCHS[@]}")
    lib_out_file_name="${lib_out_file_name}_ios_sim.a"
elif [[ "$PLATFORM_NAME" =~ ^(macosx|osx)$ ]]
then
    echo "macos detected"
    BUILD_ARCHS=("${MAC_ARCHS[@]}")
    lib_out_file_name="${lib_out_file_name}_osx.a"
fi

COUNT=0
for BUILD_ARCH in "${BUILD_ARCHS[@]}" ; do
    cargo build --release --target $BUILD_ARCH
    LIPO_LIB_PATHS[$COUNT]="$PWD/target/$BUILD_ARCH/$BUILD_TYPE/$library_file_name"
    COUNT=$((COUNT+1))
done

echo "LIPO_LIB_PATHS: ${LIPO_LIB_PATHS[@]}"

/usr/bin/lipo -create ${LIPO_LIB_PATHS[@]} -output "$PWD/build/lib/$lib_out_file_name"
cp -R $PWD/bindings/c/include $PWD/build/