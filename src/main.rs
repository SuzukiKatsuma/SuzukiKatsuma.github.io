use std::fmt::{Debug};
use yew::prelude::*;

use components::*;
pub mod components;

#[derive(Debug)]
enum Msg {
	ToggleDirection,
	Forward,
	Backward,
}

#[derive(Default, Debug)]
struct App {
	is_forward: bool,
	value: u8,
}

const MAX_VALUE: u8 = 40;

impl Component for App {
	type Message = Msg;
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self {
			is_forward: true,
			value: 0,
		}
	}

	fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::ToggleDirection => {
				self.is_forward = !self.is_forward;
			 	true
			},
			Msg::Forward => {
				if self.value < MAX_VALUE {
					self.value += 1;
				}
				true
			},
			Msg::Backward => {
				if 0 < self.value {
					self.value -= 1;
				}
				true
			}
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		let link = ctx.link();

		let movement = {
			if self.is_forward {
				link.callback(|_| Msg::Forward)
			} else {
				link.callback(|_| Msg::Backward)
			}
		};

		let arrow: char = if self.is_forward {'↑'} else {'↓'};

		let f_value: f32 = self.value as f32; 
		let profile_opacity: f32 = 1. - 0.01 * f_value * f_value;
		let works_opacity: f32 = 1. - 0.01 * (f_value - 20.) * (f_value - 20.);
		let dead_end_opacity: f32 = 1. - 0.01 * (f_value - 40.) * (f_value - 40.);

		html! {
			<>
				<header>
					if (self.value == 0) && (self.is_forward) {<p>{"Wheel your mouse!"}</p>}
					if ((self.value == 0) && (!self.is_forward)) || ((self.value == MAX_VALUE) && (self.is_forward)) {<p>{"Click to change Direction"}</p>}
					<p class="status">
						{"Direction: "}{arrow}
					</p>
				</header>

				<main onclick={link.callback(|_| Msg::ToggleDirection)} onwheel={movement}>
					<profile::Profile />
					<works::Works />
					<dead_end::DeadEnd />
				</main>

				<footer>
				</footer>

				<style>
					{".profile { opacity:"}
					{profile_opacity}{";"}
					if 0.1 > profile_opacity {{"pointer-events: none;"}}
					{"}"}
					{".works { opacity:"}
					{works_opacity}{";"}
					if 0.1 > works_opacity {{"pointer-events: none;"}}
					{"}"}
					{".dead-end { opacity:"}
					{dead_end_opacity}{";"}
					if 0.1 > dead_end_opacity {{"pointer-events: none;"}}
					{"}"}
				</style>
			</>
		}
	}
}

fn main() {
	wasm_logger::init(wasm_logger::Config::default());
	yew::start_app::<App>();
}
