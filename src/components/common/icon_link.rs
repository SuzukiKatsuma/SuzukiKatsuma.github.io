use yew::prelude::*;

pub fn icon_link(link_title: String, href: String, icon: Html) -> Html {
    html! {
      <a
        class="icon-link"
        target="_blank noopener noreferrer"
        aria-label={link_title.clone()}
        title={link_title.clone()}
        href={href.clone()}
      >
        {icon}
      </a>
    }
}
