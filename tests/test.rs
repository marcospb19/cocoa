// use cocoa::grammar::{CacauParser, Rule};
// use glob::glob;
// use pest::Parser;

// #[test]
// fn test_cacau_passes() {
//     for file in glob("tests/pass/*").unwrap() {
//         let file = file.unwrap();
//         let text = utils::read_file(file);

//         let result = CacauParser::parse(Rule::Root, &text);
//         if let Err(_) = result {
//             panic!("expected a pass, but parsing failed: {result:#?}");
//         }
//     }
// }

// #[test]
// fn test_cacau_fails() {
//     for file in glob("tests/fail/*").unwrap() {
//         let file = file.unwrap();
//         let text = utils::read_file(file);

//         let result = CacauParser::parse(Rule::Root, &text);
//         if let Ok(_) = result {
//             panic!("expected a fail, but parsing passed: {result:#?}");
//         }
//     }
// }

// mod utils {
//     use std::path::Path;

//     use fs_err as fs;

//     pub fn read_file(path: impl AsRef<Path>) -> String {
//         fs::read_to_string(path.as_ref()).unwrap_or_else(|err| {
//             panic!(
//                 "Failed to read file '{}': {err}.",
//                 path.as_ref().to_str().unwrap()
//             );
//         })
//     }
// }
