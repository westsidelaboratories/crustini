const editor = document.querySelector("#editor");
const preview = document.querySelector("#preview");
const copyHtml = document.querySelector("#copyHtml");

const starter = `# Crustini sketch notes

Vocabulary:

- \`.flour\` is source code.
- \`bake\` is compile/build.
- \`bakery\` is the generated build workspace/cache.
- \`starter\` is a project template.

Use plain words for artifacts, diagnostics, logs, generated Rust, and runtime details.

Crustini starts with Processing-style sketches and grows into structured update/draw apps.

\`\`\`flour
+++
crustini = "0.1"
name = "Counter"
fps = 30
window = [640, 360]
+++

app! Main {
  state {
    count: number = 0;
  }

  fn update() {
    if pressed(Button::A) {
      count += 1;
    }

    if pressed(Button::B) {
      count -= 1;
    }
  }

  fn draw() {
    clear(Color::Black);
    text(24, 24, "COUNT", Color::White);
  }
}
\`\`\`

## What this compiles into

\`\`\`txt
.flour source
  -> Crustini compiler
  -> generated Rust
  -> native preview or artifact
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
    if (codeLang === "flour" || codeLang === "crustini") {
      html += `<pre class="crs-code"><code class="language-flour">${CrustiniHighlight.highlight(raw, true)}</code></pre>\n`;
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
