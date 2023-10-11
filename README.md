Working prototype for (part of) my resource managing system for my fledgling pipe-dream future game engine.

Obviously 2D games are easy on modern computers performance wise, but that doesn't mean we can't have some fun with some optimizations, can we? Within graphics APIs we want to minimize context changes. The less we have to change textures/shaders/etc the better. So I made an image packer.

Basically: contents of resources/gfx are recursively crawled through and while doing this, the images contained within are loaded into memory. Sub-folders contained within gfx indicate an entity that may or may not contain animation data. If any of the images do contain data, the corresponding data will be within the resources/animations directory as a .toml file with the same name as the parent directory, indicating animation speed, number of columns, and number of rows.

This accomplishes:
1. All the images are loaded into one atlas image with a ~heavy~ emphasis on heuristics
2. The struct contained within this crate will have all the source rect information. This can be used to calculate the UV coordinates for the appropriate texels for the shader to draw, or my preferred method
