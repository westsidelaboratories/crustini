import type PrismType from "prismjs";

export function registerCrsPrism(Prism: typeof PrismType) {
  Prism.languages.crs = {
    comment: /\/\/.*/,
    string: {
      pattern: /"(?:\\.|[^"\\])*"/,
      greedy: true,
    },
    macro: /\bapp!(?=\s*\{)/,
    number: /\b\d[\d_]*(?:\.\d[\d_]*)?(?:ms|%)?\b/,
    keyword: /\b(?:if|else|match|true|false|let|mut|return|break|continue|for|while|in|use|as)\b/,
    declaration: /\b(?:name|target|display|input|button|state|screen|draw|on|every|const|goto)\b/,
    type: /\b(?:u8|u16|u32|u64|usize|i8|i16|i32|i64|isize|bool|str|char|f32|f64|rgb565|rgb888|mono)\b/,
    constant: /\b(?:BLACK|WHITE|GRAY|GREEN|RED|BLUE|YELLOW|MAGENTA|CYAN|ON|OFF|bold_12|bold_16|big|small|landscape|portrait|esp32_s3|rp2040|stm32|st7789|ssd1306|sh1106|ili9341)\b/,
    builtin: /\b(?:clear|pixel|line|rect|fill_rect|circle|text|bitmap|sprite|flush|progress|menu|card|meter|toggle|slider|redraw|min|max|clamp)(?=\s*\()/,
    property: /\b[A-Za-z_][A-Za-z0-9_]*(?=\s*:)/,
    operator: /[=+\-*/%<>!|&]+/,
    punctuation: /[{}[\](),:.]/,
  };
}
