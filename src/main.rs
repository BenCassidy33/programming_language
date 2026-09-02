use programming_language::lexer;

#[derive(rust_embed::RustEmbed)]
#[folder = "examples/"]
struct Example;

fn main() {
    let embed = Example::get("fib.pl").unwrap();
    let s = str::from_utf8(&embed.data).unwrap();
    let res = lexer::lex(s);
    dbg!(res);
}
