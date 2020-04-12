use state_pattern::Post;

fn main() {

    let mut post = Post::new();

    post.add_text("Hello Post");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Hello Post", post.content());
}