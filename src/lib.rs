use wasm_bindgen::prelude::*;
use image::{DynamicImage, ImageEncoder, ExtendedColorType};
use image::codecs::{jpeg::JpegEncoder, png::PngEncoder, webp::WebPEncoder, bmp::BmpEncoder};
use rayon::prelude::*;
use std::io::Cursor;
use libheif_rs::{HeifContext, LibHeif, ColorSpace, RgbChroma};

#[wasm_bindgen]
pub fn resize_image(image_data: &[u8], max_width: u32, max_height: u32, format: &str) -> Vec<u8> {
    let img = match format {
        "heic" => match decode_heic(image_data) {
            Ok(img) => img,
            Err(e) => panic!("Error al decodificar HEIC: {}", e),
        },
        _ => match image::load_from_memory(image_data) {
            Ok(img) => img,
            Err(_) => panic!("Formato no soportado: {}", format),
        },
    };

    let resized = if img.width() > max_width || img.height() > max_height {
        img.resize(max_width, max_height, image::imageops::FilterType::Lanczos3)
    } else {
        img
    };

    let raw_image = resized.to_rgb8();
    let raw_bytes: Vec<u8> = raw_image.as_raw().par_iter().cloned().collect();
    let (new_width, new_height) = raw_image.dimensions();

    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);

    let target_format = if format == "heic" { "jpeg" } else { format };

    match target_format {
        "jpeg" | "jpg" => {
            let encoder = JpegEncoder::new_with_quality(&mut cursor, 60);
            encoder.write_image(&raw_bytes, new_width, new_height, ExtendedColorType::Rgb8).unwrap();
        }
        "png" => {
            let encoder = PngEncoder::new(&mut cursor);
            encoder.write_image(&raw_bytes, new_width, new_height, ExtendedColorType::Rgb8).unwrap();
        }
        "webp" => {
            let encoder = WebPEncoder::new_lossless(&mut cursor);
            encoder.write_image(&raw_bytes, new_width, new_height, ExtendedColorType::Rgb8).unwrap();
        }
        "bmp" => {
            let encoder = BmpEncoder::new(&mut cursor);
            encoder.write_image(&raw_bytes, new_width, new_height, ExtendedColorType::Rgb8).unwrap();
        }
        _ => panic!("Formato no soportado: {}", format),
    }

    buf
}

