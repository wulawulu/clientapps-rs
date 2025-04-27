#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::Comment;

use super::CommentState;

#[component]
pub fn Comments() -> Element {
    let comment_state = use_context::<Signal<CommentState>>();
    match comment_state() {
        CommentState::Unset => rsx! {
          div {}
        },
        CommentState::Loading => rsx! {
          div { class: "mt-6",
            p { "Loading comments..." }
          }
        },
        CommentState::Loaded(data) => rsx! {
          ul {
            for comment in data.comments {
              StoryComment{comment:comment.clone()}
            }
          }
        },
    }
}

#[component]
pub fn StoryComment(comment: Comment) -> Element {
    rsx! {
      li {
        article { class: "p-4 leading-7 tracking-wider text-gray-500 border-b border-gray-200",
          span { "{comment.by} {comment.time} | next [-]" }
          div { dangerous_inner_html: comment.text }
        }
      }
    }
}
