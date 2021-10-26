package com.horizen.common.merkletreenative;

import com.horizen.common.librustsidechains.*;

import org.junit.Test;
import org.junit.Before;
import org.junit.After;

import java.util.List;

import java.util.ArrayList;

import java.io.*;

import static org.junit.Assert.*;

public class MerkleTreeTest {

    static long[] positions = { 458L, 478L, 161L, 0L, 291L, 666L, 313L, 532L };
    static int height = 10;
    static int numLeaves = 8;
    List<FieldElement> leaves;

    static byte[] expectedRootBytes = {
        111, -105, 99, -40, -16, -11, 28, -97, 49, -96, 37, -33, 14, 107, 81, 26,
        -108, -75, 18, -77, -34, 5, 126, 10, -6, -33, -47, 126, 64, 24, -41, 42
    };
    FieldElement expectedRoot;

    private List<FieldElement> buildLeaves(long initialSeed){
        List<FieldElement> leaves = new ArrayList<>();

        for (int i = 0; i < numLeaves; i++) {
            FieldElement leaf = FieldElement.createRandom(initialSeed);
            leaves.add(leaf);
            initialSeed += 1;
        }

        return leaves;
    }

    @Before
    public void initTestParams() throws Exception {
        leaves = buildLeaves(1234567890L);
        expectedRoot = FieldElement.deserialize(expectedRootBytes);
    }

    @Test
    public void testMerkleTrees() throws Exception {

        //Get InMemoryOptimizedMerkleTree
        BaseMerkleTree mht = BaseMerkleTree.init(height, numLeaves);

        // Must place the leaves at the same positions of the previous trees
        List<FieldElement> mhtLeaves = new ArrayList<>();
        //Initialize all leaves to zero
        FieldElement zero = FieldElement.createFromLong(0L);
        for(int j = 0; j < 1024; j++)
            mhtLeaves.add(zero);
        //Substitute at positions the correct leaves
        for (int j = 1; j < numLeaves - 1; j++) {
            // Warning: Conversion from long to int is not to be used for production.
            mhtLeaves.set((int)positions[j], leaves.get(j));
        }

        //Append all the leaves to mht
        for (FieldElement leaf: mhtLeaves)
            mht.append(leaf.clone());

        //Finalize the tree
        mht.finalizeTreeInPlace();

        //Compute root and assert equality with the expected one
        FieldElement mhtRoot = mht.root();
        assertEquals("InMemoryOptimizedMerkleTree root is not as expected", mhtRoot, expectedRoot);

        //It is the same with finalizeTree()
        BaseMerkleTree mhtCopy = mht.finalizeTree();

        FieldElement mhtRootCopy = mhtCopy.root();
        assertEquals("InMemoryOptimizedMerkleTree copy root is not as expected", mhtRootCopy, expectedRoot);

        //Free memory
        zero.freeFieldElement();
        mht.freeMerkleTree();
        mhtCopy.freeMerkleTree();
        mhtRoot.freeFieldElement();
        mhtRootCopy.freeFieldElement();
    }

    @Test
    public void testTreeExceptions() throws Exception {

        // Attempt to init an invalid Merkle Tree
        try {
            BaseMerkleTree.init(100, 1 << 100);
            assertTrue("Must be unable to init a MerkleTree with unsupported height", false);
        } catch (InitializationException mte) {
            assertTrue(mte.getMessage().contains("Unsupported height"));
        }

        // Init valid tree
        BaseMerkleTree mht = BaseMerkleTree.init(height, numLeaves);

        // Attempt to get root of a non-finalized tree
        try {
            mht.root();
            assertTrue("Must be unable to get the root of a non-finalized tree", false);
        } catch (MerkleTreeException mte) {
            assertTrue(mte.getMessage().contains("Unable to get MerkleTree root"));
        }

        // Attempt to get merkle path of a non-finalized tree
        try {
            mht.getMerklePath(0);
            assertTrue("Must be unable to get a Merkle Path from of a non-finalized tree", false);
        } catch (MerkleTreeException mte) {
            assertTrue(mte.getMessage().contains("Unable to get MerklePath"));
        }

        mht.freeMerkleTree();
    }

    @Test
    public void testTreeSerializeDeserialize() throws Exception {

        byte[] treeBytes;
        FieldElement treeRoot;

        try(BaseMerkleTree tree = BaseMerkleTree.init(height, numLeaves)) {
            for (FieldElement leaf: leaves)
                tree.append(leaf.clone());

            tree.finalizeTreeInPlace();
            treeRoot = tree.root();

            try (
                ByteArrayOutputStream bos = new ByteArrayOutputStream();
                ObjectOutputStream out = new ObjectOutputStream(bos)
            ) {   
                out.writeObject(tree);
                treeBytes =  bos.toByteArray();
            }
        }

        try (
            ByteArrayInputStream bis = new ByteArrayInputStream(treeBytes);
            ObjectInputStream in = new ObjectInputStream(bis);
            BaseMerkleTree treeDeserialized = (BaseMerkleTree)in.readObject()
        ) {
            FieldElement expectedRoot = treeDeserialized.root();
            assertEquals(expectedRoot, treeRoot);

            expectedRoot.freeFieldElement();
            treeRoot.freeFieldElement();

            for (FieldElement leaf: leaves)
                assertTrue(treeDeserialized.isLeafInTree(leaf.clone()));
        }
    }

