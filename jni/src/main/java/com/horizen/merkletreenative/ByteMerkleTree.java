package com.horizen.merkletreenative;

import com.horizen.librustsidechains.FieldElement;
import java.io.*;

public abstract class ByteMerkleTree implements MerkleTree<byte[]> {

    FieldBasedMerkleTree tree;

    private void writeObject(ObjectOutputStream out) throws IOException {
        if (tree.inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        out.write(tree.nativeSerialize());
    }

    private void readObject(ObjectInputStream in) throws IOException, ClassNotFoundException {
        byte[] serialized = in.readAllBytes();
        this.tree.inMemoryOptimizedMerkleTreePointer = FieldBasedMerkleTree.nativeDeserialize(serialized).inMemoryOptimizedMerkleTreePointer;
    }

    /**
     * Convert input bytes into a FieldElement, according to internal policies.
     */
    protected abstract FieldElement bytesToFieldElement(byte[] bytes);

    /**
     * Append bytes to tree and return true if operation was successfull, false otherwise.
     */
    @Override
    public boolean append(byte[] bytes) {
        FieldElement fe = bytesToFieldElement(bytes);
        boolean result = this.tree.append(fe);
        fe.freeFieldElement();
        return result;
    }

    /*
     * Finalize the tree by computing the root and updates the actual instance. It is not possible
     * to continue updating the tree, unless by restoring the original state (by calling reset()).
     * Return True if the tree has been correctly finalized, False otherwise.
     */
    @Override
    public boolean finalizeTreeInPlace() {
        return this.tree.finalizeTreeInPlace();
    }

    /* Returns the root of the Merkle Tree. */
    @Override
    public byte[] root() {
        FieldElement root = this.tree.root();
        byte[] serializedRoot = root.serializeFieldElement();
        root.freeFieldElement();
        return serializedRoot;
    }

    /**
     * Return the index of the leaf in the tree if present, -1 otherwise.
     */
    @Override
    public long getLeafIndex(byte[] leaf) {
        FieldElement fe = bytesToFieldElement(leaf);
        long idx = this.tree.getLeafIndex(fe);
        fe.freeFieldElement();
        return idx;
    }

    /**
     * Returns true if "leaf" is present as a leaf in the tree, false otherwise
     */
    @Override
    public boolean isLeafInTree(byte[] leaf) {
        FieldElement fe = bytesToFieldElement(leaf);
        boolean result = this.tree.isLeafInTree(fe);
        fe.freeFieldElement();
        return result;
    }

    /*
    * Compute and return the MerklePath from the leaf at `leafIndex` to the root of the tree.
    * Return NULL if it was not possible to get the MerklePath.
    */
    @Override
    public FieldBasedMerklePath getMerklePath(long leafIndex) {
        return this.tree.getMerklePath(leafIndex);
    }

    /*
    * Compute and return the MerklePath from 'leaf' to the root of the tree.
    * Return NULL if it was not possible to get the MerklePath.
    */
    @Override
    public FieldBasedMerklePath getMerklePath(byte[] leaf){
        FieldElement fe = bytesToFieldElement(leaf);
        FieldBasedMerklePath path = this.tree.getMerklePath(fe);
        fe.freeFieldElement();
        return path; 
    }

    /*
     * Restore the internal state of this instance to its initial one.
     */
    @Override
    public void reset() {
        this.tree.reset();
    }

    /* Free the memory Rust side */
    @Override
    public void freeMerkleTree(){
        this.tree.freeMerkleTree();
    }

    @Override
    public void close() throws Exception {
        freeMerkleTree();
    }
}