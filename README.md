# siege-font

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

**siege-font** is a Font generation library for the Siege Engine MMO game engine.

The Siege Engine is an MMO game engine on the Vulkan API written in the Rust language.

siege-font takes a .TTF font file and generates two things:
* A signed-distance packed font atlas texture, and
* A serialized structure containing data about the character glyph locations

It also serves as a library defining the font atlas types and for deserializing the font
atlas information.

## Font Generation

After building this crate, the `create` binary creates fonts. Run it like this:

```sh
# You should generate glyphs at very large sizes (don't worry: the output atlas will
# be shrunk):
FONTSIZE=750

# This is the width of the font atlas after generation (it will be square)
SMALLWIDTH=512

# This is the size of the margin around a character, which defines how much "spread"
# to use.  This is defined in terms of the large font size, not the shrunk version.
# Typically a power of 8, since we will probably be going from e.g. 4096 down to 512
BIG_MARGIN=32

./target/release/siege_font_create \
    ../font-atlas/examples/Gudea-Regular.ttf $FONTSIZE $BIG_MARGIN $SMALLWIDTH \
                            "Basic Latin" \
                            "Specials" \
                            "Latin-1 Supplement" \
                            "CJK Symbols and Punctuation" \
                            "Katakana" \
                            "Hiragana" \
                            "Cyrillic" \
                            "Arabic" \
                            "CJK Unified Ideographs 1" \
                            "CJK Unified Ideographs 2" \
                            "CJK Unified Ideographs 3" \
                            "CJK Unified Ideographs 4" \
                            "CJK Unified Ideographs 5" \
                            "CJK Unified Ideographs 6" \
                            "General Punctuation" \
                            "Currency Symbols" \
                            "Latin Extented-A" \
                            "Spacing Modifier Letters" \
                            "Box Drawing" \
                            "Runic"
```
