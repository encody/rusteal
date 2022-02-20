extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

// use rusteal_ast;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct RtealParser;

#[cfg(test)]
mod tests {
    use std::fs;

    use pest::Parser;

    use crate::RtealParser;
    use crate::Rule;

    #[test]
    fn test() {
        let unparsed_file = fs::read_to_string("examples/1.rteal").expect("could not open file");
        let file = RtealParser::parse(Rule::file, &unparsed_file)
            .expect("successful parse")
            .next()
            .unwrap();

        println!("{:?}", file);
    }
}
