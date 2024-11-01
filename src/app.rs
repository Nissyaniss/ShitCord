#![allow(non_snake_case)]

use std::{rc::Rc, sync::Mutex};

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

#[derive(Serialize, Deserialize)]
struct TotpArgs {
	code: String,
	ticket: String,
}

pub fn App() -> Element {
	let mut email = use_signal(String::new);
	let mut password = use_signal(String::new);
	let mut code = use_signal(String::new);
	let mut ticket = use_signal(String::new);
	let mut token = use_signal(String::new);

	let login = move |_: FormEvent| async move {
		if email.read().is_empty() || password.read().is_empty() {
			return;
		}

		let test_args = serde_wasm_bindgen::to_value(&LoginArgs {
			login: email.read().to_string(),
			password: password.read().to_string(),
		})
		.unwrap();
		*ticket.write() = invoke_args("login", test_args).await.as_string().unwrap();
	};

	let totp = move |_: FormEvent| async move {
		if code.read().is_empty() {
			return;
		}

		let test_args = serde_wasm_bindgen::to_value(&TotpArgs {
			code: code.read().to_string(),
			ticket: ticket.read().to_string(),
		})
		.unwrap();
		*token.write() = invoke_args("totp", test_args).await.as_string().unwrap();
	};

	rsx! {
		link { rel: "stylesheet", href: "styles.css" }
		main {
			class: "container",
			if ticket.read().is_empty() {
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

					},
					button { r#type: "submit", "Login" },
				}
			},
			if !ticket.read().is_empty() {
				form {
					class: "row",
					onsubmit: totp,
					input {
						id: "login-input",
						placeholder: "Enter a code...",
						value: "{code}",
						oninput: move |event| code.set(event.value())
					},
					button { r#type: "submit", "Confirmer" },
				}
			}
		}
	}
}