// 📝 Función corregida para decodificar HEIC con `libheif-rs v1.1.0`
fn decode_heic(image_data: &[u8]) -> Result<DynamicImage, String> {
    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_bytes(image_data)
        .map_err(|e| format!("Error al leer HEIC: {}", e))?;

    let img_handle = ctx.primary_image_handle()
        .map_err(|e| format!("No se encontró la imagen principal en el HEIC: {}", e))?;

    let decoded_image = lib_heif.decode(
        &img_handle,
        ColorSpace::Rgb(RgbChroma::Rgb), // ✅ Nueva sintaxis
        None,
    ).map_err(|e| format!("Error al decodificar HEIC: {}", e))?;

    let width = decoded_image.width();
    let height = decoded_image.height();
    let planes = decoded_image.planes();

    let interleaved_plane = planes.interleaved.ok_or("No se encontró el canal de color RGB")?;
    let data = interleaved_plane.data;

    // Convertimos el buffer en `DynamicImage`
    let img = image::ImageBuffer::from_raw(width, height, data.to_vec())
        .ok_or_else(|| "Error al crear ImageBuffer desde datos HEIC".to_string())?;

    Ok(DynamicImage::ImageRgb8(img))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageReader, GenericImageView};
    use std::fs::File;
    use std::io::{Cursor, Read};

    fn load_test_image(path: &str) -> Vec<u8> {
        let mut file = File::open(path).expect("Failed to open test image");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read image file");
        buffer
    }

    #[test]
    fn test_resize_image_reduces_large_images() {
        let image_data = load_test_image("test_images/test_4k.jpg");

        let output = resize_image(&image_data, 1920, 1080, "jpeg");
        assert!(!output.is_empty(), "La imagen procesada no debe estar vacía");

        assert!(output.len() < image_data.len(), "La imagen optimizada debe ser más pequeña que la original");
    }

    #[test]
    fn test_resize_image_keeps_hd_images() {
        let image_data = load_test_image("test_images/test_hd.jpg");

        let output = resize_image(&image_data, 1920, 1080, "jpeg");
        assert!(!output.is_empty(), "La imagen procesada no debe estar vacía");

        let original_size = image_data.len();
        let processed_size = output.len();

        println!(
            "Tamaño original: {} bytes, Tamaño procesado: {} bytes",
            original_size, processed_size
        );

        let min_size = (original_size as f32 * 0.85) as usize;
        let max_size = (original_size as f32 * 1.20) as usize;

        println!("Tolerancia mínima: {} bytes, máxima: {} bytes", min_size, max_size);

        assert!(
            processed_size >= min_size && processed_size <= max_size,
            "La imagen HD debe mantener un tamaño similar después de la optimización"
        );
    }

    #[test]
    fn test_output_is_valid_jpeg() {
        let image_data = load_test_image("test_images/test_4k.jpg");

        let output = resize_image(&image_data, 1920, 1080, "jpeg");
        let cursor = Cursor::new(output);

        let img_result = ImageReader::new(cursor)
            .with_guessed_format()
            .expect("Failed to read image format")
            .decode();

        assert!(img_result.is_ok(), "La imagen resultante debe ser un JPEG válido");
    }

    #[test]
    fn test_resize_image_handles_extremely_large_images() {
        let image_data = load_test_image("test_images/test_8k.jpg");

        let output = resize_image(&image_data, 1920, 1080, "jpeg");
        assert!(!output.is_empty(), "La imagen procesada no debe estar vacía");

        let img_result = ImageReader::new(Cursor::new(output))
            .with_guessed_format()
            .expect("Failed to read image format")
            .decode();

        assert!(img_result.is_ok(), "La imagen 8K debe procesarse sin errores");
    }

    #[test]
    fn test_resize_image_converts_non_rgb_images() {
        let image_data = load_test_image("test_images/test_grayscale.jpg");

        let output = resize_image(&image_data, 1920, 1080, "jpeg");
        assert!(!output.is_empty(), "La imagen procesada no debe estar vacía");

        let img_result = ImageReader::new(Cursor::new(output))
            .with_guessed_format()
            .expect("Failed to read image format")
            .decode();

        assert!(img_result.is_ok(), "La imagen en escala de grises debe convertirse correctamente a RGB");
    }

    #[test]
    fn test_output_is_valid_webp() {
        let image_data = load_test_image("test_images/test_4k.jpg");

        let output = resize_image(&image_data, 1920, 1080, "webp");
        let cursor = Cursor::new(output);

        let img_result = ImageReader::new(cursor)
            .with_guessed_format()
            .expect("Failed to read image format")
            .decode();

        assert!(img_result.is_ok(), "La imagen resultante debe ser un WebP válido");
    }

    #[test]
    fn test_output_is_valid_bmp() {
        let image_data = load_test_image("test_images/test_4k.jpg");

        let output = resize_image(&image_data, 1920, 1080, "bmp");
        let cursor = Cursor::new(output);

        let img_result = ImageReader::new(cursor)
            .with_guessed_format()
            .expect("Failed to read image format")
            .decode();

        assert!(img_result.is_ok(), "La imagen resultante debe ser un BMP válido");
    }

    #[test]
    fn test_output_is_valid_heic() {
        let image_data = load_test_image("test_images/test_heic.heic");

        let output = resize_image(&image_data, 1920, 1080, "heic");
        let cursor = Cursor::new(output);

        let img_result = ImageReader::new(cursor)
            .with_guessed_format()
            .expect("Failed to read HEIC image format")
            .decode();

        assert!(img_result.is_ok(), "La imagen HEIC debe procesarse y convertirse correctamente a JPEG");
    }

    #[test]
    fn test_resized_image_has_correct_dimensions() {
        let image_data = load_test_image("test_images/test_4k.jpg");

        let output = resize_image(&image_data, 1920, 1080, "jpeg");
        let cursor = Cursor::new(output);

        let img = ImageReader::new(cursor)
            .with_guessed_format()
            .expect("Failed to read image format")
            .decode()
            .expect("Failed to decode resized image");

        let (width, height) = img.dimensions();
        assert!(width <= 1920 && height <= 1080, "Las dimensiones de la imagen deben estar dentro del límite");
    }
}
