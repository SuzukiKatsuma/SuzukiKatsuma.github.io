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

const NUM_CARDS: usize = 3;
const MAX_VALUE: u8 = (NUM_CARDS as u8 - 1) * 20;

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

		let mut opacity: [f32; NUM_CARDS] = [0.; NUM_CARDS];
		let mut i = 0;
		while i < NUM_CARDS {
			opacity[i] = 1. - 0.01 * (f_value - (i as f32 * 20.)) * (f_value - (i as f32 * 20.));
			i += 1;
		}

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
					{ profile::profile(opacity[0]) }
					{ works::works(opacity[1]) }
					{ dead_end::dead_end(opacity[2]) }
				</main>

				<footer>
				</footer>
			</>
		}
	}
}

fn main() {
	wasm_logger::init(wasm_logger::Config::default());
	yew::start_app::<App>();
}
