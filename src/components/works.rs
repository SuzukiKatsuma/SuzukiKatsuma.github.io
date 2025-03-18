use crate::components::common::icon_link;
use yew::prelude::*;

pub fn works(opacity: f32) -> Html {
    html! {
      <div class="card works">
        <h1>{"Works"}</h1>

        <h2>
          {"二進指数え法道場"}
          {
            icon_link(
              String::from("GitHub"),
              String::from("https://github.com/SuzukiKatsuma/finger-binary-dojo"),
              html! { <i class="fa-brands fa-github"></i> }
            )
          }
          {
            icon_link(
              String::from("二進指数え法道場"),
              String::from("https://suzukikatsuma.github.io/finger-binary-dojo/"),
              html! { <i class="fa-solid fa-globe"></i> }
            )
          }
        </h2>
        <hr />
        <p>{"— 二進指数え法を練習するための WEB アプリ —"}</p>
        <figure>
          <img src="img/finger-binary-dojo.png" alt="二進指数え法道場" />
        </figure>
        <div class="core-technology">
          <h3>{"Core Technologiy: "}</h3>
          <img src="https://img.shields.io/badge/Svelte-fff.svg?logo=svelte&style=for-the-badge" alt="Svelte" />
        </div>

        <h2>
          {"Badge Generator"}
          {
            icon_link(
              String::from("GitHub"),
              String::from("https://github.com/SuzukiKatsuma/badge-generator"),
              html! { <i class="fa-brands fa-github"></i> }
            )
          }
          {
            icon_link(
              String::from("Badge Generator"),
              String::from("https://suzukikatsuma.github.io/badge-generator/"),
              html! { <i class="fa-solid fa-globe"></i> }
            )
          }
        </h2>
        <hr />
        <p>{"— shields.io のバッジを簡単に作るための WEB アプリ —"}</p>
        <figure>
          <img src="img/badge-generator.png" alt="Badge Generator" />
        </figure>
        <div class="core-technology">
          <h3>{"Core Technologiy: "}</h3>
          <img src="https://img.shields.io/badge/React-20232a.svg?logo=react&style=for-the-badge" alt="React"/>
        </div>

        <style>
          {".works {"}
          {"opacity:"}{opacity}{";"}
          if 0.1 > opacity {{"pointer-events: none;"}}
          {"}"}
        </style>
      </div>
    }
}
