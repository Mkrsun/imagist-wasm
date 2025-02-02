# imagist-wasm 🖼️🚀

📸 **Librería de procesamiento de imágenes en WebAssembly (WASM) con Rust**  
Optimiza imágenes con alta velocidad y bajo consumo de recursos. Compatible con **JPEG, PNG, WebP, BMP y HEIC**.

---

## 📦 Instalación

### Para Node.js:
```sh
npm install imagist-wasm
```

### Para React Native con Expo:
```sh
expo install imagist-wasm
```

---

## 🚀 Uso en Node.js

```javascript
const fs = require("fs");
const { resize_image } = require("imagist-wasm");

// 📥 Cargar imagen (Ejemplo: test_4k.jpg)
const imageData = fs.readFileSync("test_images/test_4k.jpg");

// 📏 Redimensionar a 1920x1080 y convertir a JPEG
const resizedImage = resize_image(imageData, 1920, 1080, "jpeg");

// 💾 Guardar la imagen optimizada
fs.writeFileSync("output/test_resized.jpg", resizedImage);

console.log("✅ Imagen procesada y guardada exitosamente.");
```

---

## 📱 Uso en React Native con Expo

```javascript
import { resize_image } from "imagist-wasm";
import * as FileSystem from "expo-file-system";

async function processImage(imageUri) {
  try {
    // 📥 Leer la imagen desde la URI del dispositivo
    const imageData = await FileSystem.readAsStringAsync(imageUri, {
      encoding: FileSystem.EncodingType.Base64,
    });

    // 📏 Redimensionar a 1080p y convertir a WebP
    const resizedImage = resize_image(Buffer.from(imageData, "base64"), 1920, 1080, "webp");

    // 💾 Guardar la imagen optimizada
    const newUri = `${FileSystem.cacheDirectory}resized.webp`;
    await FileSystem.writeAsStringAsync(newUri, Buffer.from(resizedImage).toString("base64"), {
      encoding: FileSystem.EncodingType.Base64,
    });

    console.log("✅ Imagen optimizada guardada en:", newUri);
    return newUri;
  } catch (error) {
    console.error("❌ Error procesando la imagen:", error);
  }
}
```

---

## 📌 Formatos Soportados

| Formato | Entrada | Salida |
|---------|---------|--------|
| **JPEG** | ✅ | ✅ |
| **PNG** | ✅ | ✅ |
| **WebP** | ✅ | ✅ |
| **BMP** | ✅ | ✅ |
| **HEIC** | ✅ (convertido a JPEG) | ❌ |

**Nota:** HEIC es soportado como entrada, pero siempre se convierte a JPEG al salir.

---

## 📖 API

```typescript
resize_image(imageData: Uint8Array, maxWidth: number, maxHeight: number, format: string): Uint8Array;
```

### **Parámetros:**
- `imageData: Uint8Array` – Buffer de la imagen de entrada.
- `maxWidth: number` – Ancho máximo deseado.
- `maxHeight: number` – Alto máximo deseado.
- `format: string` – Formato de salida (`jpeg`, `png`, `webp`, `bmp`, `heic`).

### **Retorno:**
- `Uint8Array` – Imagen procesada en el formato especificado.

---

## 🛠️ Ejemplos Adicionales

### 📍 Convertir PNG a WebP
```javascript
const input = fs.readFileSync("input.png");
const output = resize_image(input, 1024, 768, "webp");
fs.writeFileSync("output.webp", output);
```

### 📍 Procesar imagen HEIC (convertir a JPEG)
```javascript
const input = fs.readFileSync("photo.heic");
const output = resize_image(input, 1920, 1080, "heic"); // Se convierte a JPEG
fs.writeFileSync("output.jpg", output);
```

---

## 🛠️ Construcción Manual

Si deseas construir este paquete desde código fuente:
```sh
wasm-pack build --target nodejs
npm link
```

---

## 📄 Licencia
**MIT © 2025 - imagist-wasm**

🚀 ¡Listo para mejorar el rendimiento de tus imágenes con WebAssembly!  
Si tienes dudas o quieres sugerir mejoras, abre un **Issue** en GitHub. 🔥