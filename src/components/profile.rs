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
        </p>

        <p class="right">
          {"Articles: "}
          <a
            class="icon-link"
            target="_blank noopener noreferrer"
            aria-label="Zenn"
            href="https://zenn.dev/suzuki_katsuma"
          >
            { "Zenn" }
          </a>
          { ", " }
          <a
            class="icon-link"
            target="_blank noopener noreferrer"
            aria-label="Qiita"
            href="https://qiita.com/_katsuma"
          >
            { "Qiita" }
          </a>
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
