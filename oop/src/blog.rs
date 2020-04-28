pub mod blog_stuff{
    struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    }
    impl Post{
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }
        //add text to post
        pub fn add_text(&mut self, text:&str){
            self.content.push_str(text);
        }

        //return content based on it's state
        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }

        //request requiew on post
        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        //approve post
        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
        //reject post
        pub fn reject(&mut self){
            if let Some(s)=self.state.take(){
                self.state=Some(s.reject_post())
            }
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject_post(self:Box<Self>)->Box<dyn State>;
    //return detault empty string on trait
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    //change state from draft to pending
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    //return current state
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    //return current stat
    fn reject_post(self: Box<Self>)->Box<dyn State>{
       self
    }
}

struct PendingReview {}

impl State for PendingReview {
    //return current state
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    //change state to approve
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    //change state to draft 
    fn reject_post(self: Box<Self>)->Box<dyn Draft>{
        Box::new(Draft{})
    }
}
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    //return content string if content state is approved
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn reject_post(self: Box<Self>)->Box<dyn State>{
        self
     }
}
}