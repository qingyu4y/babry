//! 工具栏图标以内嵌 SVG 为唯一来源，启动时解析一次并缓存。
use super::{Icon, IconShape};

const SOURCES: &[(&str, &str)] = &[
    ("grip", include_str!("../icons/v2/grip.svg")),
    (
        "shape-rectangle",
        include_str!("../icons/v2/shape-rectangle.svg"),
    ),
    ("shape-circle", include_str!("../icons/v2/shape-circle.svg")),
    ("shape-square", include_str!("../icons/v2/shape-square.svg")),
    ("shape-line", include_str!("../icons/v2/shape-line.svg")),
    ("shape-arrow", include_str!("../icons/v2/shape-arrow.svg")),
    ("pen", include_str!("../icons/v2/pen.svg")),
    ("mark", include_str!("../icons/v2/mark.svg")),
    ("number", include_str!("../icons/v2/number.svg")),
    ("effects", include_str!("../icons/v2/effects.svg")),
    ("blur", include_str!("../icons/v2/blur.svg")),
    ("eraser", include_str!("../icons/v2/eraser.svg")),
    ("highlight", include_str!("../icons/v2/highlight.svg")),
    ("undo", include_str!("../icons/v2/undo.svg")),
    ("redo", include_str!("../icons/v2/redo.svg")),
    ("pin", include_str!("../icons/v2/pin.svg")),
    ("save", include_str!("../icons/v2/save.svg")),
    ("cancel", include_str!("../icons/v2/cancel.svg")),
    ("copy", include_str!("../icons/v2/copy.svg")),
    ("longshot", include_str!("../icons/v2/longshot.svg")),
    (
        "longshot-stop",
        include_str!("../icons/v2/longshot-stop.svg"),
    ),
    ("ocr", include_str!("../icons/v2/ocr.svg")),
    ("translate", include_str!("../icons/v2/translate.svg")),
    ("chevron", include_str!("../icons/v2/chevron.svg")),
];

pub fn definitions() -> Vec<Icon> {
    SOURCES
        .iter()
        .filter_map(|(name, svg)| icon(name, svg))
        .collect()
}

fn icon(name: &'static str, svg: &'static str) -> Option<Icon> {
    let mut shapes = Vec::new();
    for tag in svg.split('<').skip(1) {
        let tag = tag.split('>').next()?;
        match tag.split_whitespace().next()? {
            "path" => shapes.push(IconShape::Path(attr(tag, "d")?, 2.0)),
            "rect" => shapes.push(IconShape::Rect(
                number(tag, "x")?,
                number(tag, "y")?,
                number(tag, "width")?,
                number(tag, "height")?,
                number(tag, "rx").unwrap_or(0.0),
                2.0,
            )),
            "circle" => shapes.push(IconShape::Circle(
                number(tag, "cx")?,
                number(tag, "cy")?,
                number(tag, "r")?,
                2.0,
            )),
            _ => {}
        }
    }
    Some(Icon::new(name, &shapes))
}

fn attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let rest = tag.split_once(&format!(" {name}=\""))?.1;
    rest.split('"').next()
}

fn number(tag: &str, name: &str) -> Option<f32> {
    attr(tag, name)?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_toolbar_icon_parses_with_strokes() {
        let icons = definitions();
        assert_eq!(icons.len(), SOURCES.len());
        for (name, _) in SOURCES {
            let icon = icons.iter().find(|icon| icon.name == *name).unwrap();
            assert!(!icon.paths.is_empty(), "{name} has no geometry");
            assert!(
                icon.paths.iter().all(|(_, width)| *width > 0.0),
                "{name} has a filled path"
            );
        }
    }
}
