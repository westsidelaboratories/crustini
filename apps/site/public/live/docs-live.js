const editor = document.querySelector("#editor");
const preview = document.querySelector("#preview");
const copyHtml = document.querySelector("#copyHtml");

const starter = `# Crustini UI notes

Vocabulary:

- \`.flour\` is source code.
- \`bake\` is compile/build.
- \`bakery\` is the generated build workspace/cache.
- \`starter\` is a project template.

Use plain words for artifacts, diagnostics, logs, generated Rust, and runtime details.

We use macro-shaped source. \`app!\` is the root macro, and sections/calls can stay macro-flavored too.

The first UI layer is not LVGL. It is tiny immediate-mode drawing plus MiniUI helpers.

\`\`\`flour
app! {
  screen!(240, 135)
  fps!(30)

  state! {
    count: i32 = 0
  }

  update! {
    if a {
      count = count + 1
    }

    if b {
      count = count - 1
    }
  }

  draw! {
    clear!(0)
    rect!(8, 8, 224, 119, 24)
    line!(8, 34, 232, 34, 96)
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
    if (codeLang === "flour" || codeLang === "crs" || codeLang === "crustini") {
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
