package com.horizen.common.merkletreenative;

import com.horizen.common.librustsidechains.FieldElement;
import java.io.Serializable;

/**
 * Interface for a MerkleTree accepting leaves of arbitrary type but internally represented
 * as Field Elements. Data returned by tree will always be of type FieldElement.
 */
public interface MerkleTree extends AutoCloseable, Serializable {
    /**
     * Append a new leaf `input` to this instance.
     * @param input data to append to the tree
     * @throws MerkleTreeException if unable to append input leaf to this tree
     * @throws MerkleTreeLeafException for any error during conversion from MerkleTreeLeaf to FieldElement
     */
    void append(MerkleTreeLeaf input) throws MerkleTreeException, MerkleTreeLeafException;

    /*
     * Finalize the tree by computing the root and returns the finalized tree. It is possible
     * to continue updating the original tree.
     * Return NULL if it was not possible to finalize the tree.
     */
    MerkleTree finalizeTree() throws MerkleTreeException;

    /**
     * Finalize the tree by computing the root and updates the actual instance. It is not possible
     * to continue updating the tree, unless by restoring the original state (by calling reset()).
     * Return True if the tree has been correctly finalized, False otherwise.
     *
     * @throws MerkleTreeException if unable to finalize this tree
     */
     void finalizeTreeInPlace() throws MerkleTreeException;


    /* Returns the root of the Merkle Tree. This function must be called on a finalized tree.
     * If not, the call will result in an exception.
     * Return NULL if it was not possible to get the root.
     */
     FieldElement root() throws MerkleTreeException;

    /**
     * Return the index of the leaf in the tree if present, -1 otherwise.
     */
     long getLeafIndex(MerkleTreeLeaf leaf) throws MerkleTreeLeafException;

    /**
     * Return true if leaf is present in tree, false otherwise.
     */
     boolean isLeafInTree(MerkleTreeLeaf leaf) throws MerkleTreeLeafException;

    /*
    * Compute and return the MerklePath from the leaf at `leafIndex` to the root of the tree.
    * Return NULL if it was not possible to get the MerklePath.
    */
     FieldBasedMerklePath getMerklePath(long leafIndex) throws MerkleTreeException;

    /*
    * Compute and return the MerklePath from 'leaf' to the root of the tree.
    * Return NULL if it was not possible to get the MerklePath.
    */
     FieldBasedMerklePath getMerklePath(MerkleTreeLeaf leaf) throws MerkleTreeException, MerkleTreeLeafException;

    /*
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
    default void close() throws MerkleTreeException {
        this.freeMerkleTree();
    }
}