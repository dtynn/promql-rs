use std::env;

use lrpar::{Node, NonStreamingLexer};
use promql_rs::{lexerdef, parse, token_epp};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("input required");
        std::process::exit(1);
    }

    let input = &args[1];
    let ldef = lexerdef();
    let lexer = ldef.lexer(input);
    let (pt, errs) = parse(&lexer);

    for e in errs {
        println!("{}", e.pp(&lexer, &token_epp));
    }

    if let Some(node) = pt {
        walk_node(node, input, &lexer, 0);
    }
}

fn walk_node<'i, L: NonStreamingLexer<'i, u32>>(
    node: Node<u32>,
    input: &str,
    l: &'i L,
    depth: usize,
) {
    match node {
        Node::Nonterm { ridx, nodes } => {
            println!("#{} RULE: {:?}", depth, ridx.0);
            for n in nodes {
                walk_node(n, input, l, depth + 1);
            }
        }

        Node::Term { lexeme } => {
            let span = lexeme.span();
            println!("#{} captured: {}", depth, l.span_str(span));
        }
    }
}
