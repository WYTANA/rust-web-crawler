use html5ever::tokenizer::{
    BufferQueue,
    Tag,
    TagKind,
    Token,
    TokenSink,
    TokenSinkResult,
    Tokenizer,
    TokenizerOpts, // TagToken
};
use std::borrow::Borrow;
use url::{ParseError, Url};

#[derive(Debug, Default)]
struct LinkQueue {
    links: Vec<String>,
}

fn main() {
    println!("Hello World!");
}
