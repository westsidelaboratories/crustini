import { CrsToken, tokenizeCrs } from "./crs-tokenizer";

const CLASS_BY_KIND: Record<CrsToken["kind"], string> = {
  space: "",
  comment: "crs-comment",
  string: "crs-string",
  number: "crs-number",
  macro: "crs-macro",
  keyword: "crs-keyword",
  declaration: "crs-declaration",
  type: "crs-type",
  constant: "crs-constant",
  builtin: "crs-builtin",
  property: "crs-property",
  identifier: "crs-identifier",
  punctuation: "crs-punctuation",
  operator: "crs-operator",
};

export function escapeHtml(value: string): string {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;");
}

export function renderCrsTokens(tokens: CrsToken[]): string {
  return tokens.map((token) => {
    const escaped = escapeHtml(token.value);
    const cls = CLASS_BY_KIND[token.kind];
    return cls ? `<span class="${cls}">${escaped}</span>` : escaped;
  }).join("");
}

export function highlightCrsToHtml(source: string, lineNumbers = false): string {
  const lines = tokenizeCrs(source);

  if (!lineNumbers) {
    return lines.map(renderCrsTokens).join("\n");
  }

  return lines.map((tokens, index) => {
    return `<span class="crs-line"><span class="crs-line-number">${index + 1}</span><span class="crs-line-source">${renderCrsTokens(tokens) || " "}</span></span>`;
  }).join("\n");
}

export function renderCrsBlock(source: string): string {
  return `<pre class="crs-code"><code>${highlightCrsToHtml(source, true)}</code></pre>`;
}
