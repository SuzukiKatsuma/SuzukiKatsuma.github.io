use yew::prelude::*;

pub fn profile(opacity :f32) -> Html {
    html! {
      <>
        <div class="card profile">
          <p>{"Hello, I'm"}</p>
          <h1>{"Suzuki Katsuma"}</h1>

          <br/>

          <p class="right">
            {"Contact me: "}
            <a target="__blank" aria-label="Twitter" title="Twitter" href="https://twitter.com/_katsuma">
              <i class="fa-brands fa-x-twitter"></i>
            </a>
            <a target="__blank" aria-label="GitHub" title="GitHub" href="https://github.com/SuzukiKatsuma">
              <i class="fa-brands fa-github"></i>
            </a>
            <a target="__blank" aria-label="Zenn" title="Zenn" href="https://zenn.dev/suzuki_katsuma">
              <i class="fa-solid fa-paperclip"></i>
            </a>
          </p>

          <style>
            {".profile {"}
            {"opacity: "}{opacity}{";"}
            if 0.1 > opacity {{"pointer-events: none;"}}
            {"}"}
          </style>
        </div>
      </>
    }
}