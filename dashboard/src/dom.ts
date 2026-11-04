// Element builders. Text always goes in as text nodes, never as HTML: run names,
// actor keys and violation messages come from the data.

type Attrs = Record<string, string | number | boolean | undefined>;
type Child = Node | string | number | null | undefined | false;

const SVG_NS = "http://www.w3.org/2000/svg";

function fill(element: Element, attrs: Attrs, children: Child[]): void {
	for (const [name, value] of Object.entries(attrs)) {
		if (value === undefined || value === false) continue;
		element.setAttribute(name, value === true ? "" : String(value));
	}
	for (const child of children) {
		if (child === null || child === undefined || child === false) continue;
		element.append(child instanceof Node ? child : document.createTextNode(String(child)));
	}
}

export function h<K extends keyof HTMLElementTagNameMap>(
	tag: K,
	attrs: Attrs = {},
	...children: Child[]
): HTMLElementTagNameMap[K] {
	const element = document.createElement(tag);
	fill(element, attrs, children);
	return element;
}

export function s<K extends keyof SVGElementTagNameMap>(
	tag: K,
	attrs: Attrs = {},
	...children: Child[]
): SVGElementTagNameMap[K] {
	const element = document.createElementNS(SVG_NS, tag);
	fill(element, attrs, children);
	return element;
}
