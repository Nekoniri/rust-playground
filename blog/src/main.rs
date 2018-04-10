mod state_type;

use state_type::Post;

fn main() {
    // DraftPost struct
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    // PendingReviewPost struct
    let post = post.request_review();

    // Post struct
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
