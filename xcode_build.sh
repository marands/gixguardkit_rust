#!/bin/zsh

# XCode must call this script without build environment variables

set -e
#source $HOME/.cargo/env

# Fix the issue with rust cargo build.
SDKROOT=$(xcrun --sdk macosx --show-sdk-path)
export LIBRARY_PATH="$SDKROOT/usr/lib"

PWD=$(pwd)
BUILD_TYPE=release

library_file_name="libgix_guard_kit.a"
lib_out_file_name="libgix_guard_kit"
declare -A LIPO_LIB_PATHS=()

mkdir -p "$PWD"/build/lib
mkdir -p "$PWD"/build/include

array_copy() {
    set -- "$(declare -p "$1")" "$2"
    eval "$2=${1#*=}"
}

__build_for_platform() {
  declare -A BUILD_ARCHS=()
  # shellcheck disable=SC2034
  declare -A MAC_ARCHS=( [0]="x86_64-apple-darwin" [1]="aarch64-apple-darwin")
  # shellcheck disable=SC2034
  declare -A IOS_ARCHS=( [0]="aarch64-apple-ios" [1]="x86_64-apple-ios")
  # shellcheck disable=SC2034
  declare -A IOS_SIM_ARCHS=([0]="aarch64-apple-ios-sim")
  PLATFORM_NAME=$1

  if [[ "$PLATFORM_NAME" =~ ^(iphoneos)$ ]]; then
    echo "iphone detected"
    #BUILD_ARCHS=(${IOS_ARCHS[@]})
    array_copy IOS_ARCHS BUILD_ARCHS
    lib_out_file_name="${lib_out_file_name}_ios.a"
  elif [[ "$PLATFORM_NAME" =~ ^(iphonesimulator)$ ]]; then
    echo "iphone simulator detected"
    array_copy IOS_SIM_ARCHS BUILD_ARCHS
    lib_out_file_name="${lib_out_file_name}_ios_sim.a"
  elif [[ "$PLATFORM_NAME" =~ ^(macosx|osx)$ ]]; then
    echo "macos detected"
    array_copy MAC_ARCHS BUILD_ARCHS
    lib_out_file_name="${lib_out_file_name}_osx.a"
  fi

  COUNT=0
  for BUILD_ARCH in "${BUILD_ARCHS[@]}"; do
    cargo build --release --target "$BUILD_ARCH"
    LIPO_LIB_PATHS[$COUNT]="$PWD/target/$BUILD_ARCH/$BUILD_TYPE/$library_file_name"
    COUNT=$((COUNT + 1))
  done

  echo "LIPO_LIB_PATHS:" "${LIPO_LIB_PATHS[@]}"
  lipo -create "${LIPO_LIB_PATHS[@]}" -output "$PWD/build/lib/$lib_out_file_name"
}

__build_for_platform "iphoneos"
__build_for_platform iphonesimulator

__build_for_platform "macosx"

cp -R "$PWD"/bindings/c/include "$PWD"/build/
