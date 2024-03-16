#!/bin/sh

cargo lipo --release

mkdir -p include
mkdir -p libs

make ios
make macos
cbindgen src/lib.rs -l c > totp_ios.h

cp totp_ios.h include
rm totp_ios.h

if [ ! -f include/module.modulemap ]; then
    echo 'module TOTP {
      header "totp_ios.h"
      export *
    }' > include/module.modulemap
fi

if [ ! -f TOTP.xcframework ]; then
    xcodebuild -create-xcframework \
    -library libs/libtotp_ios-macos.a \
    -headers ./include/ \
    -library libs/libtotp_ios-sim.a \
    -headers ./include/ \
    -library libs/libtotp_ios.a \
    -headers ./include/ \
    -output TOTP.xcframework
fi