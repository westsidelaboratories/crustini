import type { LanguageFn } from "highlight.js";

export const crsHighlightJs: LanguageFn = function(hljs) {
  return {
    name: "Crustini",
    aliases: ["crs", "crustini"],
    keywords: {
      keyword: "if else match true false let mut return break continue for while in use as",
      literal: "true false",
      built_in: "clear pixel line rect fill_rect circle text bitmap sprite flush progress menu card meter toggle slider redraw min max clamp",
      type: "u8 u16 u32 u64 usize i8 i16 i32 i64 isize bool str char f32 f64 rgb565 rgb888 mono"
    },
    contains: [
      hljs.COMMENT("//", "$"),
      hljs.QUOTE_STRING_MODE,
      {
        className: "meta",
        begin: "\\b[A-Za-z_][A-Za-z0-9_]*!"
      },
      {
        className: "keyword",
        begin: "\\b(?:app|name|target|display|input|button|state|screen|fps|setup|update|draw|on|every|const|goto)\\b"
      },
      {
        className: "number",
        begin: "\\b\\d[\\d_]*(?:\\.\\d[\\d_]*)?(?:ms|%)?\\b"
      },
      {
        className: "attr",
        begin: "\\b[A-Za-z_][A-Za-z0-9_]*(?=\\s*:)"
      },
      {
        className: "symbol",
        begin: "\\b(?:BLACK|WHITE|GRAY|GREEN|RED|BLUE|YELLOW|MAGENTA|CYAN|ON|OFF|bold_12|bold_16|big|small|landscape|portrait|esp32_s3|rp2040|stm32|st7789|ssd1306|sh1106|ili9341)\\b"
      }
    ]
  };
};
