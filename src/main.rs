use chrono::{TimeZone, Utc};
use clap::Parser;
use ego_tree::NodeRef;
use reqwest;
use scraper::node::Node;
use scraper::{ElementRef, Html};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Maximum number of stories to display (default: 5, max: 25)
    #[clap(short = 'm', long = "max-stories", default_value = "5", value_parser = clap::value_parser!(u8).range(1..=25))]
    max_stories: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)] // Added Clone
struct Story {
    id: u32,
    title: Option<String>,
    text: Option<String>,
    time: Option<i64>,
    url: Option<String>,
    by: Option<String>,
}

async fn fetch_story_details(story_id: u32) -> Result<Story, reqwest::Error> {
    let url = format!(
        "https://hacker-news.firebaseio.com/v0/item/{}.json?print=pretty",
        story_id
    );
    let story = reqwest::get(&url).await?.json::<Story>().await?;
    Ok(story)
}

async fn fetch_top_stories_ids() -> Result<Vec<u32>, reqwest::Error> {
    let url = "https://hacker-news.firebaseio.com/v0/topstories.json?print=pretty";
    let story_ids = reqwest::get(url).await?.json::<Vec<u32>>().await?;
    Ok(story_ids)
}

fn format_timestamp(timestamp: i64) -> String {
    let datetime = Utc.timestamp_opt(timestamp, 0).single();
    match datetime {
        Some(dt) => dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        None => "Invalid timestamp".to_string(),
    }
}

// Function to recursively process HTML nodes and build text representation
fn process_html_node(node_ref: NodeRef<'_, Node>, processed_text: &mut String) {
    match node_ref.value() {
        Node::Text(text_node) => {
            // Replace actual non-breaking space characters with regular spaces
            let cleaned_text = text_node.text.replace('\u{a0}', " ");
            processed_text.push_str(&cleaned_text);
        }
        Node::Element(_element_data) => {
            if let Some(element) = ElementRef::wrap(node_ref) {
                match element.value().name() {
                    "a" => {
                        let href_opt = element.value().attr("href");
                        let mut inner_text_builder = String::new();
                        for child in node_ref.children() {
                            process_html_node(child, &mut inner_text_builder);
                        }
                        let inner_text = inner_text_builder.trim();

                        match (href_opt, !inner_text.is_empty()) {
                            (Some(href_attr_val), true) => {
                                let href = href_attr_val.trim();
                                if href.is_empty() {
                                    processed_text.push_str(inner_text);
                                } else if href == inner_text {
                                    processed_text.push_str(href);
                                } else {
                                    processed_text.push_str(&format!("{} ({})", inner_text, href));
                                }
                            }
                            (Some(href_attr_val), false) => {
                                let href = href_attr_val.trim();
                                if !href.is_empty() {
                                    processed_text.push_str(href);
                                }
                            }
                            (None, true) => {
                                processed_text.push_str(inner_text);
                            }
                            (None, false) => {
                                // Do nothing
                            }
                        }
                    }
                    "p" => {
                        for child in node_ref.children() {
                            process_html_node(child, processed_text);
                        }
                        // Add a newline after paragraph content if needed
                        if !processed_text.is_empty() && !processed_text.ends_with('\n') {
                            processed_text.push('\n');
                        }
                    }
                    "br" => {
                        processed_text.push('\n');
                    }
                    _ => {
                        // Process children of other elements
                        for child in node_ref.children() {
                            process_html_node(child, processed_text);
                        }
                    }
                }
            }
        }
        _ => { /* Ignore comments, doctypes, etc. */ }
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();

    println!("Top {} Hacker News Stories:", args.max_stories);

    let top_story_ids = fetch_top_stories_ids().await?;

    let mut stories = Vec::new();
    let mut fetch_count = 0;
    for id in top_story_ids {
        if fetch_count >= args.max_stories as usize {
            // Limit to max stories specified by user
            break;
        }
        match fetch_story_details(id).await {
            Ok(story) => stories.push(story),
            Err(e) => eprintln!("Error fetching story {}: {}", id, e),
        }
        fetch_count += 1;
    }

    for story in stories {
        println!("--------------------------------------------------");
        println!("");

        let mut info_line = String::new();
        if let Some(time) = story.time {
            info_line.push_str(&format!("Timestamp: {} | ", format_timestamp(time)));
        } else {
            info_line.push_str("Timestamp: N/A | ");
        }
        if let Some(by) = &story.by {
            info_line.push_str(&format!("By: {} | ", by));
        } else {
            info_line.push_str("By: N/A | ");
        }
        info_line.push_str(&format!("ID: {}", story.id));
        println!("{}", info_line);

        let is_show_hn = story
            .title
            .as_deref()
            .map_or(false, |t| t.starts_with("Show HN:"));

        if let Some(title) = &story.title {
            println!("Title: {}", title);
        }

        if is_show_hn {
            if let Some(url) = &story.url {
                println!("URL: {}", url);
            }
        }

        if let Some(text) = &story.text {
            let document = Html::parse_fragment(text);
            let mut processed_text_intermediate = String::new();

            // Process all top-level nodes in the fragment
            for node_ref in document.root_element().children() {
                process_html_node(node_ref, &mut processed_text_intermediate);
            }

            // Decode common HTML entities
            let mut final_text = processed_text_intermediate.replace("&#x27;", "'");
            final_text = final_text.replace("&quot;", "\"");
            final_text = final_text.replace("&amp;", "&");
            final_text = final_text.replace("&#x2F;", "/");

            // Replace literal "\\n" (e.g. from bad data or earlier processing) with actual newline.
            final_text = final_text.replace("\\n", "\n");

            // Trim leading/trailing whitespace (includes newlines) from the final string.
            // Also, ensure that we don't have excessive blank lines by splitting and rejoining.
            final_text = final_text
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<&str>>()
                .join("\n");

            if !final_text.is_empty() {
                println!("Text: {}", final_text);
            }
        } else if !is_show_hn {
            // Only print URL if not a Show HN and no text
            if let Some(url) = &story.url {
                println!("URL: {}", url);
            }
        }
    }

    Ok(())
}
