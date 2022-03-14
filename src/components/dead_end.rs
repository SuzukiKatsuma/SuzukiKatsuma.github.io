use yew::prelude::*;

#[function_component(DeadEnd)]
pub fn dead_end() -> Html {
  html! {
    <div class="dead-end">
      <h1>{"この先、行き止まり。"}</h1>
      <p>{"DEAD END"} </p>
    </div>
  }
}
