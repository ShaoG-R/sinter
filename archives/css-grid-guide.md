---
id: "5"
title: "A Complete Guide to CSS Grid"
slug: "css-grid-guide"
date: "2023-07-22"
tags: ["web", "css", "layout"]
summary: "Mastering the most powerful layout system in CSS."
---

# CSS Grid Layout

CSS Grid Layout excels at dividing a page into major regions or defining the relationship in terms of size, position, and layer, between parts of a control built from HTML primitives.

## Basic Concepts

- **Grid Container**: The element on which `display: grid` is applied.
- **Grid Item**: The children of the grid container.
- **Grid Line**: The dividing lines that make up the structure of the grid.

## Example

```css
.container {
  display: grid;
  grid-template-columns: 1fr 200px;
  gap: 20px;
}
```

Grid allows for two-dimensional layouts, handling both columns and rows simultaneously, unlike Flexbox which is largely one-dimensional.

> Flexbox is for layout in one dimension. Grid is for layout in two dimensions.
