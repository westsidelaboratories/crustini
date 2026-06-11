/* Crustini .flour syntax highlighter
   No dependencies after bundling. Browser + plain script usage.

   Usage:
   <link rel="stylesheet" href="themes/crustini-dark.css">
   <script src="dist/crustini-highlight.js"></script>
   <pre class="crs-code"><code class="language-flour">app! Main { ... }</code></pre>
   <script>CrustiniHighlight.highlightAll();</script>
*/
(() => {

  // src/crs-tokenizer.ts
  var CRS_KEYWORDS = new Set([
    "if",
    "else",
    "match",
    "true",
    "false",
    "let",
    "mut",
    "return",
    "break",
    "continue",
    "for",
    "while",
    "in"
  ]);
  var CRS_DECLARATIONS = new Set([
    "state",
    "fn",
    "struct",
    "enum",
    "setup",
    "update",
    "draw"
  ]);
  var CRS_TYPES = new Set([
    "number",
    "text",
    "bool",
    "Vec2",
    "Color",
    "Button",
    "Sprite"
  ]);
  var CRS_CONSTANTS = new Set([
    "Black",
    "White",
    "Gray",
    "Green",
    "Red",
    "Blue",
    "Yellow",
    "Magenta",
    "Cyan",
    "A",
    "B",
    "Left",
    "Right",
    "Up",
    "Down",
    "Start",
    "Select",
    "Title",
    "Playing",
    "Dead"
  ]);
  var CRS_BUILTINS = new Set([
    "clear",
    "line",
    "rect",
    "circle",
    "text",
    "sprite",
    "vec2",
    "pressed",
    "down",
    "released",
    "axis_x",
    "axis_y",
    "stick",
    "mouse_x",
    "mouse_y",
    "mouse_down",
    "dt",
    "min",
    "max",
    "clamp",
    "abs",
    "hit_rect"
  ]);
  var isAlpha = (ch) => /[A-Za-z_]/.test(ch);
  var isDigit = (ch) => /[0-9]/.test(ch);
  var isIdent = (ch) => /[A-Za-z0-9_]/.test(ch);
  var isSpace = (ch) => /\s/.test(ch);
  function tokenizeCrsLine(line) {
    const tokens = [];
    let i = 0;
    while (i < line.length) {
      const ch = line[i];
      const next = line[i + 1];
      if (isSpace(ch)) {
        let j = i + 1;
        while (j < line.length && isSpace(line[j]))
          j++;
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
        while (j < line.length && /[0-9_]/.test(line[j]))
          j++;
        if (line[j] === "." && isDigit(line[j + 1] ?? "")) {
          j++;
          while (j < line.length && /[0-9_]/.test(line[j]))
            j++;
        }
        if (line.slice(j, j + 2) === "ms")
          j += 2;
        if (line[j] === "%")
          j++;
        tokens.push({ kind: "number", value: line.slice(i, j) });
        i = j;
        continue;
      }
      if (isAlpha(ch)) {
        let j = i + 1;
        while (j < line.length && isIdent(line[j]))
          j++;
        const word = line.slice(i, j);
        const rest = line.slice(j);
        if (word === "app" && rest.startsWith("!")) {
          tokens.push({ kind: "compiler", value: `${word}!` });
          i = j + 1;
          continue;
        }
        if (rest.startsWith("!")) {
          tokens.push({ kind: "identifier", value: word });
          i = j;
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
        if (/^\s*[:=]/.test(rest)) {
          tokens.push({ kind: "property", value: word });
          i = j;
          continue;
        }
        tokens.push({ kind: "identifier", value: word });
        i = j;
        continue;
      }
      const two = line.slice(i, i + 2);
      if (["::", "==", "!=", "<=", ">=", "&&", "||", "+=", "-=", "->", "=>"].includes(two)) {
        tokens.push({ kind: two === "::" ? "punctuation" : "operator", value: two });
        i += 2;
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
  function tokenizeCrs(source) {
    return source.split(`
`).map(tokenizeCrsLine);
  }

  // src/html.ts
  var CLASS_BY_KIND = {
    space: "",
    comment: "crs-comment",
    string: "crs-string",
    number: "crs-number",
    compiler: "crs-compiler",
    keyword: "crs-keyword",
    declaration: "crs-declaration",
    type: "crs-type",
    constant: "crs-constant",
    builtin: "crs-builtin",
    property: "crs-property",
    identifier: "crs-identifier",
    punctuation: "crs-punctuation",
    operator: "crs-operator"
  };
  function escapeHtml(value) {
    return value.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");
  }
  function renderCrsTokens(tokens) {
    return tokens.map((token) => {
      const escaped = escapeHtml(token.value);
      const cls = CLASS_BY_KIND[token.kind];
      return cls ? `<span class="${cls}">${escaped}</span>` : escaped;
    }).join("");
  }
  function highlightCrsToHtml(source, lineNumbers = false) {
    const lines = tokenizeCrs(source.replace(/\r\n/g, `
`));
    if (!lineNumbers) {
      return lines.map(renderCrsTokens).join(`
`);
    }
    return lines.map((tokens, index) => {
      return `<span class="crs-line"><span class="crs-line-number">${index + 1}</span><span class="crs-line-source">${renderCrsTokens(tokens) || " "}</span></span>`;
    }).join(`
`);
  }

  // src/browser.ts
  var DEFAULT_CRS_SELECTOR = [
    "code.language-flour",
    "code.language-crustini",
    "pre.crs-code > code"
  ].join(", ");
  function shouldUseLineNumbers(options) {
    return options?.lineNumbers ?? true;
  }
  function highlightCrsElement(element, options = {}) {
    const source = element.getAttribute("data-crustini-source") ?? element.textContent ?? "";
    element.setAttribute("data-crustini-source", source);
    element.innerHTML = highlightCrsToHtml(source, shouldUseLineNumbers(options));
    element.classList.add(options.highlightedClass ?? "crs-highlighted");
    if (element.parentElement?.tagName.toLowerCase() === "pre") {
      element.parentElement.classList.add("crs-code");
    }
    return element;
  }
  function highlightAllCrs(options = {}) {
    const selector = options.selector ?? DEFAULT_CRS_SELECTOR;
    const elements = Array.from(document.querySelectorAll(selector));
    for (const element of elements) {
      highlightCrsElement(element, options);
    }
    return elements;
  }
  var CrustiniHighlight = {
    tokenizeLine: tokenizeCrsLine,
    highlight(source, lineNumbers = true) {
      return highlightCrsToHtml(source, lineNumbers);
    },
    highlightToHtml(source, options = {}) {
      return highlightCrsToHtml(source, shouldUseLineNumbers(options));
    },
    highlightElement: highlightCrsElement,
    highlightAll: highlightAllCrs
  };
  function installCrsBrowserHighlighter(global = globalThis) {
    Object.assign(global, { CrustiniHighlight });
    return CrustiniHighlight;
  }

  // src/browser-global.ts
  installCrsBrowserHighlighter();
})();
