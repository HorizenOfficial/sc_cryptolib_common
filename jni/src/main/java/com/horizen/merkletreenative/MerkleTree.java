package com.horizen.merkletreenative;

import java.io.Serializable;

// TODO: Instead of defining serialize() and deserialize() function use the Java Serializable interface.
//       This is true also for the other classes.
public interface MerkleTree<T> extends AutoCloseable, Serializable {

    /*
     * Append a new leaf `input` to this instance.
     * Return false if the operation was not successfull
     */
    public abstract boolean append(T input);

    /*
     * Finalize the tree by computing the root and returns the finalized tree. It is possible
     * to continue updating the original tree.
     * Return NULL if it was not possible to finalize the tree.
     */
    public abstract MerkleTree<T> finalizeTree();

    /*
     * Finalize the tree by computing the root and updates the actual instance. It is not possible
     * to continue updating the tree, unless by restoring the original state (by calling reset()).
     * Return True if the tree has been correctly finalized, False otherwise.
     */
    public abstract boolean finalizeTreeInPlace();


    /* Returns the root of the Merkle Tree. This function must be called on a finalized tree.
     * If not, the call will result in an exception.
     * Return NULL if it was not possible to get the root.
     */
    public abstract T root();

    /**
     * Return the index of the leaf in the tree if present, -1 otherwise.
     */
    public abstract long getLeafIndex(T leaf);

    /**
     * Return true if leaf is present in tree, false otherwise.
     */
    public abstract boolean isLeafInTree(T leaf);

    /*
    * Compute and return the MerklePath from the leaf at `leafIndex` to the root of the tree.
    * Return NULL if it was not possible to get the MerklePath.
    */
    public abstract FieldBasedMerklePath getMerklePath(long leafIndex);

    /*
    * Compute and return the MerklePath from 'leaf' to the root of the tree.
    * Return NULL if it was not possible to get the MerklePath.
    */
    public abstract FieldBasedMerklePath getMerklePath(T leaf);

    /*
     * Restore the internal state of this instance to its initial one.
     */
    public abstract void reset();

    /**
     * Free memory Rust side
     */
    public abstract void freeMerkleTree();
}

