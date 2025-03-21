# Icons

This directory contains the application icons and a script to generate them.

## Usage

1. Replace `icon.png` with your desired base icon (recommended size: 512x512 or larger)
2. Run the generation script:
   ```bash
   chmod +x icons/generate_icons.sh
   ./generate_icons.sh # Run in this path
   ```
3. The script will:
   - Create resized versions in the `icon.iconset` directory
   - Generate the final `icon.icns` file

## Icon Sizes

The script generates the following icon sizes:
- 16x16
- 16x16@2x (32x32)
- 32x32
- 32x32@2x (64x64)
- 128x128
- 128x128@2x (256x256)
- 256x256
- 256x256@2x (512x512)