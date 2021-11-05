package io.horizen.common.merkletreenative;

import io.horizen.common.librustsidechains.*;
import java.io.Serializable;

/**
 * Interface for a MerkleTree accepting leaves as FieldElement.
 */
public interface MerkleTree extends AutoCloseable, Serializable {
    /**
     * Append a new leaf `input` to this instance.
     * @param input data to append to the tree
     * @throws MerkleTreeException if unable to append input leaf to this tree
     */
    void append(FieldElement input) throws MerkleTreeException;

    /**
     * Finalize the tree by computing the root and returns the finalized tree. It is possible
     * to continue updating the original tree. 
     * @return the finalized tree
     * @throws FinalizationException if unable to finalize the tree
     */
    MerkleTree finalizeTree() throws FinalizationException;

    /**
     * Finalize the tree by computing the root and updates the actual instance. It is not possible
     * to continue updating the tree, unless by restoring the original state (by calling reset()).
     * Return True if the tree has been correctly finalized, False otherwise.
     * @throws FinalizationException if unable to finalize this tree
     */
     void finalizeTreeInPlace() throws FinalizationException;

    /**
     * Returns the root of the Merkle Tree. This function must be called on a finalized tree.
     * If not, the call will result in an exception.
     * @return the root of the finalized tree
     * @throws MerkleTreeException if unable to get the root
     */
     FieldElement root() throws MerkleTreeException;

    /**
     * Get the index of the leaf in the tree
     * @return the index of the leaf in the tree if present, -1 otherwise.
     */
     long getLeafIndex(FieldElement leaf);

    /**
     * Check if leaf is in the tree
     * @return true if leaf is present in tree, false otherwise.
     */
     boolean isLeafInTree(FieldElement leaf);

    /**
     * Compute and return the MerklePath from the leaf at `leafIndex` to the root of the tree.
     * @return the MerklePath 
     * @throws MerkleTreeException if it was not possible to get the MerklePath
     */
     FieldBasedMerklePath getMerklePath(long leafIndex) throws MerkleTreeException;

    /**
    * Compute and return the MerklePath from 'leaf' to the root of the tree.
    * @return the MerklePath 
    * @throws MerkleTreeException if it was not possible to get the MerklePath
    */
     FieldBasedMerklePath getMerklePath(FieldElement leaf) throws MerkleTreeException;

    /**
     * Restore the internal state of this instance to its initial one.
     */
     void reset();

    /**
     * Free memory Rust side
     */
     void freeMerkleTree();

    /**
     * Downcast exception to MerkleTreeException
     */
    @Override
    default void close() {
        this.freeMerkleTree();
    }
}