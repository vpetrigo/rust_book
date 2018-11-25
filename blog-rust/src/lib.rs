pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approve_counter: 0,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    approve_counter: i8,
}

pub enum ApproveResult {
    Pending(PendingReviewPost),
    Approved(Post),
}

pub use ApproveResult::*;

impl PendingReviewPost {
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }

    pub fn approve(self) -> ApproveResult {
        if self.approve_counter + 1 == 2 {
            return Approved(Post {
                content: self.content
            });
        }

        Pending(PendingReviewPost {
            content: self.content,
            approve_counter: self.approve_counter + 1,
        })
    }

    pub fn publish(self) -> Option<Post> {
        if self.approve_counter >= 2 {
            return Some(Post {
                content: self.content,
            });
        }

        None
    }
}
