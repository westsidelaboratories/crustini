import { StreamLanguage } from "@codemirror/language";
import type { StreamParser } from "@codemirror/language";

const words = (items: string[]) => new Set(items);

const keywords = words([
  "if", "else", "match", "true", "false", "let", "mut", "return",
  "break", "continue", "for", "while", "in", "use", "as"
]);

const declarations = words([
  "app", "name", "target", "display", "input", "button", "state",
  "screen", "fps", "setup", "update", "draw", "on", "every", "const", "goto"
]);

const types = words([
  "u8", "u16", "u32", "u64", "usize",
  "i8", "i16", "i32", "i64", "isize",
  "bool", "str", "char", "f32", "f64",
  "rgb565", "rgb888", "mono"
]);

const constants = words([
  "BLACK", "WHITE", "GRAY", "GREEN", "RED", "BLUE", "YELLOW",
  "MAGENTA", "CYAN", "ON", "OFF",
  "bold_12", "bold_16", "big", "small",
  "landscape", "portrait", "esp32_s3", "rp2040", "stm32",
  "st7789", "ssd1306", "sh1106", "ili9341"
]);

const builtins = words([
  "clear", "pixel", "line", "rect", "fill_rect", "circle",
  "text", "bitmap", "sprite", "flush",
  "progress", "menu", "card", "meter", "toggle", "slider",
  "redraw", "min", "max", "clamp"
]);

const parser: StreamParser<{}> = {
  name: "crs",

  token(stream) {
    if (stream.eatSpace()) return null;

    if (stream.match("//")) {
      stream.skipToEnd();
      return "comment";
    }

    if (stream.peek() === '"') {
      stream.next();
      let escaped = false;
      while (!stream.eol()) {
        const ch = stream.next();
        if (ch === '"' && !escaped) break;
        escaped = ch === "\\" && !escaped;
        if (ch !== "\\") escaped = false;
      }
      return "string";
    }

    if (stream.match(/[0-9][0-9_]*/, true)) {
      stream.match(/(\.[0-9_]+)?(ms|%)?/, true);
      return "number";
    }

    if (stream.match(/[A-Za-z_][A-Za-z0-9_]*/, true)) {
      const word = stream.current();

      if (word === "app" && stream.peek() === "!") {
        stream.next();
        return "meta";
      }

      if (declarations.has(word)) return "keyword";
      if (keywords.has(word)) return "keyword";
      if (types.has(word)) return "typeName";
      if (constants.has(word)) return "atom";
      if (builtins.has(word)) return "function(variableName)";

      const rest = stream.string.slice(stream.pos);
      if (/^\s*:/.test(rest)) return "propertyName";

      return "variableName";
    }

    if (stream.match(/[{}[\](),:.]/, true)) return "punctuation";
    if (stream.match(/[=+\-*/%<>!|&]+/, true)) return "operator";

    stream.next();
    return null;
  },
};

export const crsLanguage = StreamLanguage.define(parser);
