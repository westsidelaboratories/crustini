const editor = document.querySelector("#editor");
const preview = document.querySelector("#preview");
const copyHtml = document.querySelector("#copyHtml");

const starter = `# Crustini UI notes

Vocabulary:

- \`.flour\` is source code.
- \`bake\` is compile/build.
- \`oven\` is the generated build workspace and runtime target directory.
- \`loaf\` is the final binary, firmware, wasm bundle, or artifact.
- \`crumbs\` are diagnostics, logs, warnings, errors, traces, and build metadata.
- \`starter\` is a project template.

We use \`app!\` as the root macro.

The first UI layer is not LVGL. It is tiny immediate-mode drawing plus MiniUI helpers.

\`\`\`crs
app! {
  name: "TinyCounter"

  target: esp32_s3

  display: st7789 {
    width: 240
    height: 135
    color: rgb565
    rotation: landscape
  }

  input {
    button a
    button b
  }

  state {
    count: i32 = 0
  }

  screen Main {
    draw {
      clear(BLACK)

      text(8, 8, "COUNT", color: WHITE, font: bold_12)
      line(8, 24, 232, 24, color: GRAY)

      text(8, 52, count, color: GREEN, font: bold_16)

      text(8, 116, "A:+  B:-", color: GRAY)
    }

    on button.a {
      count = count + 1
      redraw()
    }

    on button.b {
      count = count - 1
      redraw()
    }
  }
}
\`\`\`

## What this compiles into

\`\`\`crs
CLEAR BLACK
TEXT 8 8 "COUNT" WHITE BOLD_12
LINE 8 24 232 24 GRAY
TEXT_STATE_I32 8 52 count GREEN BOLD_16
TEXT 8 116 "A:+  B:-" GRAY
FLUSH
\`\`\`
`;

function esc(s) {
  return s
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;");
}

// Extremely tiny Markdown-ish renderer.
// Good enough for language design docs.
// Later replace with micromark/marked if you want full Markdown.
function renderDocs(md) {
  const lines = md.replace(/\r\n/g, "\n").split("\n");
  let html = "";
  let para = [];
  let inCode = false;
  let codeLang = "";
  let code = [];

  function flushPara() {
    if (!para.length) return;
    const text = para.join(" ");
    html += `<p>${inline(text)}</p>\n`;
    para = [];
  }

  function inline(text) {
    return esc(text).replace(/`([^`]+)`/g, "<code>$1</code>");
  }

  function flushCode() {
    const raw = code.join("\n");
    if (codeLang === "crs" || codeLang === "crustini") {
      html += `<pre class="crs-code"><code class="language-crs">${CRS.highlight(raw, true)}</code></pre>\n`;
    } else {
      html += `<pre><code>${esc(raw)}</code></pre>\n`;
    }
    code = [];
  }

  for (const line of lines) {
    const fence = line.match(/^```([A-Za-z0-9_-]*)\s*$/);

    if (fence && !inCode) {
      flushPara();
      inCode = true;
      codeLang = fence[1] || "";
      code = [];
      continue;
    }

    if (fence && inCode) {
      inCode = false;
      flushCode();
      continue;
    }

    if (inCode) {
      code.push(line);
      continue;
    }

    if (line.startsWith("# ")) {
      flushPara();
      html += `<h1>${inline(line.slice(2))}</h1>\n`;
    } else if (line.startsWith("## ")) {
      flushPara();
      html += `<h2>${inline(line.slice(3))}</h2>\n`;
    } else if (line.startsWith("### ")) {
      flushPara();
      html += `<h3>${inline(line.slice(4))}</h3>\n`;
    } else if (line.trim() === "") {
      flushPara();
    } else {
      para.push(line.trim());
    }
  }

  flushPara();

  if (inCode) {
    flushCode();
  }

  return html;
}

function render() {
  preview.innerHTML = renderDocs(editor.value);
  localStorage.setItem("crustini-docs-live", editor.value);
}

editor.value = localStorage.getItem("crustini-docs-live") || starter;
editor.addEventListener("input", render);

copyHtml.addEventListener("click", async () => {
  await navigator.clipboard.writeText(preview.innerHTML);
});

render();
