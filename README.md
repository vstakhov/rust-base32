## rust-base32

This is a conversion module from and to [zbase32](http://philzimmermann.com/docs/human-oriented-base-32-encoding.txt)
encoding. It also supports RFC and Bleach alphabets.

The main purpose of zbase32 is to provide *human* readable encoding that is more efficient than `hex` encoding.
`zbase32` utilizes up to `len * 5 / 8` of space for encoded date and contains no padding (and hence no error control, like `base64`). However, it seems to be much readable for a human when an encoding does not contain padding.

This is my first experiment with Rust, so many things might be ugly/broken.