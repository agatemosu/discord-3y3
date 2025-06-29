export function isBright(hexColor: number) {
	const r = (hexColor >> 16) & 0xff;
	const g = (hexColor >> 8) & 0xff;
	const b = hexColor & 0xff;
	const brightness = 0.299 * r + 0.587 * g + 0.114 * b;
	return brightness > 128;
}

export function toCssHex(color: number) {
	return `#${color.toString(16).padStart(6, "0")}`;
}

/**
 * Encodes primary and accent colors into a 3y3 string for Discord profile themes.
 * @param primary - The primary color as a number (e.g., 0xFFFFFF).
 * @param accent - The accent color as a number (e.g., 0xFFFFFF).
 * @returns The 3y3 encoded string.
 */
export function encode(primary: number, accent: number): string {
	const primaryHex = toCssHex(primary);
	const accentHex = toCssHex(accent);

	const message = `[${primaryHex},${accentHex}]`;
	const encoded = Array.from(message)
		.map((x) => x.codePointAt(0) ?? 0)
		.filter((x) => x >= 0x20 && x <= 0x7f)
		.map((x) => String.fromCodePoint(x + 0xe0000))
		.join("");

	return ` ${encoded}`;
}
