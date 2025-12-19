---
id: "4"
title: "Optimizing Images for the Web"
slug: "optimizing-images"
date: "2023-08-10"
tags: ["web", "performance", "images"]
summary: "Techniques and formats for delivering high-quality images with low footprint."
---

# Optimizing Images for the Web

Images often account for the majority of the downloaded bytes for a web page. Optimizing them is crucial for performance.

## Formats

1. **WebP**: Superior compression for both lossy and lossless images.
2. **AVIF**: Next-generation format with even better compression than WebP.
3. **JPEG XL**: A promising new format for high-fidelity images.

## Techniques

- **Lazy Loading**: Use `loading="lazy"` to defer loading of off-screen images.
- **Responsive Images**: Use `srcset` and `sizes` to serve the right image size for the device.

![Optimization Chart](https://upload.wikimedia.org/wikipedia/commons/thumb/b/b2/JPEG_compression_Example.jpg/800px-JPEG_compression_Example.jpg)

Remember, the fastest request is the one that's never made!
