# Raw Pixel Sprite Showcase

This showcase demonstrates how to create a sprite from raw pixel data using Bevy. It is designed to be pixel-perfect and is suitable for rendering sprites for a Game Boy emulator or similar use cases.
Getting Started

To run the showcase, ensure that you have Rust and Cargo installed, and run the following command:

`cargo run`

This should launch the showcase in a window, where you can see the sprite being rendered.
## How it Works

The showcase uses the Image resource in Bevy to create a texture from raw pixel data. The pixel data is stored in a vector of u8 values, where each pixel is represented by four consecutive bytes (one for each color channel: red, green, blue, and alpha).

The showcase includes two systems that generate and update the sprite. The first system (spawn_sprite) creates a new sprite using the SpriteBundle component, and adds it to the game world. The second system (update_sprites) updates the pixel data for each sprite every tick / frame.

## Further Reading

For more information on using Bevy to create games and interactive applications, check out the [official Bevy documentation](https://bevyengine.org/learn/).
