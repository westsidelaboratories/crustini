export type CrsTokenKind =
  | "space"
  | "comment"
  | "string"
  | "number"
  | "macro"
  | "keyword"
  | "declaration"
  | "type"
  | "constant"
  | "builtin"
  | "property"
  | "identifier"
  | "punctuation"
  | "operator";

export type CrsToken = {
  kind: CrsTokenKind;
  value: string;
};

export const CRS_KEYWORDS = new Set([
  "if", "else", "match", "true", "false", "let", "mut", "return",
  "break", "continue", "for", "while", "in", "use", "as"
]);

export const CRS_DECLARATIONS = new Set([
  "name", "target", "display", "input", "button", "state",
  "screen", "draw", "on", "every", "const", "goto"
]);

export const CRS_TYPES = new Set([
  "u8", "u16", "u32", "u64", "usize",
  "i8", "i16", "i32", "i64", "isize",
  "bool", "str", "char", "f32", "f64",
  "rgb565", "rgb888", "mono"
]);

export const CRS_CONSTANTS = new Set([
  "BLACK", "WHITE", "GRAY", "GREEN", "RED", "BLUE", "YELLOW",
  "MAGENTA", "CYAN", "ON", "OFF",
  "Main", "Run", "Wifi", "Info", "Settings",
  "bold_12", "bold_16", "big", "small",
  "landscape", "portrait",
  "esp32_s3", "rp2040", "stm32",
  "st7789", "ssd1306", "sh1106", "ili9341"
]);

export const CRS_BUILTINS = new Set([
  "clear", "pixel", "line", "rect", "fill_rect", "circle",
  "text", "bitmap", "sprite", "flush",
  "progress", "menu", "card", "meter", "toggle", "slider",
  "redraw", "min", "max", "clamp"
]);

const isAlpha = (ch: string) => /[A-Za-z_]/.test(ch);
const isDigit = (ch: string) => /[0-9]/.test(ch);
const isIdent = (ch: string) => /[A-Za-z0-9_]/.test(ch);
const isSpace = (ch: string) => /\s/.test(ch);

export function tokenizeCrsLine(line: string): CrsToken[] {
  const tokens: CrsToken[] = [];
  let i = 0;

  while (i < line.length) {
    const ch = line[i];
    const next = line[i + 1];

    if (isSpace(ch)) {
      let j = i + 1;
      while (j < line.length && isSpace(line[j])) j++;
      tokens.push({ kind: "space", value: line.slice(i, j) });
      i = j;
      continue;
    }

    if (ch === "/" && next === "/") {
      tokens.push({ kind: "comment", value: line.slice(i) });
      break;
    }

    if (ch === '"') {
      let j = i + 1;
      while (j < line.length) {
        if (line[j] === "\\" && j + 1 < line.length) {
          j += 2;
          continue;
        }
        if (line[j] === '"') {
          j++;
          break;
        }
        j++;
      }
      tokens.push({ kind: "string", value: line.slice(i, j) });
      i = j;
      continue;
    }

    if (isDigit(ch)) {
      let j = i + 1;
      while (j < line.length && /[0-9_]/.test(line[j])) j++;

      if (line[j] === "." && isDigit(line[j + 1] ?? "")) {
        j++;
        while (j < line.length && /[0-9_]/.test(line[j])) j++;
      }

      if (line.slice(j, j + 2) === "ms") j += 2;
      if (line[j] === "%") j++;

      tokens.push({ kind: "number", value: line.slice(i, j) });
      i = j;
      continue;
    }

    if (isAlpha(ch)) {
      let j = i + 1;
      while (j < line.length && isIdent(line[j])) j++;

      const word = line.slice(i, j);
      const rest = line.slice(j);

      if (word === "app" && rest.startsWith("!")) {
        tokens.push({ kind: "macro", value: "app!" });
        i = j + 1;
        continue;
      }

      if (CRS_DECLARATIONS.has(word)) {
        tokens.push({ kind: "declaration", value: word });
        i = j;
        continue;
      }

      if (CRS_KEYWORDS.has(word)) {
        tokens.push({ kind: "keyword", value: word });
        i = j;
        continue;
      }

      if (CRS_TYPES.has(word)) {
        tokens.push({ kind: "type", value: word });
        i = j;
        continue;
      }

      if (CRS_CONSTANTS.has(word)) {
        tokens.push({ kind: "constant", value: word });
        i = j;
        continue;
      }

      if (CRS_BUILTINS.has(word) || rest.trimStart().startsWith("(")) {
        tokens.push({ kind: "builtin", value: word });
        i = j;
        continue;
      }

      if (/^\s*:/.test(rest)) {
        tokens.push({ kind: "property", value: word });
        i = j;
        continue;
      }

      tokens.push({ kind: "identifier", value: word });
      i = j;
      continue;
    }

    if ("{}[](),:.".includes(ch)) {
      tokens.push({ kind: "punctuation", value: ch });
      i++;
      continue;
    }

    if ("=+-*/%<>!|&".includes(ch)) {
      tokens.push({ kind: "operator", value: ch });
      i++;
      continue;
    }

    tokens.push({ kind: "punctuation", value: ch });
    i++;
  }

  return tokens;
}

export function tokenizeCrs(source: string): CrsToken[][] {
  return source.split("\n").map(tokenizeCrsLine);
}
