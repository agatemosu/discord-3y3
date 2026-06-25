use web_sys::{HtmlInputElement, js_sys::RegExp, wasm_bindgen::JsCast};
use yew::{platform::spawn_local, prelude::*};

use crate::util;

#[derive(Properties, PartialEq)]
struct ColorPickerProps {
	label: String,
	color: u32,
	onchange: Callback<u32>,
}

#[component]
fn ColorPicker(
	ColorPickerProps {
		label,
		color,
		onchange,
	}: &ColorPickerProps,
) -> Html {
	let hex_color = util::to_css_hex(*color);

	let handle_change = {
		let onchange = onchange.clone();
		move |e: InputEvent| {
			let target = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();

			let value = target.value().replace("#", "");
			let value = u32::from_str_radix(&value, 16).unwrap();

			onchange.emit(value);
		}
	};

	let handle_text_change = {
		let onchange = onchange.clone();
		move |e: InputEvent| {
			let target = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();

			let value = target.value().replace("#", "");
			if !RegExp::new("^[0-9A-Fa-f]{6}$", "").test(&value) {
				return;
			}

			let value = u32::from_str_radix(&value, 16).unwrap();
			onchange.emit(value);
		}
	};

	html! {
		<div class="color-input-group">
			<span>{label}</span>
			<label
				class="color-input-label"
				style={format!("--hex-color: {}", &hex_color)}
			>
				<input
					type="color"
					value={hex_color.clone()}
					class="color-input"
					oninput={handle_change}
				/>
			</label>
			<input
				type="text"
				value={hex_color.clone()}
				maxlength={7}
				class="color-text-input"
				oninput={handle_text_change}
			/>
		</div>
	}
}

#[component]
pub fn App() -> Html {
	let primary_color = use_state(|| 0xd2e5db as u32);
	let accent_color = use_state(|| 0xd2e5db as u32);

	let encoded_output = util::encode(*primary_color, *accent_color);

	let handle_copy = move |_| {
		let encoded_output = encoded_output.clone();
		spawn_local(async move {
			let window = web_sys::window().unwrap();
			window
				.navigator()
				.clipboard()
				.write_text(&encoded_output)
				.await
				.unwrap();
			window
				.alert_with_message("Copied 3y3 string to clipboard!")
				.unwrap();
		});
	};

	let style = format!(
		"--primary-color: {}; --accent-color: {}; --mix-color: {}; --text-color: {}",
		util::to_css_hex(*primary_color),
		util::to_css_hex(*accent_color),
		if util::is_bright(*primary_color) {
			"white"
		} else {
			"black"
		},
		if util::is_bright(*primary_color) {
			"#303030"
		} else {
			"#efefef"
		},
	);

	html! {
		<main class="container">
			<h1 class="heading">{"Fake Profile Themes Encoder"}</h1>

			<div class="color-pickers">
			<ColorPicker
					label="Primary Color"
					color={*primary_color}
					onchange={
						let primary_color = primary_color.clone();
						move |color| primary_color.set(color)
					}
				/>
				<ColorPicker
					label="Accent Color"
					color={*accent_color}
					onchange={
						let accent_color = accent_color.clone();
						move |color| accent_color.set(color)
					}
				/>
			</div>

			<button type="button" class="button" onclick={handle_copy}>
				{"Copy 3y3"}
			</button>

			<h2 class="heading heading-2">{"Live Preview"}</h2>
			<div
				class="profile-preview"
				{style}
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
						<p class="profile-username">{"Username"}</p>
						<p class="profile-bio">
							{"This is a sample bio. Your encoded 3y3 will be hidden here!"}
						</p>
					</div>
				</div>
			</div>
		</main>
	}
}
