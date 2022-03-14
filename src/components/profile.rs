use yew::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    html! {
      <>
        <div class="card profile">
          <p>{"Hello, I'm"}</p>
          <h1>{"Suzuki Katsuma"}</h1>

          <br/>

          <p class="right">
            {"Contact me: "}
            <a target="__blank" aria-label="GitHub" title="GitHub" href="https://github.com/SuzukiKatsuma">
              <i class="fa-brands fa-github"></i>
            </a>
            <a target="__blank" aria-label="Twitter" title="Twitter" href="https://twitter.com/SuzukiKatsuma">
              <i class="fa-brands fa-twitter"></i>
            </a>
          </p>
        </div>
      </>
    }
}