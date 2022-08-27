const Parser = require("web-tree-sitter");

const main = async () => {
    await Parser.init();

    const parser = new Parser();

    const Imp = await Parser.Language.load("./tree-sitter-imp.wasm");
    parser.setLanguage(Imp);

    const code = "x := 1 * 2 + 5 - 4";
    const tree = parser.parse(code);

    console.log(tree.rootNode);
    console.log(tree.rootNode.type);
};

main().then(() => console.log("EOL"));
