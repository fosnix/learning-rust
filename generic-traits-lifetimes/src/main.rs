mod generic;
mod traits;

use generic::Point;
use generic::max;

use traits::NewsArticle;
use traits::Tweet;
use traits::Summary;
use traits::notify;
use traits::returns_summarizeable;
use traits::Pair;

fn main() {

    let list  = vec!['u', 'y', 'a', 'd'];
    println!("{}", max(&list));
    println!("{:?}", list);

    let point_1 = Point{x : 22.2, y : 33.6};
    let point_2 = Point{x : 10, y : 20};

    // This function is only available for functions for floating types ->
    point_1.get_point();

    println!("{:?}", point_1.get_x());
    println!("{:?}", point_2.get_y());
    
    let tw = Tweet {
        username : String::from("Uday Tiwari"),
        content : String::from("Learning Rust!"),
        is_reply : false,
        is_retweet : false,
    };

    let tw1 = Tweet {
        username : String::from("Uday Tiwari"),
        content : String::from("Learning Rust!"),
        is_reply : false,
        is_retweet : false,
    };

    let news = NewsArticle {
        author : String::from("Jakes"),
        headline : String::from("Sky is falling"),
        content : String::from("Sky is not actually falling"),
    };

    println!("Tweet Summary : {}", tw.summarize());
    println!("NewsArticle Summary : {}", news.summarize());
    println!("{}", tw.whois_auth());
    println!("{}", news.whois_auth());

    notify(&news);

    println!("{}", returns_summarizeable().summarize());

    let p1 = Pair::new(12.3, 33.6);
    p1.cmp_display(); // Because f64 impl Display and PartialOrd traits!

    let p2 = Pair::new(tw, tw1);
    // p2.cmp_display(); // Error : Because 'Tweet' does not impl Display and PartialOrd traits!
    
}  