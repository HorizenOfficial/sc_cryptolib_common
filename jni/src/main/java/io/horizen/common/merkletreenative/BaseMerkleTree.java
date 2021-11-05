package io.horizen.common.merkletreenative;

import io.horizen.common.librustsidechains.*;

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

    private static native BaseMerkleTree nativeInit(int height, long processingStep) throws InitializationException;

    public static BaseMerkleTree init(int height, long processingStep) throws InitializationException {
        return nativeInit(height, processingStep);
    }

    public static BaseMerkleTree init(int height) throws InitializationException {
        return nativeInit(height, 1 << height);
    }

    protected native byte[] nativeSerialize();

    private void writeObject(ObjectOutputStream out) throws IOException {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        out.write(nativeSerialize());
    }

    protected static native BaseMerkleTree nativeDeserialize(byte[] serializedTree) throws DeserializationException;

    private void readObject(ObjectInputStream in) throws IOException, ClassNotFoundException {
        byte[] serialized = in.readAllBytes();
        try {
            this.inMemoryOptimizedMerkleTreePointer = nativeDeserialize(serialized).inMemoryOptimizedMerkleTreePointer;
        } catch (DeserializationException ex) {
            throw new IOException(ex.getMessage());
        }
    }

    private native void nativeAppend(FieldElement input) throws MerkleTreeException;

    @Override
    public void append(FieldElement input) throws MerkleTreeException {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        nativeAppend(input);
    }

    private native BaseMerkleTree nativeFinalize() throws FinalizationException;

    @Override
    public BaseMerkleTree finalizeTree() throws FinalizationException {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        return nativeFinalize();
    }

    private native void nativeFinalizeInPlace() throws FinalizationException;

    @Override
    public void finalizeTreeInPlace() throws FinalizationException {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        nativeFinalizeInPlace();
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
    public long getLeafIndex(FieldElement leaf) {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        return nativeGetLeafIndex(leaf);
    }

    @Override
    public boolean isLeafInTree(FieldElement leaf) {
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
    public FieldBasedMerklePath getMerklePath(FieldElement leaf) throws MerkleTreeException {        
        long leafIndex = getLeafIndex(leaf);
        if (leafIndex == -1)
            throw new IllegalStateException("Leaf not found inside tree");

        return getMerklePath(leafIndex);
    }

    private native void nativeReset();

    @Override
    public void reset() {
        if (inMemoryOptimizedMerkleTreePointer == 0)
            throw new IllegalStateException("InMemoryOptimizedMerkleTree instance was freed.");
        nativeReset();
    }

    private native void nativeFreeMerkleTree();

    @Override
    public void freeMerkleTree(){
        if (inMemoryOptimizedMerkleTreePointer != 0) {
            nativeFreeMerkleTree();
            inMemoryOptimizedMerkleTreePointer = 0;
        }
    }
}
