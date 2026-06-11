import { StreamLanguage } from "@codemirror/language";
import type { StreamParser } from "@codemirror/language";

const words = (items: string[]) => new Set(items);

const keywords = words([
  "if", "else", "match", "true", "false", "let", "mut", "return",
  "break", "continue", "for", "while", "in"
]);

const declarations = words([
  "state", "fn", "struct", "enum", "setup", "update", "draw"
]);

const types = words([
  "number", "text", "bool", "Vec2", "Color", "Button", "Sprite"
]);

const constants = words([
  "Black", "White", "Gray", "Green", "Red", "Blue", "Yellow",
  "Magenta", "Cyan", "A", "B", "Left", "Right", "Up", "Down",
  "Start", "Select", "Title", "Playing", "Dead"
]);

const builtins = words([
  "clear", "line", "rect", "circle", "text", "sprite",
  "vec2", "pressed", "down", "released", "axis_x", "axis_y",
  "stick", "mouse_x", "mouse_y", "mouse_down", "dt",
  "min", "max", "clamp", "abs", "hit_rect"
]);

const parser: StreamParser<{}> = {
  name: "crs",

  token(stream) {
    if (stream.eatSpace()) return null;

    if (stream.sol() && stream.match("+++")) return "meta";

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
      stream.match(/(\.[0-9_]+)?/, true);
      return "number";
    }

    if (stream.match(/[A-Za-z_][A-Za-z0-9_]*/, true)) {
      const word = stream.current();

      if (word === "app" && stream.peek() === "!") {
        stream.next();
        return "meta";
      }

      if (/^\s*=/.test(stream.string.slice(stream.pos))) return "propertyName";
      if (declarations.has(word)) return "keyword";
      if (keywords.has(word)) return "keyword";
      if (types.has(word)) return "typeName";
      if (constants.has(word)) return "atom";
      if (builtins.has(word)) return "function(variableName)";

      const rest = stream.string.slice(stream.pos);
      if (/^\s*[:=]/.test(rest)) return "propertyName";

      return "variableName";
    }

    if (stream.match(/[{}[\](),:.]/, true)) return "punctuation";
    if (stream.match(/(==|!=|<=|>=|&&|\|\||\+=|-=|->|=>|[=+\-*/%<>!|&]+)/, true)) return "operator";

    stream.next();
    return null;
  },
};

export const crsLanguage = StreamLanguage.define(parser);
