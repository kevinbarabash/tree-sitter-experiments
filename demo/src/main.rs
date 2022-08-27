fn main() {
    let code = "x := 1 * 2 + 5 - 4";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(tree_sitter_imp::language())
        .expect("Error loading imp grammar");

    let tree = parser.parse(code, None).unwrap();
    println!("tree = {tree:#?}");

    let root = tree.root_node();
    println!("root = {root:#?}");
    println!("child_count = {}", root.child_count());

    let child = root.child(0).unwrap();
    println!("child = {child:#?}");
    println!("child_count = {}", child.child_count());

    let child = child.child(0).unwrap();
    println!("child = {child:#?}");
    println!("child_count = {}", child.child_count());

    for i in 0..3 {
        let child = child.child(i).unwrap();
        println!("child[{i}] = {child:#?}");
    }
}
