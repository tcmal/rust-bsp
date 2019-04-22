# bsp

Library for reading `.bsp` files.

# Lumps

Unfinished:

```
digraph G {
    "Brush" -> "Texture"
    "Brush" -> "BrushSide"
    "BrushSide" -> "Plane"
    "BrushSide" -> "Texture"
    "Effect" -> "Brush"
    "Entity" -> ""
    "Face" -> "Texture"
    "Face" -> "Effect"
    "Face" -> "Vertex"
    "Face" -> "MeshVert"
    "Face" -> "Lightmap"
    "LightVol" -> ""
    "BSPTree" -> "BSPNode"
    "BSPNode" -> "BSPLeaf"
    "BSPLeaf" -> "Face"
    "BSPLeaf" -> "Brush"
}
```

# TODO
    - Models
    - VisData