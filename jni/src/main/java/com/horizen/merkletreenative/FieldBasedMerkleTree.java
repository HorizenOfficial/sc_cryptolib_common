package com.horizen.merkletreenative;

import com.horizen.librustsidechains.FieldElement;
import com.horizen.librustsidechains.Library;

import java.io.*;

public class FieldBasedMerkleTree implements MerkleTree<FieldElement> {
    
    protected long inMemoryOptimizedMerkleTreePointer;

    static {
        Library.load();
    }

    protected FieldBasedMerkleTree(long inMemoryOptimizedMerkleTreePointer) {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalArgumentException("inMemoryOptimizedMerkleTreePointer must be not null.");
        this.inMemoryOptimizedMerkleTreePointer = inMemoryOptimizedMerkleTreePointer;
    }

    private FieldBasedMerkleTree() {
        this.inMemoryOptimizedMerkleTreePointer = 0;
    }

    private static native FieldBasedMerkleTree nativeInit(int height, long processingStep);

    /* Creates a new tree given its `height` and `processing_step`, that defines the
    *  number of leaves to store before triggering the computation of the hashes
    *  of the upper levels. Changing this parameter will affect the performances.
    *  Return NULL if it was not possible to initialize the tree.
    */
    public static FieldBasedMerkleTree init(int height, long processingStep){
        return nativeInit(height, processingStep);
    }

    protected native byte[] nativeSerialize();

    private void writeObject(ObjectOutputStream out) throws IOException {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        out.write(nativeSerialize());
    }

    protected static native FieldBasedMerkleTree nativeDeserialize(byte[] serializedTree);

    private void readObject(ObjectInputStream in) throws IOException, ClassNotFoundException {
        byte[] serialized = in.readAllBytes();
        this.inMemoryOptimizedMerkleTreePointer = nativeDeserialize(serialized).inMemoryOptimizedMerkleTreePointer;
    }

    private native boolean nativeAppend(FieldElement input);

    /*
     * Append a new leaf `input` to this instance.
     * Return false if the operation was not successfull
     * (for the moment this happens whenever the maximum number
     * of leaves is exceeded)
     */
    @Override
    public boolean append(FieldElement input) {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        return nativeAppend(input);
    }

    private native FieldBasedMerkleTree nativeFinalize();

    /*
     * Finalize the tree by computing the root and returns the finalized tree. It is possible
     * to continue updating the original tree.
     * Return NULL if it was not possible to finalize the tree.
     */
    @Override
    public FieldBasedMerkleTree finalizeTree() {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        return nativeFinalize();
    }

    private native boolean nativeFinalizeInPlace();

    /*
     * Finalize the tree by computing the root and updates the actual instance. It is not possible
     * to continue updating the tree, unless by restoring the original state (by calling reset()).
     * Return True if the tree has been correctly finalized, False otherwise.
     */
    @Override
    public boolean finalizeTreeInPlace() {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        return nativeFinalizeInPlace();
    }

    private native FieldElement nativeRoot();

    /* Returns the root of the Merkle Tree. This function must be called on a finalized tree.
     * If not, the call will result in an exception.
     * Return NULL if it was not possible to get the root.
     */
    @Override
    public FieldElement root() {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        return nativeRoot();
    }

    private native long nativeGetLeafIndex(FieldElement leaf);
    
    /**
     * Return the index of the leaf in the tree if present, -1 otherwise.
     */
    @Override
    public long getLeafIndex(FieldElement leaf) {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        return nativeGetLeafIndex(leaf);
    }

    /**
     * Return true if leaf is present in tree, false otherwise.
     */
    @Override
    public boolean isLeafInTree(FieldElement leaf) {
        return getLeafIndex(leaf) != -1;
    }

    private native FieldBasedMerklePath nativeGetMerklePath(long leafIndex);

    /*
    * Compute and return the MerklePath from the leaf at `leafIndex` to the root of the tree.
    * Return NULL if it was not possible to get the MerklePath.
    */
    @Override
    public FieldBasedMerklePath getMerklePath(long leafIndex) {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        return nativeGetMerklePath(leafIndex);
    }

    /*
    * Compute and return the MerklePath from 'leaf' to the root of the tree.
    * Return NULL if it was not possible to get the MerklePath.
    */
    @Override
    public FieldBasedMerklePath getMerklePath(FieldElement leaf) {        
        long leafIndex = getLeafIndex(leaf);
        if (leafIndex == -1)
            throw new IllegalStateException("Address not found inside tree");

        return getMerklePath(leafIndex);
    }

    private native void nativeReset();

    /*
     * Restore the internal state of this instance to its initial one.
     */
    @Override
    public void reset() {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        nativeReset();
    }

    private native void nativeFreeMerkleTree(long inMemoryOptimizedMerkleTreePointer);

    @Override
    public void freeMerkleTree(){
        if (inMemoryOptimizedMerkleTreePointer != 0) {
            nativeFreeMerkleTree(this.inMemoryOptimizedMerkleTreePointer);
            inMemoryOptimizedMerkleTreePointer = 0;
        }
    }

    @Override
    public void close() throws Exception {
        freeMerkleTree();
    }
}
