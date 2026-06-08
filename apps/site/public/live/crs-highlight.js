// Tiny .crs highlighter. No dependencies.

const CRS = (() => {
  const keywords = new Set([
    "if", "else", "match", "true", "false", "let", "mut", "return",
    "break", "continue", "for", "while", "in", "use", "as"
  ]);

  const declarations = new Set([
    "app", "name", "target", "display", "input", "button", "state",
    "screen", "fps", "setup", "update", "draw", "on", "every", "const", "goto"
  ]);

  const types = new Set([
    "u8", "u16", "u32", "u64", "usize",
    "i8", "i16", "i32", "i64", "isize",
    "bool", "str", "char", "f32", "f64",
    "rgb565", "rgb888", "mono"
  ]);

  const constants = new Set([
    "BLACK", "WHITE", "GRAY", "GREEN", "RED", "BLUE", "YELLOW",
    "MAGENTA", "CYAN", "ON", "OFF",
    "Main", "Run", "Wifi", "Info", "Settings",
    "bold_12", "bold_16", "big", "small",
    "landscape", "portrait",
    "esp32_s3", "rp2040", "stm32",
    "st7789", "ssd1306", "sh1106", "ili9341"
  ]);

  const builtins = new Set([
    "clear", "pixel", "line", "rect", "fill_rect", "circle",
    "text", "bitmap", "sprite", "flush",
    "progress", "menu", "card", "meter", "toggle", "slider",
    "redraw", "min", "max", "clamp"
  ]);

  const esc = (s) => s
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;");

  const span = (cls, s) => `<span class="${cls}">${esc(s)}</span>`;

  const isAlpha = (ch) => /[A-Za-z_]/.test(ch);
  const isDigit = (ch) => /[0-9]/.test(ch);
  const isIdent = (ch) => /[A-Za-z0-9_]/.test(ch);

  function highlightLine(line) {
    let out = "";
    let i = 0;

    while (i < line.length) {
      const ch = line[i];
      const next = line[i + 1];

      if (ch === "/" && next === "/") {
        out += span("crs-comment", line.slice(i));
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
        out += span("crs-string", line.slice(i, j));
        i = j;
        continue;
      }

      if (isDigit(ch)) {
        let j = i + 1;
        while (j < line.length && /[0-9_]/.test(line[j])) j++;
        if (line[j] === "." && isDigit(line[j + 1] || "")) {
          j++;
          while (j < line.length && /[0-9_]/.test(line[j])) j++;
        }
        if (line.slice(j, j + 2) === "ms") j += 2;
        if (line[j] === "%") j++;
        out += span("crs-number", line.slice(i, j));
        i = j;
        continue;
      }

      if (isAlpha(ch)) {
        let j = i + 1;
        while (j < line.length && isIdent(line[j])) j++;

        const word = line.slice(i, j);
        const rest = line.slice(j);

        if (rest.startsWith("!")) {
          out += span("crs-macro", `${word}!`);
          i = j + 1;
          continue;
        }

        if (declarations.has(word)) out += span("crs-declaration", word);
        else if (keywords.has(word)) out += span("crs-keyword", word);
        else if (types.has(word)) out += span("crs-type", word);
        else if (constants.has(word)) out += span("crs-constant", word);
        else if (builtins.has(word) || rest.trimStart().startsWith("(")) out += span("crs-builtin", word);
        else if (/^\s*:/.test(rest)) out += span("crs-property", word);
        else out += esc(word);

        i = j;
        continue;
      }

      if ("{}[](),:.".includes(ch)) out += span("crs-punctuation", ch);
      else if ("=+-*/%<>!|&".includes(ch)) out += span("crs-operator", ch);
      else out += esc(ch);

      i++;
    }

    return out || " ";
  }

  function highlight(source, lineNumbers = true) {
    const lines = String(source).replace(/\r\n/g, "\n").split("\n");

    if (!lineNumbers) {
      return lines.map(highlightLine).join("\n");
    }

    return lines.map((line, i) => {
      return `<span class="crs-line"><span class="crs-line-number">${i + 1}</span><span>${highlightLine(line)}</span></span>`;
    }).join("\n");
  }

  return { highlight };
})();
