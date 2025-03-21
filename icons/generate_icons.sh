#!/bin/bash

# Check if icon.png exists
if [ ! -f "icon.png" ]; then
    echo "Error: icon.png not found in current directory"
    exit 1
 fi

# Create icon.iconset directory if it doesn't exist
mkdir -p icon.iconset

# Generate different icon sizes
sips -z 16 16 icon.png --out icon.iconset/icon_16x16.png
sips -z 32 32 icon.png --out icon.iconset/icon_16x16@2x.png
sips -z 32 32 icon.png --out icon.iconset/icon_32x32.png
sips -z 64 64 icon.png --out icon.iconset/icon_32x32@2x.png
sips -z 128 128 icon.png --out icon.iconset/icon_128x128.png
sips -z 256 256 icon.png --out icon.iconset/icon_128x128@2x.png
sips -z 256 256 icon.png --out icon.iconset/icon_256x256.png
sips -z 512 512 icon.png --out icon.iconset/icon_256x256@2x.png

# Convert iconset to icns file
iconutil -c icns icon.iconset -o icon.icns

echo "Icon generation complete!"