use markdown::{
    mdast::{Node, Text},
    to_mdast,
};
use regex::Regex;
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

#[test]
fn ci_badge() {
    let readme = read_to_string(PathBuf::from_iter(["..", "README.md"])).unwrap();
    let regex = Regex::new("(?m)/actions/workflows/(.*?)(?:/|$)").unwrap();
    let filename = regex
        .captures_iter(&readme)
        .map(|captures| captures.get(1).expect("a capture").as_str())
        .reduce(|prev, curr| {
            assert_eq!(prev, curr);
            prev
        })
        .expect("at least one match");
    let path = PathBuf::from_iter(["..", ".github", "workflows", filename]);
    assert!(Path::new(&path).exists());
}

#[test]
// because markdown_toc does the wrong thing with some characters when generating links
fn limited_character_set_in_headings() {
    let readme = read_to_string(PathBuf::from_iter(["..", "README.md"])).unwrap();
    let ast = to_mdast(&readme, &Default::default()).unwrap();
    ast.children()
        .unwrap()
        .iter()
        .filter_map(|node| match node {
            Node::Heading(heading) => Some(heading),
            _ => None,
        })
        .for_each(|heading| {
            let [Node::Text(Text { value, .. })] = heading.children.as_slice() else {
                panic!("heading should be one text node but is: {heading:?}");
            };

            assert!(
                value
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == ' ' || c == '_' || c == '-'),
                "{}",
                value
            );
        });
}
