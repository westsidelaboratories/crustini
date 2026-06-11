import type PrismType from "prismjs";

export function registerCrsPrism(Prism: typeof PrismType) {
  const grammar = {
    frontmatter: /^(\+\+\+)$/m,
    comment: /\/\/.*/,
    string: {
      pattern: /"(?:\\.|[^"\\])*"/,
      greedy: true,
    },
    compiler: /\bapp!/,
    number: /\b\d[\d_]*(?:\.\d[\d_]*)?\b/,
    keyword: /\b(?:if|else|match|true|false|let|mut|return|break|continue|for|while|in)\b/,
    declaration: /\b(?:state|fn|struct|enum|setup|update|draw)\b/,
    type: /\b(?:number|text|bool|Vec2|Color|Button|Sprite)\b/,
    constant: /\b(?:Black|White|Gray|Green|Red|Blue|Yellow|Magenta|Cyan|A|B|Left|Right|Up|Down|Start|Select|Title|Playing|Dead)\b/,
    builtin: /\b(?:clear|line|rect|circle|text|sprite|vec2|pressed|down|released|axis_x|axis_y|stick|mouse_x|mouse_y|mouse_down|dt|min|max|clamp|abs|hit_rect)\b/,
    property: /\b[A-Za-z_][A-Za-z0-9_]*(?=\s*[:=])/,
    operator: /(?:==|!=|<=|>=|&&|\|\||\+=|-=|->|=>|[=+\-*/%<>!|&]+)/,
    punctuation: /[{}[\](),:.]/,
  };
  Prism.languages.flour = grammar;
  Prism.languages.crustini = grammar;
  Prism.languages.crs = grammar;
}
