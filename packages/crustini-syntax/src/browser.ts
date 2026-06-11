import { highlightCrsToHtml } from "./html";
import { tokenizeCrsLine } from "./crs-tokenizer";

export type CrsHighlightOptions = {
  selector?: string;
  lineNumbers?: boolean;
  highlightedClass?: string;
};

export const DEFAULT_CRS_SELECTOR = [
  "code.language-flour",
  "code.language-crustini",
  "pre.crs-code > code",
].join(", ");

function shouldUseLineNumbers(options?: CrsHighlightOptions): boolean {
  return options?.lineNumbers ?? true;
}

export function highlightCrsElement(element: Element, options: CrsHighlightOptions = {}): Element {
  const source = element.getAttribute("data-crustini-source") ?? element.textContent ?? "";
  element.setAttribute("data-crustini-source", source);
  element.innerHTML = highlightCrsToHtml(source, shouldUseLineNumbers(options));
  element.classList.add(options.highlightedClass ?? "crs-highlighted");

  if (element.parentElement?.tagName.toLowerCase() === "pre") {
    element.parentElement.classList.add("crs-code");
  }

  return element;
}

export function highlightAllCrs(options: CrsHighlightOptions = {}): Element[] {
  const selector = options.selector ?? DEFAULT_CRS_SELECTOR;
  const elements = Array.from(document.querySelectorAll(selector));

  for (const element of elements) {
    highlightCrsElement(element, options);
  }

  return elements;
}

export const CrustiniHighlight = {
  tokenizeLine: tokenizeCrsLine,
  highlight(source: string, lineNumbers = true): string {
    return highlightCrsToHtml(source, lineNumbers);
  },
  highlightToHtml(source: string, options: CrsHighlightOptions = {}): string {
    return highlightCrsToHtml(source, shouldUseLineNumbers(options));
  },
  highlightElement: highlightCrsElement,
  highlightAll: highlightAllCrs,
};

export type CrustiniHighlightGlobal = typeof CrustiniHighlight;

export function installCrsBrowserHighlighter(global: typeof globalThis = globalThis): CrustiniHighlightGlobal {
  Object.assign(global, { CrustiniHighlight });
  return CrustiniHighlight;
}
