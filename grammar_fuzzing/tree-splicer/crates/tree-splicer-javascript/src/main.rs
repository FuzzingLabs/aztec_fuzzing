use anyhow::Result;

fn main() -> Result<()> {
    tree_splicer::cli::main(
        tree_sitter_javascript::language(),
        tree_sitter_javascript::NODE_TYPES,
    )
}
