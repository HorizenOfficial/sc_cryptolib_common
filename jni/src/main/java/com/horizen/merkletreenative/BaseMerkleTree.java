package com.horizen.merkletreenative;

import com.horizen.librustsidechains.FieldElement;
import com.horizen.librustsidechains.Library;

import java.io.*;

public class BaseMerkleTree implements MerkleTree {
    
    protected long inMemoryOptimizedMerkleTreePointer;

    static {
        Library.load();
    }

    protected BaseMerkleTree(long inMemoryOptimizedMerkleTreePointer) {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalArgumentException("inMemoryOptimizedMerkleTreePointer must be not null.");
        this.inMemoryOptimizedMerkleTreePointer = inMemoryOptimizedMerkleTreePointer;
    }

    private BaseMerkleTree() {
        this.inMemoryOptimizedMerkleTreePointer = 0;
    }

    private static native BaseMerkleTree nativeInit(int height, long processingStep) throws MerkleTreeException;

    public static BaseMerkleTree init(int height, long processingStep) throws MerkleTreeException {
        return nativeInit(height, processingStep);
    }

    protected native byte[] nativeSerialize();

    private void writeObject(ObjectOutputStream out) throws IOException {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        out.write(nativeSerialize());
    }

    protected static native BaseMerkleTree nativeDeserialize(byte[] serializedTree) throws MerkleTreeException;

    private void readObject(ObjectInputStream in) throws IOException, ClassNotFoundException {
        byte[] serialized = in.readAllBytes();
        try {
            this.inMemoryOptimizedMerkleTreePointer = nativeDeserialize(serialized).inMemoryOptimizedMerkleTreePointer;
        } catch (MerkleTreeException ex) {
            throw new IOException(ex.getMessage());
        }
    }

    private native boolean nativeAppend(FieldElement input) throws MerkleTreeException;

    @Override
    public boolean append(MerkleTreeLeaf input) throws MerkleTreeException, MerkleTreeLeafException {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        FieldElement leafFe = input.getLeafAsFieldElement();
        boolean result = nativeAppend(leafFe);
        leafFe.freeFieldElement();
        return result;
    }

    private native BaseMerkleTree nativeFinalize() throws MerkleTreeException;

    @Override
    public BaseMerkleTree finalizeTree() throws MerkleTreeException {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        return nativeFinalize();
    }

    private native boolean nativeFinalizeInPlace() throws MerkleTreeException;

    @Override
    public boolean finalizeTreeInPlace() throws MerkleTreeException {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        return nativeFinalizeInPlace();
    }

    private native FieldElement nativeRoot() throws MerkleTreeException;

    @Override
    public FieldElement root() throws MerkleTreeException {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        return nativeRoot();
    }

    private native long nativeGetLeafIndex(FieldElement leaf);
    
    @Override
    public long getLeafIndex(MerkleTreeLeaf leaf) throws MerkleTreeLeafException {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        FieldElement leafFe = leaf.getLeafAsFieldElement();
        long idx = nativeGetLeafIndex(leaf.getLeafAsFieldElement());
        leafFe.freeFieldElement();
        return idx;
    }

    @Override
    public boolean isLeafInTree(MerkleTreeLeaf leaf) throws MerkleTreeLeafException {
        return getLeafIndex(leaf) != -1;
    }

    private native FieldBasedMerklePath nativeGetMerklePath(long leafIndex) throws MerkleTreeException;

    @Override
    public FieldBasedMerklePath getMerklePath(long leafIndex) throws MerkleTreeException {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        return nativeGetMerklePath(leafIndex);
    }

    @Override
    public FieldBasedMerklePath getMerklePath(MerkleTreeLeaf leaf) throws MerkleTreeException, MerkleTreeLeafException {        
        long leafIndex = getLeafIndex(leaf);
        if (leafIndex == -1)
            throw new IllegalStateException("Address not found inside tree");

        return getMerklePath(leafIndex);
    }

    private native void nativeReset();

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
