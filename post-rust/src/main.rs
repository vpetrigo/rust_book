extern crate blog_rust;

use blog_rust::*;

fn process_pending(mut post: PendingReviewPost) -> Post {
    loop {
        post = match post.approve() {
            Approved(p) => return p,
            Pending(p) => p,
        };
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("Hello World!");

    let post = post.request_review();
    let post = process_pending(post);

    assert_eq!("Hello World!", post.content());
}
