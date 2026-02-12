use std::fmt::Debug;
use yew::prelude::*;

use components::*;
pub mod components;

#[derive(Debug)]
enum Msg {
  Forward,
  Backward,
  NoChange,
}

#[derive(Default, Debug)]
struct App {
  value: u8,
}

const NUM_CARDS: usize = 3;
const MAX_VALUE: u8 = (NUM_CARDS as u8 - 1) * 20;

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self { value: 0 }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::Forward => {
        if self.value < MAX_VALUE {
          self.value += 1;
        }
        true
      }
      Msg::Backward => {
        if 0 < self.value {
          self.value -= 1;
        }
        true
      }
      Msg::NoChange => false,
    }
  }

    fn view(&self, ctx: &Context<Self>) -> Html {
      let link = ctx.link();

      let on_wheel = link.callback(|e: WheelEvent| {
        let delta_y = e.delta_y() as f32;
        if delta_y > 0.0 {
          Msg::Forward
        } else if delta_y < 0.0 {
          Msg::Backward
        } else {
          Msg::NoChange
        }
      });

      let f_value: f32 = self.value as f32;
      let percentage: f32 = ((f_value / MAX_VALUE as f32) * 100.0).round();

      let mut opacity: [f32; NUM_CARDS] = [0.; NUM_CARDS];
      let mut i = 0;
      while i < NUM_CARDS {
        opacity[i] = 1. - 0.01 * (f_value - (i as f32 * 20.)) * (f_value - (i as f32 * 20.));
        i += 1;
      }

      html! {
        <>
          <header>
            if self.value == 0 {<p>{"Wheel your mouse!"}</p>}
            <p class="status">
              {percentage}{"%"}
            </p>
          </header>

          <main>
            <div class="handling-area" onwheel={on_wheel} />
            { profile::profile(opacity[0]) }
            { works::works(opacity[1]) }
            { dead_end::dead_end(opacity[2]) }
          </main>

          <footer>
            <p class="right">
              <a
                class="icon-link"
                target="_blank noopener noreferrer"
                aria-label="GitHub"
                href="https://github.com/SuzukiKatsuma"
              >
                { ">> GitHub" }
              </a>
            </p>
          </footer>
        </>
      }
  }
}

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::Renderer::<App>::new().render();
}
