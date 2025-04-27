#![allow(non_snake_case)]

mod comment;
mod stories;

use dioxus::{document::Stylesheet, prelude::*};
use stories::Stories;

use crate::StoryData;
use comment::Comments;

#[derive(Debug, Clone)]
pub enum CommentState {
    Unset,
    Loading,
    Loaded(Box<StoryData>),
}

pub fn App() -> Element {
    use_context_provider(|| Signal::new(CommentState::Unset));
    rsx! {
      Stylesheet { href: asset!("assets/tailwind.css") }
      main { class: "flex w-full h-full shadow-lg rounded-3xl",
        section { class: "flex flex-col w-4/12 h-full pt-3 overflow-y-scroll bg-gray-50",
          Stories {}
        }
        section { class: "flex flex-col w-8/12 px-4 bg-white rounded-r-3xl", Comments {} }
      }
    }
}
