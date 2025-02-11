use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::u8;


pub const GRAMMAR_ENTRY: &str = "Start";

#[derive(Debug)]
pub enum GrammarRule {
    Terminal(String),
    NonTerminal(String),
}

pub type Grammar = HashMap<String, Vec<Vec<GrammarRule>>>;

pub fn create_cgi_grammar() -> Grammar {
    /*
    Grammar definition from https://www.fuzzingbook.org/html/Grammars.html
    "<start>":
        ["<string>"],
    "<string>":
        ["<letter>", "<letter><string>"],
    "<letter>":
        ["<plus>", "<percent>", "<other>"],
    "<plus>":
        ["+"],
    "<percent>":
        ["%<hexdigit><hexdigit>"],
    "<hexdigit>":
        ["0", "1", "2", "3", "4", "5", "6", "7",
            "8", "9", "a", "b", "c", "d", "e", "f"],
    "<other>":  # Actually, could be _all_ letters
        ["0", "1", "2", "3", "4", "5", "a", "b", "c", "d", "e", "-", "_"],
     */
    let mut grammar = HashMap::new();
    grammar.insert(GRAMMAR_ENTRY.to_string(), vec![
        vec![GrammarRule::NonTerminal("String".to_string())],
    ]);
    grammar.insert("String".to_string(), vec![
        vec![GrammarRule::NonTerminal("Letter".to_string())],
        vec![GrammarRule::NonTerminal("Letter".to_string()), GrammarRule::NonTerminal("String".to_string())],
    ]);
    grammar.insert("Letter".to_string(), vec![
        vec![GrammarRule::NonTerminal("Plus".to_string())],
        vec![GrammarRule::NonTerminal("Percent".to_string())],
        vec![GrammarRule::NonTerminal("Other".to_string())], 
    ]);
    grammar.insert("Plus".to_string(), vec![
        vec![GrammarRule::Terminal("+".to_string())],
    ]);
    grammar.insert("Percent".to_string(), vec![
        vec![GrammarRule::Terminal("%".to_string()), GrammarRule::Terminal("MAGIC-HexDigit".to_string()), GrammarRule::Terminal("MAGIC-HexDigit".to_string())],
    ]);
    grammar.insert("Other".to_string(), vec![
        vec![GrammarRule::Terminal("MAGIC-AsciiDigit".to_string())],
        vec![GrammarRule::Terminal("MAGIC-LowerCaseLetter".to_string())],
        vec![GrammarRule::Terminal("MAGIC-HexDigit".to_string())], 
    ]);
    grammar
}

#[allow(unused)]
pub fn create_grammar_calculation() -> Grammar {
    let mut grammar = HashMap::new();
    grammar.insert(GRAMMAR_ENTRY.to_string(), vec![
        vec![GrammarRule::NonTerminal("Term".to_string()), GrammarRule::NonTerminal("ExprTail".to_string())],
    ]);
    grammar.insert("ExprTail".to_string(), vec![
        vec![GrammarRule::Terminal("+".to_string()), GrammarRule::NonTerminal("Term".to_string()), GrammarRule::NonTerminal("ExprTail".to_string())],
        vec![GrammarRule::Terminal("-".to_string()), GrammarRule::NonTerminal("Term".to_string()), GrammarRule::NonTerminal("ExprTail".to_string())],
        vec![],
    ]);
    grammar.insert("Term".to_string(), vec![
        vec![GrammarRule::NonTerminal("Factor".to_string()), GrammarRule::NonTerminal("TermTail".to_string())],
    ]);
    grammar.insert("TermTail".to_string(), vec![
        vec![GrammarRule::Terminal("*".to_string()), GrammarRule::NonTerminal("Factor".to_string()), GrammarRule::NonTerminal("TermTail".to_string())],
        vec![GrammarRule::Terminal("/".to_string()), GrammarRule::NonTerminal("Factor".to_string()), GrammarRule::NonTerminal("TermTail".to_string())],
        vec![],
    ]);
    grammar.insert("Factor".to_string(), vec![
        vec![GrammarRule::Terminal("MAGIC-u8".to_string())],
        vec![GrammarRule::Terminal("(".to_string()), GrammarRule::Terminal(")".to_string())],
    ]);
    grammar
}

/**
 * Helper function for grammar generation.
 * Defines special strings, which will be substituted in the generation phase.
 * Either a new created String will be returned, or None if the expression cannot be unfolded.
 */
fn match_special_expression(expr: &str, rng: &mut impl Rng) -> Option<String> {
    match expr {
        "MAGIC-u8" => {
            Some(rng.gen_range(00..u8::MAX).to_string())
        },
        "MAGIC-u16" => {
            Some(rng.gen_range(00..u16::MAX).to_string())
        },
        "MAGIC-u32" => {
            Some(rng.gen_range(00..u32::MAX).to_string())
        },
        "MAGIC-AsciiDigit" => {
            Some( (rng.gen_range(0..10) as u8 + b'0').to_string() )
        },
        "MAGIC-LowerCaseLetter" => {
            Some(((rng.gen_range(0..26) as u8 + b'a') as char).to_string() )
        },
        "MAGIC-HexDigit" => {
            let mut num = rng.gen_range(0..16) as u8;
            if num > 10 {
                num = (num - 10) + b'A';
            } else {
                num = num + b'0';
            }
            Some((num as char).to_string())
        },
        _ => None
    }
}

/**
 * Generates an input from the given grammar
 */
pub fn generate(grammar: &Grammar, symbol: &str, rng: &mut impl Rng) -> String {
    //println!("Start generating");
    if let Some(rules) = grammar.get(symbol) {
        // Choose a random rule
        let rule = rules.choose(rng).unwrap();
        //println!("Chose rule: {:?}", rule);
        rule.iter()
            .map(|r| match r {
                GrammarRule::Terminal(ref t) => {
                    // Replace NUMBER with a random number for example
                    let s = match_special_expression(t, rng);
                    match s {
                        Some(s) => {
                            s
                        },
                        None => {
                            t.clone()
                        }
                    }

                }
                GrammarRule::NonTerminal(ref nt) => generate(grammar, nt, rng),
            })
            .collect::<Vec<String>>()
            .join("")
    } else {
        String::new()
    }
}

mod tests {
    #[allow(unused_imports)]
    use crate::grammar::generate;
    #[allow(unused_imports)]
    use super::{create_cgi_grammar, create_grammar_calculation};    

    #[test]
    fn test_grammar_calculation() {
        let mut rng = rand::thread_rng();
        let grammar = create_grammar_calculation();
        let output_count = 10; // Number of outputs to generate
    
        for _ in 0..output_count {
            let output = generate(&grammar, "Start", &mut rng);
            assert!(!output.is_empty());
            println!("{}", output);
        }    
    }

    #[test]
    fn test_cgi_grammar() {
        let mut rng = rand::thread_rng();
        let grammar = create_cgi_grammar();
        let output_count = 100; // Number of outputs to generate
    
        for _ in 0..output_count {
            let output = generate(&grammar, "Start", &mut rng);
            assert!(!output.is_empty());
            println!("{}", output);
        }   
    }


}