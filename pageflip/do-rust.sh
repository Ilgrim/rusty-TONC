#!/bin/bash -ex

target=pageflip

mkdir -p out

# "Compile images"
cd resources
grit page_pic.png -gb -ah16 -gB8 -o page_pic_1 -ftb -p'!'
grit page_pic.png -gb -at16 -ah16 -gB8 -o page_pic_2 -ftb -p'!'
grit page_pic.png -g'!' -p -ftb -pu32 -pn16 -o page_pic
cd ..



xargo build --target=gba --release --verbose

# I have no idea what it's in target/gba
arm-none-eabi-objcopy -O binary target/gba/release/$target out/${target}.gba
gbafix out/${target}.gba
open out/${target}.gba
