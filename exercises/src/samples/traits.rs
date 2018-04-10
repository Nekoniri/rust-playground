use std::fmt::Display;
use std::fmt::Debug;

trait Summarizable {
    fn summary(&self) -> String {
        String::from("Read more...")
    }
}

#[derive(Debug)]
struct Tweet {
    is_retweet: bool,
    content: String,
    user: String,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{} by @{}", self.content, self.user)
    }
}

#[derive(Debug)]
struct NewsArticle {
    title: String,
    content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}", self.title)
    }
}

#[derive(Debug)]
struct WeatherForecast {
    degrees: u8,
}

impl Summarizable for WeatherForecast {}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub fn traits() {
    let my_tweet = Tweet {
        is_retweet: false,
        content: String::from("My tweet's content"),
        user: String::from("ykcin"),
    };

    let my_article = NewsArticle {
        title: String::from("The title of my news article"),
        content: String::from("The content of my news article"),
    };

    let my_forecast = WeatherForecast { degrees: 30 };

    notify_debug(my_tweet);
    notify_debug(my_article);
    notify_debug(my_forecast);
}

fn notify<T: Summarizable>(item: T) {
    println!("{}", item.summary());
}

fn notify_debug<T>(item: T)
where
    T: Summarizable + Debug,
{
    println!("{} is the summary of {:#?}", item.summary(), item);
}
