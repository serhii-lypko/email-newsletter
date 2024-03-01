/*
    We will try to build an email newsletter service that supports what
    you need to get off the ground if you are willing to add an email
    subscription page to your blog - nothing more, nothing less.


    -- User stories --
    • As a blog visitor,
    I want to subscribe to the newsletter,
    So that I can receive email updates when new content is published on the blog;

    • As the blog author,
    I want to send an email to all my subscribers,
    So that I can notify them when new content is published;

    • As a subscriber,
    I want to be able to unsubscribe from the newsletter,
    So that I can stop receiving email updates from the blog.
*/

#[tokio::main]
async fn main() -> std::io::Result<()> {
    email_newsletter::run()?.await
}
