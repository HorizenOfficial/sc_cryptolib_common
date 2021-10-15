use super::*;
use primitives::merkle_tree::*;
pub fn new_ginger_mht(height: usize, processing_step: usize) -> Result<GingerMHT, Error> {
    GingerMHT::init(height, processing_step)
}

pub fn append_leaf_to_ginger_mht(tree: &mut GingerMHT, leaf: &FieldElement) -> Result<(), Error> {
    let _ = tree.append(*leaf)?;
    Ok(())
}

pub fn finalize_ginger_mht(tree: &GingerMHT) -> Result<GingerMHT, Error> {
    tree.finalize()
}

pub fn finalize_ginger_mht_in_place(tree: &mut GingerMHT) -> Result<(), Error> {
    tree.finalize_in_place()?;
    Ok(())
}

pub fn get_ginger_mht_root(tree: &GingerMHT) -> Result<FieldElement, Error> {
    let root = tree
        .root()
        .ok_or("Unable to get root of a non finalized tree")?;
    Ok(root)
}

pub fn get_leaf_index(tree: &GingerMHT, leaf: &FieldElement) -> Option<usize> {
    // Search for address inside the leaves of the tree
    let tree_leaves = tree.get_leaves();
    tree_leaves.iter().position(|x| x == leaf)
}

pub fn get_ginger_mht_path(tree: &GingerMHT, leaf_index: u64) -> Result<GingerMHTPath, Error> {
    use std::convert::TryInto;

    let path = match tree.get_merkle_path(leaf_index as usize) {
        Some(path) => path
            .try_into()
            .map_err(|e| format!("Unable to convert to binary Merkle Path {:?}", e)),
        None => Err("Unable to get path of a non finalized tree".to_owned()),
    }?;

    Ok(path)
}

pub fn reset_ginger_mht(tree: &mut GingerMHT) {
    tree.reset();
}

pub fn verify_ginger_merkle_path(
    path: &GingerMHTPath,
    height: usize,
    leaf: &FieldElement,
    root: &FieldElement,
) -> Result<bool, Error> {
    path.verify(height, leaf, root)
}

pub fn verify_ginger_merkle_path_without_length_check(
    path: &GingerMHTPath,
    leaf: &FieldElement,
    root: &FieldElement,
) -> bool {
    path.verify_without_length_check(leaf, root)
}

pub fn is_path_leftmost(path: &GingerMHTPath) -> bool {
    path.is_leftmost()
}

pub fn is_path_rightmost(path: &GingerMHTPath) -> bool {
    path.is_rightmost()
}

pub fn are_right_leaves_empty(path: &GingerMHTPath) -> bool {
    path.are_right_leaves_empty()
}

pub fn get_leaf_index_from_path(path: &GingerMHTPath) -> u64 {
    path.leaf_index() as u64
}

pub fn get_root_from_path(path: &GingerMHTPath, leaf: &FieldElement) -> FieldElement {
    path.compute_root(leaf)
}
