use crate::components::common::icon_link;
use yew::prelude::*;

pub fn profile(opacity: f32) -> Html {
    html! {
      <div class="card profile">
        <div class="introduction">
          <figure class="icon">
            <img src="img/icon.png" alt="icon" />
          </figure>

          <p>{"Hello, I'm"}
          <h1>{"Suzuki Katsuma"}</h1>
          </p>
        </div>

        <p class="right">
          {"Contact me: "}
          {
            icon_link(
              String::from("X"),
              String::from("https://x.com/_katsuma"),
              html! { <i class="fa-brands fa-x-twitter"></i> }
            )
          }
          {
            icon_link(
              String::from("GitHub"),
              String::from("https://github.com/SuzukiKatsuma"),
              html! { <i class="fa-brands fa-github"></i> }
            )
          }
          {
            icon_link(
              String::from("Zenn"),
              String::from("https://zenn.dev/suzuki_katsuma"),
              html! { <i class="fa-solid fa-newspaper"></i> }
            )
          }
        </p>

        <style>
          {".profile {"}
          {"opacity: "}{opacity}{";"}
          if 0.1 > opacity {{"pointer-events: none;"}}
          {"}"}
        </style>
      </div>
    }
}
