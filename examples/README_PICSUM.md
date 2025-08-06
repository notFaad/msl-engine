# 📸 Lorem Picsum Scraping Guide

This guide demonstrates how to use the **MSL Engine** to scrape images from [Lorem Picsum](https://picsum.photos/), a royalty-free image service.

## 🎯 What is Lorem Picsum?

Lorem Picsum is a free image placeholder service that provides:
- **Random Images**: Different image every time you visit
- **Specific Sizes**: Custom width/height combinations
- **Multiple Formats**: JPG, WebP support
- **Special Effects**: Grayscale, blur, and more
- **API Access**: JSON endpoints for programmatic access

## 📋 MSL Script Examples

### 1. Basic Image Scraping

```msl
# Simple random image
open "https://picsum.photos/800/600"
```

### 2. Multiple Image Sizes

```msl
# Scrape different image sizes
open "https://picsum.photos/400/300"
open "https://picsum.photos/800/600"
open "https://picsum.photos/1200/800"
```

### 3. Square Images

```msl
# Square images (same width and height)
open "https://picsum.photos/500"
open "https://picsum.photos/1000"
```

### 4. Specific Images by ID

```msl
# Get a specific image by ID
open "https://picsum.photos/id/237/600/400"
open "https://picsum.photos/id/870/800/600"
```

### 5. Special Effects

```msl
# Grayscale images
open "https://picsum.photos/400/300?grayscale"

# Blurred images
open "https://picsum.photos/500/500?blur=2"

# Combined effects
open "https://picsum.photos/600/400?grayscale&blur=3"
```

### 6. Different Formats

```msl
# JPG format (default)
open "https://picsum.photos/800/600.jpg"

# WebP format
open "https://picsum.photos/800/600.webp"
```

### 7. Seeded Random Images

```msl
# Same random image every time
open "https://picsum.photos/seed/picsum/400/300"
open "https://picsum.photos/seed/myseed/600/400"
```

## 🚀 Advanced Scraping Patterns

### Batch Image Collection

```msl
# Collect multiple images with different parameters
open "https://picsum.photos/400/300"
open "https://picsum.photos/400/300?grayscale"
open "https://picsum.photos/400/300?blur=2"
open "https://picsum.photos/400/300.webp"
```

### Image List Scraping

```msl
# Scrape the image list API
open "https://picsum.photos/v2/list"

click "a"
  set image_id = attr("href").split("/")[-1]
  set author = text
```

## 📁 File Organization

Organize your scraped images with descriptive folders:

```
picsum_images/
├── random/
│   ├── 400x300/
│   ├── 800x600/
│   └── square_500/
├── effects/
│   ├── grayscale/
│   ├── blurred/
│   └── combined/
├── formats/
│   ├── jpg/
│   └── webp/
└── specific/
    ├── id_237/
    └── seeded/
```

## 🛠️ Usage Commands

### Parse Scripts (Current Working)
```bash
# Parse a simple script
cargo run -- parse examples/picsum_working.msl

# Parse a complex script (when media parsing is fixed)
cargo run -- parse examples/picsum_comprehensive.msl
```

### Run Scripts (When Media Parsing is Fixed)
```bash
# Run a simple scraping script
cargo run -- run examples/picsum_simple.msl

# Run with verbose output
cargo run -- run examples/picsum_comprehensive.msl --verbose
```

## 📊 Lorem Picsum URL Patterns

| Pattern | Description | Example |
|---------|-------------|---------|
| `/{width}/{height}` | Random image | `https://picsum.photos/800/600` |
| `/{size}` | Square image | `https://picsum.photos/500` |
| `/id/{id}/{width}/{height}` | Specific image | `https://picsum.photos/id/237/600/400` |
| `/seed/{seed}/{width}/{height}` | Seeded random | `https://picsum.photos/seed/picsum/400/300` |
| `?grayscale` | Grayscale effect | `https://picsum.photos/400/300?grayscale` |
| `?blur={1-10}` | Blur effect | `https://picsum.photos/400/300?blur=3` |
| `.jpg` | JPG format | `https://picsum.photos/800/600.jpg` |
| `.webp` | WebP format | `https://picsum.photos/800/600.webp` |

## 🎨 Creative Scraping Ideas

### 1. Image Collections
```msl
# Create a collection of landscape images
open "https://picsum.photos/1200/800"
open "https://picsum.photos/1600/900"
open "https://picsum.photos/1920/1080"
```

### 2. Effect Variations
```msl
# Create variations of the same image
open "https://picsum.photos/seed/art/400/300"
open "https://picsum.photos/seed/art/400/300?grayscale"
open "https://picsum.photos/seed/art/400/300?blur=2"
```

### 3. Format Comparison
```msl
# Compare different formats
open "https://picsum.photos/800/600.jpg"
open "https://picsum.photos/800/600.webp"
```

## 🔧 Technical Notes

### Current MSL Engine Status
- ✅ **Basic Parsing**: `open` commands work perfectly
- ✅ **URL Handling**: Proper URL resolution and validation
- 🔄 **Media Parsing**: In development (needs refinement)
- 🔄 **Variable Templating**: Planned feature

### Lorem Picsum API Features
- **Rate Limiting**: No strict limits, but be respectful
- **Caching**: Images are cached for performance
- **Random Seeds**: Use seeds for reproducible results
- **Multiple Formats**: JPG and WebP support
- **Special Effects**: Grayscale, blur, and combinations

## 📝 Best Practices

1. **Respectful Scraping**: Don't overwhelm the service
2. **Organized Storage**: Use descriptive folder structures
3. **Format Selection**: Choose appropriate formats for your needs
4. **Seeded Images**: Use seeds for consistent results
5. **Effect Combinations**: Experiment with different effects

## 🎯 Next Steps

When the MSL Engine's media parsing is fully implemented, you'll be able to:

1. **Download Images**: Automatically save scraped images
2. **Filter by Type**: Extract specific image formats
3. **Variable Templating**: Use dynamic file naming
4. **Batch Processing**: Scrape multiple images efficiently
5. **Error Handling**: Robust error recovery and retry logic

---

*This guide demonstrates the power of the MSL Engine for scraping royalty-free images from Lorem Picsum. The engine provides a clean, declarative way to define scraping workflows.* 