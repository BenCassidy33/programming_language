use programming_language::lexer;

fn main() {
    dbg!(lexer::lex("if a == b {
    return c;
    }"));
}