    @Test
    public void testMerklePaths() throws Exception {
        List<FieldElement> testLeaves = new ArrayList<>();
        BaseMerkleTree mht = BaseMerkleTree.init(6, numLeaves);

        int numLeaves = 64;

        // Append leaves to the tree
        for (int i = 0; i < numLeaves/2; i ++) {
            FieldElement leaf = FieldElement.createRandom(i);
            testLeaves.add(leaf);
            mht.append(leaf.clone());
        }
        for (int i = numLeaves/2; i < numLeaves; i ++) {
            FieldElement leaf = FieldElement.createFromLong(0L);
            testLeaves.add(leaf);
        }

        //Finalize the tree and get the root
        mht.finalizeTreeInPlace();
        FieldElement mhtRoot = mht.root();

        for (int i = 0; i < numLeaves; i ++) {

            // Get/Verify Merkle Path
            FieldBasedMerklePath path = mht.getMerklePath((long)i);
            assertTrue("Merkle Path must be verified", path.verify(testLeaves.get(i), mhtRoot));

            // Serialization/Deserialization test
            byte[] merklePathBytes = path.serialize();
            FieldBasedMerklePath pathDeserialized = FieldBasedMerklePath.deserialize(merklePathBytes);
            assertTrue("Deserialized Merkle Path must be verified", pathDeserialized.verify(testLeaves.get(i), mhtRoot));

            if (i == 0) { // leftmost check
                assertTrue("Path must be the leftmost", path.isLeftmost());
            }
            else if (i == (numLeaves / 2) - 1) { // areRightLeavesEmpty check
                assertTrue("Right leaves must be all empty", path.areRightLeavesEmpty());
            }
            else if (i == numLeaves - 1) { //rightmost check
                assertTrue("Path must be the rightmost", path.isRightmost());
            }
            else { // Other cases check
                assertFalse("Path must not be the leftmost", path.isLeftmost());
                assertFalse("Path must not be the rightmost", path.isRightmost());

                if (i < (numLeaves / 2) - 1) {
                    assertFalse("Right leaves must not be all empty", path.areRightLeavesEmpty());
                }
            }

            assertEquals("Leaf index computed from path must be correct", i, path.leafIndex());

            // apply() test
            FieldElement rootComputed = path.apply(testLeaves.get(i));
            assertEquals("Root computed out of Merkle Path must be the same", rootComputed, mhtRoot);
            rootComputed.freeFieldElement();

            // Free paths
            path.freeMerklePath();
            pathDeserialized.freeMerklePath();
        }

        // Free memory
        mht.freeMerkleTree();
        mhtRoot.freeFieldElement();
        for (FieldElement leaf: testLeaves)
            leaf.freeFieldElement();
    }

    @Test
    public void testMerklePathExceptions() throws Exception {

        // Init, finalize and get merkle path of a valid tree
        BaseMerkleTree mht = BaseMerkleTree.init(height, numLeaves);
        mht.finalizeTreeInPlace();
        FieldBasedMerklePath path = mht.getMerklePath(0);
        FieldElement leaf = FieldElement.createFromLong(0L);
        FieldElement root = mht.root();

        try {
            path.verify(height + 1, leaf, root);
            assertTrue("Must not be able to verify a Merkle Path smaller than tree height", false);
        } catch (MerklePathException mpe) {
            assertTrue(mpe.getMessage().contains("IncorrectPathLength"));
        }

        mht.freeMerkleTree();
        path.freeMerklePath();
        leaf.freeFieldElement();
        root.freeFieldElement();
    }

    @Test
    public void testAreRightLeavesEmpty() throws Exception {
        BaseMerkleTree mht = BaseMerkleTree.init(6, numLeaves);

        int numLeaves = 64;

        // Generate random leaves
        for (int i = 0; i < numLeaves; i ++) {
            FieldElement leaf = FieldElement.createRandom(i);
            mht.append(leaf.clone());

            BaseMerkleTree mhtCopy = mht.finalizeTree();

            FieldBasedMerklePath path = mhtCopy.getMerklePath((long)i);
            assertTrue(path.areRightLeavesEmpty());

            leaf.freeFieldElement();
            path.freeMerklePath();
            mhtCopy.freeMerkleTree();
        }
    }

    @After
    public void freeTestParams(){
        for (FieldElement leaf: leaves)
            leaf.freeFieldElement();
        expectedRoot.freeFieldElement();
    }
}
