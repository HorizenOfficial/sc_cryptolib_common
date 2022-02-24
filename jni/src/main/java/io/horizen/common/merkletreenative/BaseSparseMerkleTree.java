package io.horizen.common.merkletreenative;

import java.io.IOException;
import java.io.ObjectInputStream;
import java.io.ObjectOutputStream;
import java.util.Map;
import java.util.Set;

import io.horizen.common.librustsidechains.FieldElement;
import io.horizen.common.librustsidechains.FinalizationException;
import io.horizen.common.librustsidechains.Library;

public class BaseSparseMerkleTree implements SparseMerkleTree {

    private long merkleTreePointer;

    static {
        Library.load();
    }

    private BaseSparseMerkleTree(long merkleTreePointer) {
        if (merkleTreePointer == 0)
            throw new IllegalArgumentException("merkleTreePointer must be not null.");
        this.merkleTreePointer = merkleTreePointer;
    }

    private static native BaseSparseMerkleTree nativeInit(int height);

    /**
     * Initialize an instance of BaseSparseMerkleTree with specified height
     * @param height the height of the tree
     * @return an instance of BaseSparseMerkleTree, allocated Rust side
     */
    public static BaseSparseMerkleTree init(int height) {
        return nativeInit(height);
    }

    private native void nativeAddLeaves(Map<Long, FieldElement> leaves) throws MerkleTreeException;

    @Override
    public void addLeaves(Map<Long, FieldElement> leaves) throws MerkleTreeException {
        if (merkleTreePointer == 0)
            throw new IllegalStateException("BaseSparseMerkleTree instance was freed.");
        nativeAddLeaves(leaves);
    }

    // TODO: Currently we don't have good utilities to deal with a Set Rust-side.
    //       For the moment, let's pass an array, that will be converted back to
    //       a Set Rust side
    private native void nativeRemoveLeaves(Long[] positions) throws MerkleTreeException;

    @Override
    public void removeLeaves(Set<Long> positions) throws MerkleTreeException{
        if (merkleTreePointer == 0)
            throw new IllegalStateException("BaseSparseMerkleTree instance was freed.");
        nativeRemoveLeaves(positions.toArray(new Long[0]));
    }

    private native BaseSparseMerkleTree nativeFinalize() throws FinalizationException;

    @Override
    public BaseSparseMerkleTree finalizeTree() throws FinalizationException {
        if (merkleTreePointer == 0)
            throw new IllegalStateException("BaseSparseMerkleTree instance was freed.");
        return nativeFinalize();
    }

    private native void nativeFinalizeInPlace() throws FinalizationException;

    @Override
    public void finalizeTreeInPlace() throws FinalizationException {
        if (merkleTreePointer == 0)
            throw new IllegalStateException("BaseSparseMerkleTree instance was freed.");
        nativeFinalizeInPlace();
    }

    private native void nativeReset();

    @Override
    public void reset() {
        if (merkleTreePointer == 0)
            throw new IllegalStateException("BaseSparseMerkleTree instance was freed.");
        nativeReset();
    }

    private native FieldElement nativeRoot() throws MerkleTreeException;

    @Override
    public FieldElement root() throws MerkleTreeException {
        if (merkleTreePointer == 0)
            throw new IllegalStateException("BaseSparseMerkleTree instance was freed.");
        return nativeRoot();
    }

    private native boolean nativeIsPositionEmpty(long position) throws MerkleTreeException;

    @Override
    public boolean isPositionEmpty(long position) throws MerkleTreeException {
        if (merkleTreePointer == 0)
            throw new IllegalStateException("BaseSparseMerkleTree instance was freed.");
        return nativeIsPositionEmpty(position);
    }

    private native long nativeGetLeafIndex(FieldElement leaf);
    
    @Override
    public long getLeafIndex(FieldElement leaf) {
        if (merkleTreePointer == 0)
            throw new IllegalStateException("BaseSparseMerkleTree instance was freed.");
        return nativeGetLeafIndex(leaf);
    }

    @Override
    public boolean isLeafInTree(FieldElement leaf) {
        return getLeafIndex(leaf) != -1;
    }

    private native FieldBasedMerklePath nativeGetMerklePath(long leafPosition) throws MerkleTreeException;

    @Override
    public FieldBasedMerklePath getMerklePath(long leafPosition) throws MerkleTreeException {
        if (merkleTreePointer == 0)
            throw new IllegalStateException("BaseSparseMerkleTree instance was freed.");
        return nativeGetMerklePath(leafPosition);
    }

    @Override
    public FieldBasedMerklePath getMerklePath(FieldElement leaf) throws MerkleTreeException {
        long leafIndex = getLeafIndex(leaf);
        if (leafIndex == -1)
            throw new IllegalStateException("Leaf not found inside tree");

        return getMerklePath(leafIndex);
    }

    private void writeObject(ObjectOutputStream out) throws IOException {
        throw new UnsupportedOperationException("It's not possible to serialize a BaseSparseMerkleTree at the moment.");
    }

    private void readObject(ObjectInputStream in) throws IOException, ClassNotFoundException {
        throw new UnsupportedOperationException("It's not possible to deserialize a BaseSparseMerkleTree at the moment.");
    }

    private native void nativeFreeBaseSparseMerkleTree();

    /**
     * Free memory Rust side
     */
    @Override
    public void freeMerkleTree() {
        if (merkleTreePointer != 0) {
            nativeFreeBaseSparseMerkleTree();
            merkleTreePointer = 0;
        }
    }

    @Override
    public void close() {
        freeMerkleTree();
    }
}
