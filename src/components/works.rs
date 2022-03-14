use yew::prelude::*;

#[function_component(Works)]
pub fn works() -> Html {
  html! {
    <div class="card works">
      <h1>{"Works"}</h1>

      <h2>
        {"二進指数え法道場"}
        <a target="__blank" aria-label="GitHub" title="GitHub" href="https://github.com/SuzukiKatsuma/finger-binary-dojo">
          <i class="fa-brands fa-github"></i>
        </a>
        <a target="__blank" aria-label="二進指数え法道場" title="二進指数え法道場" href="https://suzukikatsuma.github.io/finger-binary-dojo/">
          <i class="fa-solid fa-globe"></i>
        </a>
      </h2>
      <hr />
      <p>{"— 二進指数え法を練習するための WEB アプリ —"}</p>
      <figure>
        <img src="img/finger-binary-dojo.png" alt="二進指数え法道場" />
      </figure>
      <div class="core-technologies">
        <h3>{"Core Technologies: "}</h3>
        <img src="https://img.shields.io/badge/Svelte-fff.svg?logo=svelte&style=for-the-badge" alt="Svelte" />
      </div>

      <h2>
        {"Badge Generator"}
        <a target="__blank" aria-label="GitHub" title="GitHub" href="https://github.com/SuzukiKatsuma/badge-generator">
          <i class="fa-brands fa-github"></i>
        </a>
        <a target="__blank" aria-label="Badge Generator" title="Badge Generator" href="https://suzukikatsuma.github.io/badge-generator/">
          <i class="fa-solid fa-globe"></i>
        </a>
      </h2>
      <hr />
      <p>{"— shields.io のバッジを簡単に作るための WEB アプリ —"}</p>
      <figure>
        <img src="img/badge-generator.png" alt="Badge Generator" />
      </figure>
      <div class="core-technologies">
        <h3>{"Core Technologies: "}</h3>
        <img src="https://img.shields.io/badge/React-20232a.svg?logo=react&style=for-the-badge" alt="React"/>
      </div>
    </div>
  }
}