#![feature(plugin)]
#![plugin(lint_plugin_test)]

fn lintme() { }

fn main() {
    lintme();
    println!("whee!")
}
