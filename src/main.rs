use chrono::{TimeZone, Utc};
use clap::{Parser, Subcommand};
use ego_tree::NodeRef;
use reqwest;
use scraper::node::Node;
use scraper::{ElementRef, Html};
use serde::{Deserialize, Serialize};
use serde_json;
use html2text::from_read;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show Hacker News stories (default)
    Show {
        /// Maximum number of stories to display (default: 5, max: 25)
        #[clap(short = 'm', long = "max-stories", default_value = "5", value_parser = clap::value_parser!(u8).range(1..=25))]
        max_stories: u8,
    },
    /// Show Hacker News stories with summaries
    Summarize {
        /// Maximum number of stories to display (default: 5, max: 25)
        #[clap(short = 'm', long = "max-stories", default_value = "5", value_parser = clap::value_parser!(u8).range(1..=25))]
        max_stories: u8,
    },
    /// Run diagnostic checks
    Doctor,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

async fn check_network_connectivity(urls: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔌 Checking network connectivity...");
    
    for url in urls {
        print!("  Checking {}... ", url);
        match reqwest::get(*url).await {
            Ok(response) => {
                if response.status().is_success() {
                    println!("✅ OK");
                } else {
                    println!("⚠️  Status: {}", response.status());
                }
            }
            Err(e) => {
                println!("❌ Failed: {}", e);
            }
        }
    }
    
    Ok(())
}

async fn check_ollama_model(models: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Checking Ollama models...");
    
    let client = reqwest::Client::new();
    
    for model in models {
        print!("  Checking model '{}'... ", model);
        
        let payload = serde_json::json!({
            "model": model,
            "prompt": "test",
            "stream": false,
        });
        
        match client.post("http://localhost:11434/api/generate")
            .json(&payload)
            .send()
            .await 
        {
            Ok(response) => {
                if response.status().is_success() {
                    println!("✅ Available");
                } else {
                    println!("❌ Status: {}", response.status());
                }
            }
            Err(e) => {
                println!("❌ Failed: {}", e);
            }
        }
    }
    
    Ok(())
}

async fn run_doctor() -> Result<(), Box<dyn std::error::Error>> {
    println!("🩺 Running HNS Doctor Diagnostics");
    println!("=====================================");
    
    // Network connectivity checks
    let urls_to_check = [
        "https://hacker-news.firebaseio.com/v0/topstories.json",
        "http://localhost:11434/api/version",
    ];
    
    check_network_connectivity(&urls_to_check).await?;
    
    println!();
    
    // Model checks
    let models_to_check = [
        "gemma3:4b",
    ];
    
    check_ollama_model(&models_to_check).await?;
    
    println!();
    println!("🎉 Diagnostics complete!");
    
    Ok(())
}

async fn summarize_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Step 1: Fetch the webpage content
    print!("Fetching... ");
    let response = reqwest::get(url).await?;
    
    if !response.status().is_success() {
        return Ok(format!("Failed to fetch URL: {}", response.status()));
    }
    
    let html_content = response.text().await?;
    
    // Step 2: Convert HTML to plain text (extract main content)
    print!("Processing... ");
    let plain_text = from_read(html_content.as_bytes(), 10000);
    
    // Trim and limit the content length to avoid overwhelming the LLM
    let trimmed_text = plain_text.trim();
    let content_to_summarize = if trimmed_text.len() > 4000 {
        format!("{}...", &trimmed_text[0..4000])
    } else {
        trimmed_text.to_string()
    };
    
    // Step 3: Create the prompt for the LLM
    let prompt = format!(
        "Summarize the following article in 3-5 sentences. Only return the summary. Here is the article:\n\n{}",
        content_to_summarize
    );
    
    // Step 4: Call the LLM (gemma3:4b) via Ollama API directly
    print!("Summarizing... ");
    // Create payload for Ollama API
    let ollama_url = "http://localhost:11434/api/generate";
    let client = reqwest::Client::new();
    
    let payload = serde_json::json!({
        "model": "gemma3:4b",
        "prompt": prompt,
        "stream": false,
        "temperature": 0.7
    });
    
    // Call Ollama API
    let result = client.post(ollama_url)
        .json(&payload)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await;
    
    println!("Done.");
    // Process response
    match result {
        Ok(response) => {
            if let Some(generated_text) = response.get("response").and_then(|v| v.as_str()) {
                let summary = generated_text.trim().to_string();
                if summary.is_empty() {
                    Ok("Summary generation failed".to_string())
                } else {
                    Ok(summary)
                }
            } else {
                Ok("Failed to extract summary from response".to_string())
            }
        },
        Err(e) => {
            Ok(format!("Error generating summary: {}", e))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();

    // Determine max_stories and summarize flag based on command
    let (max_stories, summarize) = match args.command {
        Some(Commands::Show { max_stories }) => (max_stories, false),
        Some(Commands::Summarize { max_stories }) => (max_stories, true),
        Some(Commands::Doctor) => {
            match run_doctor().await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    eprintln!("Doctor diagnostics failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        None => (5, true), // Default: summarize with 5 stories
    };

    println!("Top {} Hacker News Stories:", max_stories);

    let top_story_ids = fetch_top_stories_ids().await?;

    let mut stories = Vec::new();
    let mut fetch_count = 0;
    for id in top_story_ids {
        if fetch_count >= max_stories as usize {
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
                if summarize {
                    match summarize_url(url).await {
                        Ok(summary) => println!("Summary: {}", summary),
                        Err(e) => println!("Failed to generate summary: {}", e),
                    }
                }
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
                if summarize {
                    match summarize_url(url).await {
                        Ok(summary) => println!("Summary: {}", summary),
                        Err(e) => println!("Failed to generate summary: {}", e),
                    }
                }
            }
        }
    }

    Ok(())
}
