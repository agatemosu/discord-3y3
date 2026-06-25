pub fn is_bright(color: u32) -> bool {
	let r = (color >> 16) & 0xff;
	let g = (color >> 8) & 0xff;
	let b = color & 0xff;

	let brightness = (299 * r + 587 * g + 114 * b) / 1000;

	brightness > 128
}

pub fn to_css_hex(color: u32) -> String {
	format!("#{:06x}", color)
}

pub fn encode(primary: u32, accent: u32) -> String {
	let primary_hex = to_css_hex(primary);
	let accent_hex = to_css_hex(accent);

	let message = format!("[{},{}]", primary_hex, accent_hex);
	let encoded: String = message
		.chars()
		.map(|c| char::from_u32(c as u32 + 0xE0000).unwrap())
		.collect();

	format!(" {}", encoded)
}
