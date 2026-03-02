use gloo_net::http::Request;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const API_URL: &str = "https://api.researchmap.jp/suzuki-katsuma/published_papers";
const MAX_PAPERS: usize = 5;

#[derive(Clone, Debug, PartialEq)]
struct LocalizedText {
    ja: Option<String>,
    en: Option<String>,
}

impl LocalizedText {
    fn display_text(&self) -> String {
        self.ja
            .clone()
            .or_else(|| self.en.clone())
            .unwrap_or_else(|| String::from("タイトル未設定"))
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Paper {
    paper_title: LocalizedText,
    authors: String,
    publication_name: String,
    publication_date: String,
    detail_url: String,
}

fn extract_text(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        }
        Value::Array(arr) => arr.iter().find_map(extract_text),
        Value::Object(map) => {
            for key in ["ja", "value"] {
                if let Some(text) = map.get(key).and_then(extract_text) {
                    return Some(text);
                }
            }
            map.values().find_map(extract_text)
        }
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

fn extract_field(item: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| item.get(key))
        .and_then(extract_text)
}

fn extract_localized_text(value: &Value) -> LocalizedText {
    let ja = value
        .get("ja")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from);
    let en = value
        .get("en")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from);

    if ja.is_none() && en.is_none() {
        if let Some(single) = extract_text(value) {
            LocalizedText {
                ja: Some(single),
                en: None,
            }
        } else {
            LocalizedText { ja: None, en: None }
        }
    } else {
        LocalizedText { ja, en }
    }
}

fn collect_texts(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::String(s) => {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                out.push(trimmed.to_string());
            }
        }
        Value::Array(arr) => {
            for v in arr {
                collect_texts(v, out);
            }
        }
        Value::Object(map) => {
            let mut matched = false;
            for key in ["ja", "value", "name"] {
                if let Some(v) = map.get(key) {
                    matched = true;
                    collect_texts(v, out);
                }
            }
            if !matched {
                for v in map.values() {
                    collect_texts(v, out);
                }
            }
        }
        Value::Number(n) => out.push(n.to_string()),
        _ => {}
    }
}

fn extract_authors(item: &Value) -> Option<String> {
    let mut authors = Vec::new();

    for key in [
        "authors",
        "author",
        "author_list",
        "author_names",
        "creator",
    ] {
        if let Some(value) = item.get(key) {
            collect_texts(value, &mut authors);
        }
    }

    let mut unique_authors = Vec::new();
    for author in authors {
        if !unique_authors.iter().any(|a| a == &author) {
            unique_authors.push(author);
        }
    }

    if unique_authors.is_empty() {
        None
    } else {
        Some(unique_authors.join(", "))
    }
}

fn normalize_researchmap_url(url: &str) -> String {
    url.replace("://api.researchmap.jp/", "://researchmap.jp/")
}

fn to_papers(payload: Value) -> Vec<Paper> {
    let items = payload
        .get("items")
        .and_then(Value::as_array)
        .cloned()
        .or_else(|| payload.as_array().cloned())
        .unwrap_or_default();

    let mut papers: Vec<Paper> = items
        .into_iter()
        .map(|item| {
            let paper_title = item
                .get("paper_title")
                .map(extract_localized_text)
                .or_else(|| item.get("title").map(extract_localized_text))
                .unwrap_or(LocalizedText { ja: None, en: None });
            let authors = extract_authors(&item).unwrap_or_else(|| String::from("著者情報なし"));
            let publication_name = extract_field(&item, &["publication_name", "journal"])
                .unwrap_or_else(|| String::from("掲載誌未設定"));
            let publication_date =
                extract_field(&item, &["publication_date", "published_year"]).unwrap_or_default();
            let detail_url = normalize_researchmap_url(
                item.get("@id")
                    .and_then(Value::as_str)
                    .expect("`@id` must exist by API contract"),
            );

            Paper {
                paper_title,
                authors,
                publication_name,
                publication_date,
                detail_url,
            }
        })
        .collect();

    papers.sort_by(|a, b| b.publication_date.cmp(&a.publication_date));
    papers.truncate(MAX_PAPERS);
    papers
}

#[derive(Properties, PartialEq)]
struct ResearchProps {
    opacity: f32,
}

#[function_component(ResearchCard)]
fn research_card(props: &ResearchProps) -> Html {
    let papers = use_state(|| Option::<Vec<Paper>>::None);
    let error_message = use_state(|| Option::<String>::None);

    {
        let papers = papers.clone();
        let error_message = error_message.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                match Request::get(API_URL).send().await {
                    Ok(response) => match response.json::<Value>().await {
                        Ok(payload) => papers.set(Some(to_papers(payload))),
                        Err(err) => error_message.set(Some(format!("JSON parse error: {err}"))),
                    },
                    Err(err) => error_message.set(Some(format!("Request error: {err}"))),
                }
            });

            || ()
        });
    }

    html! {
      <div class="card research">
      <h1>{"Research"}</h1>

      if let Some(message) = &*error_message {
        <p>{format!("研究業績の取得に失敗しました: {message}")}</p>
      } else if let Some(items) = &*papers {
        if items.is_empty() {
        <p>{"表示できる論文データがありませんでした。"}</p>
        } else {
        {
          items
          .iter()
          .enumerate()
          .map(|(_index, paper)| {
            html! {
            <div class="paper">
              <a href={paper.detail_url.clone()} target="_blank" rel="noopener noreferrer">
              <h2 class="title">{paper.paper_title.display_text()}</h2>
              </a>
              <p class="authors">{&paper.authors}</p>
              <p class="venue">
              {&paper.publication_name}
              {" "}
              if !paper.publication_date.is_empty() {
                {", "}
                {&paper.publication_date}
              }
              </p>
            </div>
            }
          })
          .collect::<Html>()
        }
        }
      } else {
        <p>{"研究業績を読み込み中..."}</p>
      }

      <hr />

      <p>
        {"More: "}
        <a
        class="icon-link"
        target="_blank noopener noreferrer"
        aria-label="Google Scholar"
        href="https://scholar.google.co.jp/citations?user=S5GxBFsAAAAJ"
        >{"Google Scholar"}</a>
        {", "}
        <a
        class="icon-link"
        target="_blank noopener noreferrer"
        aria-label="Researchmap"
        href="https://researchmap.jp/suzuki-katsuma"
        >{"Researchmap"}</a>
      </p>
      <style>
        {".research {"}
        {"opacity: "}{props.opacity}{";"}
        if 0.1 > props.opacity {{"pointer-events: none;"}}
        {"}"}
      </style>
      </div>
    }
}

pub fn research(opacity: f32) -> Html {
    html! { <ResearchCard opacity={opacity} /> }
}
