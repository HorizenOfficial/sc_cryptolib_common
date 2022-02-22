package io.horizen.common.merkletreenative;

import java.util.Map;
import java.util.Set;

import io.horizen.common.librustsidechains.FieldElement;

/**
 * Interface for a Merkle Tree with possibility to insert and remove leaves in any position
 */
public interface SparseMerkleTree extends MerkleTreeCommon {

    /**
     * Add the specified leaves at the specified positions inside the tree.
     * No internal updates in the tree will be triggered by this operation.
     * @param leaves the leaves to be added to the tree and their corresponding index
     * @throws MerkleTreeException if one of the indices is > 2^height - 1
     */
    public void addLeaves(Map<Long, FieldElement> leaves) throws MerkleTreeException;

    /**
     * Remove the specified leaves at the specified positions inside the tree.
     * No internal updates in the tree will be triggered by this operation.
     * @param positions a set of the indices of the leaves to be removed
     * @throws MerkleTreeException if one of the indices is > 2^height - 1 or if attempting
     * to remove a non-existing leaf.
     */
    public void removeLeaves(Set<Long> positions) throws MerkleTreeException;

    /**
     * Check if specified position is empty
     * @param position the index of the leaf to check
     * @return True if no leaf is allocated at that position, False otherwise
     * @throws MerkleTreeException If position > 2^height - 1
     */
    public boolean isPositionEmpty(long position) throws MerkleTreeException;
}
