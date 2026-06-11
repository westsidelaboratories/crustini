import type { LanguageFn } from "highlight.js";

export const crsHighlightJs: LanguageFn = function(hljs) {
  return {
    name: "Crustini",
    aliases: ["flour", "crs", "crustini"],
    keywords: {
      keyword: "if else match true false let mut return break continue for while in",
      literal: "true false",
      built_in: "clear line rect circle text sprite vec2 pressed down released axis_x axis_y stick mouse_x mouse_y mouse_down dt min max clamp abs hit_rect",
      type: "number text bool Vec2 Color Button Sprite"
    },
    contains: [
      hljs.COMMENT("//", "$"),
      hljs.QUOTE_STRING_MODE,
      {
        className: "meta",
        begin: "^\\+\\+\\+$"
      },
      {
        className: "meta",
        begin: "\\bapp!"
      },
      {
        className: "keyword",
        begin: "\\b(?:state|fn|struct|enum|setup|update|draw)\\b"
      },
      {
        className: "number",
        begin: "\\b\\d[\\d_]*(?:\\.\\d[\\d_]*)?\\b"
      },
      {
        className: "attr",
        begin: "\\b[A-Za-z_][A-Za-z0-9_]*(?=\\s*[:=])"
      },
      {
        className: "symbol",
        begin: "\\b(?:Black|White|Gray|Green|Red|Blue|Yellow|Magenta|Cyan|A|B|Left|Right|Up|Down|Start|Select|Title|Playing|Dead)\\b"
      }
    ]
  };
};
