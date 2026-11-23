# 工具栏图标 v2

24×24 线性 SVG 图标。统一 2px 描边、圆角端点与圆角连接，颜色 `#212121`；
图形保持在 2px 安全边距内，主要结构对齐 4/8px 网格，无填充、背景、渐变和阴影。

应用通过 `include_str!` 内嵌本目录的 SVG，并在绘制时按界面主题着色；修改图标后重新编译即可生效。

| 图标 | 文件 | 用途 |
| --- | --- | --- |
| `grip` | [`grip.svg`](grip.svg) | 拖动工具栏 |
| `shape-rectangle` | [`shape-rectangle.svg`](shape-rectangle.svg) | 矩形 |
| `shape-circle` | [`shape-circle.svg`](shape-circle.svg) | 圆形 |
| `shape-square` | [`shape-square.svg`](shape-square.svg) | 方形 |
| `shape-line` | [`shape-line.svg`](shape-line.svg) | 直线 |
| `shape-arrow` | [`shape-arrow.svg`](shape-arrow.svg) | 箭头 |
| `pen` | [`pen.svg`](pen.svg) | 画笔 |
| `mark` | [`mark.svg`](mark.svg) | 文字 |
| `number` | [`number.svg`](number.svg) | 序号标记 |
| `effects` | [`effects.svg`](effects.svg) | 效果菜单 |
| `blur` | [`blur.svg`](blur.svg) | 高斯模糊 |
| `eraser` | [`eraser.svg`](eraser.svg) | 橡皮擦 |
| `highlight` | [`highlight.svg`](highlight.svg) | 荧光笔 |
| `undo` | [`undo.svg`](undo.svg) | 撤销 |
| `redo` | [`redo.svg`](redo.svg) | 重做 |
| `pin` | [`pin.svg`](pin.svg) | 贴图 |
| `save` | [`save.svg`](save.svg) | 保存 |
| `cancel` | [`cancel.svg`](cancel.svg) | 取消截图 |
| `copy` | [`copy.svg`](copy.svg) | 复制 |
| `longshot` | [`longshot.svg`](longshot.svg) | 长截图 |
| `longshot-stop` | [`longshot-stop.svg`](longshot-stop.svg) | 停止长截图 |
| `ocr` | [`ocr.svg`](ocr.svg) | 识别文字 |
| `translate` | [`translate.svg`](translate.svg) | 翻译 |
| `chevron` | [`chevron.svg`](chevron.svg) | 下拉箭头 |

## grip

拖动工具栏。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <circle cx="8" cy="4" r="1"/>
  <circle cx="16" cy="4" r="1"/>
  <circle cx="8" cy="12" r="1"/>
  <circle cx="16" cy="12" r="1"/>
  <circle cx="8" cy="20" r="1"/>
  <circle cx="16" cy="20" r="1"/>
</svg>
```

## shape-rectangle

矩形。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <rect x="4" y="6" width="16" height="12" rx="3"/>
</svg>
```

## shape-circle

圆形。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <circle cx="12" cy="12" r="8"/>
</svg>
```

## shape-square

方形。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <rect x="4" y="4" width="16" height="16" rx="3"/>
</svg>
```

## shape-line

直线。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M4 20 20 4"/>
  <circle cx="4" cy="20" r="1"/>
  <circle cx="20" cy="4" r="1"/>
</svg>
```

## shape-arrow

箭头。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M4 20 20 4M10 4h10v10"/>
</svg>
```

## pen

画笔。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M5 19l1-4L15 6a2.5 2.5 0 0 1 3.5 3.5L10 18l-5 1Z"/>
  <path d="M13 8l3 3"/>
</svg>
```

## mark

文字。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M4 4h16M12 4v14M8 20h8"/>
</svg>
```

## number

序号标记。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <circle cx="12" cy="12" r="8"/>
  <path d="M10 9l2-1v9M10 17h4"/>
</svg>
```

## effects

效果菜单。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <rect x="6" y="6" width="4" height="4" rx="1.5"/>
  <rect x="14" y="6" width="4" height="4" rx="1.5"/>
  <rect x="6" y="14" width="4" height="4" rx="1.5"/>
  <rect x="14" y="14" width="4" height="4" rx="1.5"/>
</svg>
```

## blur

高斯模糊。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M4 8c2 0 2-2 4-2s2 2 4 2 2-2 4-2 2 2 4 2"/>
  <path d="M4 12c2 0 2-2 4-2s2 2 4 2 2-2 4-2 2 2 4 2"/>
  <path d="M4 16c2 0 2-2 4-2s2 2 4 2 2-2 4-2 2 2 4 2"/>
</svg>
```

## eraser

橡皮擦。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M6 15l8-8a2.5 2.5 0 0 1 3.5 3.5l-8 8H6v-3.5Z"/>
  <path d="M10 11l4 4M4 21h16"/>
</svg>
```

## highlight

荧光笔。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M14 4l6 6-9 9H5v-5l9-10Z"/>
  <path d="M12 6l6 6M4 21h16"/>
</svg>
```

## undo

撤销。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M9 14 4 9l5-5"/>
  <path d="M4 9h9a6 6 0 0 1 0 12h-3"/>
</svg>
```

## redo

重做。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M15 14l5-5-5-5"/>
  <path d="M20 9h-9a6 6 0 0 0 0 12h3"/>
</svg>
```

## pin

贴图。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M9 4h6l-.5 5L18 12.5H6L9.5 9 9 4Z"/>
  <path d="M12 12.5V20M8 20h8"/>
</svg>
```

## save

保存。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M5 4h10l5 5v10a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1Z"/>
  <path d="M8 4v4h6V4M8 20v-5h8v5"/>
</svg>
```

## cancel

取消截图。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <circle cx="12" cy="12" r="8"/>
  <path d="M8.5 8.5l7 7m0-7-7 7"/>
</svg>
```

## copy

复制。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <rect x="9" y="9" width="11" height="11" rx="2"/>
  <path d="M15 5H7a2 2 0 0 0-2 2v8"/>
</svg>
```

## longshot

长截图，上下双向箭头。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M12 6v12M8 10l4-4 4 4M8 14l4 4 4-4"/>
</svg>
```

## longshot-stop

停止长截图。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <rect x="4" y="4" width="16" height="16" rx="3"/>
  <rect x="10" y="10" width="4" height="4" rx="1"/>
</svg>
```

## ocr

识别文字。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M4 10V6a2 2 0 0 1 2-2h4M14 4h4a2 2 0 0 1 2 2v4M20 14v4a2 2 0 0 1-2 2h-4M10 20H6a2 2 0 0 1-2-2v-4"/>
  <path d="M8 9h8M8 12h6M8 15h4"/>
</svg>
```

## translate

翻译，双向箭头。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M6 9h14m-3-3 3 3-3 3"/>
  <path d="M20 15H6m3-3-3 3 3 3"/>
</svg>
```

## chevron

下拉箭头。

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#212121" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M4 8h16L12 18 4 8Z"/>
</svg>
```
