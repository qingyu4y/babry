//! 内置 PP-OCRv6 tiny 模型；模型随二进制分发，不联网。
use crate::{core::RectPx, shm_image::ShmImage};
use oar_ocr::prelude::*;
use std::sync::{Mutex, OnceLock};

static DET: &[u8] = include_bytes!("../models/ocr/pp-ocrv6_tiny_det.onnx");
static REC: &[u8] = include_bytes!("../models/ocr/pp-ocrv6_tiny_rec.onnx");
static DICT: &str = include_str!("../models/ocr/ppocrv6_tiny_dict.txt");

const MIN_CONFIDENCE: f32 = 0.5;

fn engine() -> Result<&'static Mutex<OAROCR>, String> {
    static ENGINE: OnceLock<Result<Mutex<OAROCR>, String>> = OnceLock::new();
    ENGINE
        .get_or_init(|| {
            OAROCRBuilder::new(DET, REC, "")
                .character_dict_content(DICT)
                .build()
                .map(Mutex::new)
                .map_err(|error| format!("OCR 模型初始化失败: {error}"))
        })
        .as_ref()
        .map_err(Clone::clone)
}

fn crop_rgb(image: &ShmImage, rect: RectPx) -> Option<image::RgbImage> {
    let rect = rect.clamp_to(RectPx::new(0, 0, image.width(), image.height()));
    if rect.width <= 1 || rect.height <= 1 {
        return None;
    }
    let (width, height) = (rect.width as usize, rect.height as usize);
    let mut rgb = Vec::with_capacity(width * height * 3);
    let mut row = vec![0; width * 4];
    for y in 0..height {
        image.read_row(rect.x, rect.y + y as i32, &mut row);
        for pixel in row.chunks_exact(4) {
            rgb.extend_from_slice(&pixel[..3]);
        }
    }
    image::RgbImage::from_raw(rect.width, rect.height, rgb)
}

/// 按阅读顺序拼接高置信度文本行。
fn assemble(regions: impl IntoIterator<Item = (String, f32, f32, f32)>) -> Option<String> {
    let mut lines: Vec<(f32, f32, String)> = regions
        .into_iter()
        .filter(|(text, confidence, _, _)| !text.trim().is_empty() && *confidence >= MIN_CONFIDENCE)
        .map(|(text, _, y, x)| (y, x, text))
        .collect();
    if lines.is_empty() {
        return None;
    }
    lines.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
    Some(
        lines
            .into_iter()
            .map(|(_, _, text)| text)
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

pub fn recognize(image: &ShmImage, rect: RectPx) -> Result<String, String> {
    let image = crop_rgb(image, rect).ok_or("选区太小，无法识别文字")?;
    let engine = engine()?.lock().map_err(|_| "OCR 引擎不可用")?;
    let results = engine
        .predict(vec![image])
        .map_err(|error| format!("OCR 识别失败: {error}"))?;
    let regions = results.into_iter().next().map(|result| {
        result
            .text_regions
            .into_iter()
            .filter_map(|region| {
                let (text, confidence) = region.text_with_confidence()?;
                Some((
                    text.to_owned(),
                    confidence,
                    region.bounding_box.y_min(),
                    region.bounding_box.x_min(),
                ))
            })
            .collect::<Vec<_>>()
    });
    assemble(regions.unwrap_or_default()).ok_or_else(|| "未识别到文字".into())
}

#[cfg(test)]
mod tests {
    use super::assemble;

    #[test]
    fn assembles_by_reading_order_and_drops_weak_lines() {
        let text = assemble([
            ("第二行".into(), 0.9, 20.0, 10.0),
            ("噪声".into(), 0.3, 30.0, 10.0),
            ("第一行".into(), 0.8, 10.0, 10.0),
            ("".into(), 0.9, 40.0, 10.0),
        ]);
        assert_eq!(text.as_deref(), Some("第一行\n第二行"));
    }

    #[test]
    fn returns_none_when_nothing_is_confident() {
        assert_eq!(assemble([("噪声".into(), 0.2, 1.0, 1.0)]), None);
    }
}
