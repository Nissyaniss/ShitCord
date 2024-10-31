#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
	async fn invoke_args(cmd: &str, args: JsValue) -> JsValue;

	#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
	async fn invoke(cmd: &str) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
	name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct LoginArgs {
	login: String,
	password: String,
}

pub fn App() -> Element {
	let mut email = use_signal(String::new);
	let mut password = use_signal(String::new);

	let login = move |_: FormEvent| async move {
		if email.read().is_empty() || password.read().is_empty() {
			return;
		}

		let login_args = serde_wasm_bindgen::to_value(&LoginArgs {
			login: email.read().to_string(),
			password: password.read().to_string(),
		})
		.unwrap();
		// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
		invoke_args("login", login_args).await.as_string().unwrap();
	};

	rsx! {
		link { rel: "stylesheet", href: "styles.css" }
		main {
			class: "container",
			form {
				class: "row",
				onsubmit: login,
				input {
					id: "login-input",
					placeholder: "Enter a email...",
					value: "{email}",
					r#type: "email",
					oninput: move |event| email.set(event.value())
				},
				input {
					id: "login-input",
					placeholder: "Enter a password...",
					value: "{password}",
					r#type: "password",
					oninput: move |event| password.set(event.value())

				}
				button { r#type: "submit", "Greet" }
			}
		}
	}
}
