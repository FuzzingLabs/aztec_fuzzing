use anyhow::Result;

fn main() -> Result<()> {
    tree_splicer::cli::main(
        tree_sitter_noir::language(),
        tree_sitter_noir::NODE_TYPES,
    )
}
