import type { JSX } from "preact";
import type { Dispatch, StateUpdater } from "preact/hooks";
import { useEffect, useState } from "preact/hooks";
import { encode, isBright, toCssHex } from "./utils";

interface ColorPickerProps {
	label: string;
	color: number;
	onChange: Dispatch<StateUpdater<number>>;
}

function ColorPicker({ label, color, onChange }: ColorPickerProps) {
	const hexColor = toCssHex(color);

	const handleChange: JSX.InputEventHandler<HTMLInputElement> = (e) => {
		const value = e.currentTarget.value.replace("#", "");
		onChange(Number.parseInt(value, 16));
	};

	const handleTextChange: JSX.InputEventHandler<HTMLInputElement> = (e) => {
		let value = e.currentTarget.value.trim();

		// Allow user to omit "#" in text input
		if (!value.startsWith("#")) {
			value = `#${value}`;
		}

		// Check if it's 6-digit hex: #RRGGBB
		if (/^#[0-9A-Fa-f]{6}$/.test(value)) {
			onChange(Number.parseInt(value.replace("#", ""), 16));
		}
	};

	return (
		<div class="color-input-group">
			<span>{label}</span>
			<label class="color-input-label" style={{ backgroundColor: hexColor }}>
				<input
					type="color"
					value={hexColor}
					class="color-input"
					onInput={handleChange}
				/>
			</label>
			<input
				type="text"
				value={hexColor}
				maxLength={7}
				pattern="#[0-9A-Fa-f]{6}"
				class="color-text-input"
				onInput={handleTextChange}
			/>
		</div>
	);
}

export function App() {
	const [primaryColor, setPrimaryColor] = useState<number>(0xd2e5db);
	const [accentColor, setAccentColor] = useState<number>(0xd2e5db);
	const [encodedOutput, setEncodedOutput] = useState<string>("");

	useEffect(() => {
		setEncodedOutput(encode(primaryColor, accentColor));
	}, [primaryColor, accentColor]);

	const handleCopy = async () => {
		await navigator.clipboard.writeText(encodedOutput);
		alert("Copied 3y3 string to clipboard!");
	};

	return (
		<div class="container">
			<h1 class="heading">Fake Profile Themes Encoder</h1>

			<div class="color-pickers">
				<ColorPicker
					label="Primary Color"
					color={primaryColor}
					onChange={setPrimaryColor}
				/>
				<ColorPicker
					label="Accent Color"
					color={accentColor}
					onChange={setAccentColor}
				/>
			</div>

			<button type="button" class="button" onClick={handleCopy}>
				Copy 3y3
			</button>

			<h2 class="heading heading-2">Live Preview</h2>
			<div
				class="profile-preview"
				style={{
					"--primary-color": toCssHex(primaryColor),
					"--accent-color": toCssHex(accentColor),
					"--mix-color": isBright(primaryColor) ? "white" : "black",
					"--text-color": isBright(primaryColor) ? "#303030" : "#efefef",
				}}
			>
				<div class="profile-banner"></div>
				<div class="profile-avatar-wrapper">
					<a
						href="https://github.com/agatemosu"
						target="_blank"
						rel="noopener noreferrer"
						class="profile-avatar"
					>
						<img src="https://github.com/agatemosu.png" alt="GitHub" />
					</a>
				</div>
				<div class="profile-content">
					<div class="profile-info">
						<p class="profile-username">Username</p>
						<p class="profile-bio">
							This is a sample bio. Your encoded 3y3 will be hidden here!
						</p>
					</div>
				</div>
			</div>
		</div>
	);
}
