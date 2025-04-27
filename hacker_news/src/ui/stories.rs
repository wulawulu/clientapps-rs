#![allow(non_snake_case)]

use crate::{
    api::{get_story_comments, get_top_stories},
    StoryData, StoryItem as StoryItemType,
};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

use super::CommentState;

#[component]
pub fn Stories() -> Element {
    let stories = use_resource(|| get_top_stories(20));
    match &*stories.read_unchecked() {
        Some(Ok(stories)) => rsx! {
            ul {
                for story in stories{
                    StoryItem{story:story.clone()}
                }
              }
        },
        Some(Err(err)) => rsx! {
          div { class: "mt-6 text-red-500",
            p { "Failed to fetch stories" }
            p { "{err}" }
          }
        },
        None => rsx! {
          div { class: "mt-6",
            p { "Loading stories..." }
          }
        },
    }
}

#[component]
pub fn StoryItem(story: StoryItemType) -> Element {
    let comments_state = use_context::<Signal<CommentState>>();
    let full_story = use_signal(|| None);
    rsx! {
    li { class: "px-3 py-5 transition border-b hover:bg-indigo-100",
      a { href: "#", class: "flex items-center justify-between",
        h3 { class: "text-lg font-semibold", "{story.title}" }
        p { class: "text-gray-400 text-md" }
      }
      div { class: "italic text-gray-400 text-md",
        span { "{story.score} points by {story.by} {story.time} | " }
        a { href: "#",
            onclick: move |event|{
              let story = story.clone();
              async move{
              info!("Clicked on story: {} with event: {:#?}", story.title, event);
              load_comments(comments_state,full_story,story).await;
            }},
          "{story.kids.len()} comments"}
        }
      }
    }
}

async fn load_comments(
    mut comments_state: Signal<CommentState>,
    mut full_story: Signal<Option<StoryData>>,
    story_item: StoryItemType,
) {
    info!("Loading comments for story: {}", story_item.title);
    if let Some(story_data) = full_story.as_ref() {
        *comments_state.write() = CommentState::Loaded(Box::new(story_data.clone()));
        return;
    }
    *comments_state.write() = CommentState::Loading;

    if let Ok(story_data) = get_story_comments(story_item).await {
        *comments_state.write() = CommentState::Loaded(Box::new(story_data.clone()));
        *full_story.write() = Some(story_data);
    }
}
