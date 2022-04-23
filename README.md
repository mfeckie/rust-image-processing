## About

## 🚴 Usage

Example using directly as a `module` in modern browsers

```html
  <script type="module">
    async function loadResizer() {
      const lib = await import('./pkg/rust_image_processing.js')
      await lib.default()
      return lib.resize_image
    }

    async function loadImage() {
      const image = await fetch("sample.png")
      return image.arrayBuffer()
    }

    const resize_image = await loadResizer()

    const buffer = await loadImage()

    const u8buf = new Uint8ClampedArray(buffer)

    const result = resize_image(u8buf, 256, 256)

    const img = document.createElement('img')
    img.src = URL.createObjectURL(new Blob([result.buffer]), { type: "image/jpeg" })
    document.querySelector('body').appendChild(img)
  </script>
```

### 🛠️ Build with `wasm-pack build`

This library is designed for use in web browsers, so building is targetted to `web`.  

```
wasm-pack build --target web
```

