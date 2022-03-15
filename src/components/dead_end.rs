use yew::prelude::*;

pub fn dead_end(opacity :f32) -> Html {
  html! {
    <div class="dead-end">
      <h1>{"この先、行き止まり。"}</h1>
      <p>{"DEAD END"} </p>

      <style>
        {".dead-end {"}
        {"opacity:"}{opacity}{";"}
        if 0.1 > opacity {{"pointer-events: none;"}}
        {"}"}
      </style>
    </div>
  }
}
