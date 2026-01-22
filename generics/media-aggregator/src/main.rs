use media_aggregator::{NewsArticle, SocialPost};
use media_aggregator::{Summary, notify};

fn main() {
    let article = NewsArticle {
        headline: "Chancellor on brink of second bailout for banks".to_string(),
        location: "London".to_string(),
        author: "The Times".to_string(),
        content: "Please subscribe to Premium to read the whole story.".to_string(),
    };

    let tweet = SocialPost {
        content: "running bitcoin".to_string(),
        username: "halfin".to_string(),
        reply: false,
        repost: false,
    };

    println!("\n --- DEBUG: values ---");
    println!("{:?}", article);
    // println!("{:?}", tweet);

    println!("\n --- DEBUG: article summary ---");
    println!("{:?}", article.summarize());

    println!("\n --- DEBUG: article notify---");
    notify(&article, 5);
}
