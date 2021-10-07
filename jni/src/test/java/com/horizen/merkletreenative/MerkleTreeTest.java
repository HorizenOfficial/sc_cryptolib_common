package com.horizen.merkletreenative;

import com.horizen.librustsidechains.FieldElement;

import org.junit.Test;
import org.junit.Before;
import org.junit.After;

import java.util.List;

import javax.sound.sampled.SourceDataLine;

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
    public void initTestParams() {
        leaves = buildLeaves(1234567890L);
        expectedRoot = FieldElement.deserialize(expectedRootBytes);
    }

    @Test
    public void testMerkleTrees() {

        //Get InMemoryOptimizedMerkleTree
        FieldBasedMerkleTree mht = FieldBasedMerkleTree.init(height, numLeaves);
        assertNotNull("Merkle Tree initialization must succeed", mht);

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
            assertTrue("Leaf append must be successfull", mht.append(leaf));

        //Finalize the tree
        assertTrue("Merkle Tree finalization must succeed", mht.finalizeTreeInPlace());

        //Compute root and assert equality with the expected one
        FieldElement mhtRoot = mht.root();
        assertNotNull("Root must not be NULL", mhtRoot);
        assertEquals("InMemoryOptimizedMerkleTree root is not as expected", mhtRoot, expectedRoot);

        //It is the same with finalizeTree()
        FieldBasedMerkleTree mhtCopy = mht.finalizeTree();
        assertNotNull("Merkle Tree finalization must succeed", mhtCopy);

        FieldElement mhtRootCopy = mhtCopy.root();
        assertNotNull("Root must not be NULL", mhtRootCopy);
        assertEquals("InMemoryOptimizedMerkleTree copy root is not as expected", mhtRootCopy, expectedRoot);

        //Free memory
        zero.freeFieldElement();
        mht.freeMerkleTree();
        mhtCopy.freeMerkleTree();
        mhtRoot.freeFieldElement();
        mhtRootCopy.freeFieldElement();
    }

    @Test
    public void testTreeSerializeDeserialize() throws Exception {

        byte[] treeBytes;
        FieldElement treeRoot;

        try(FieldBasedMerkleTree tree = FieldBasedMerkleTree.init(height, numLeaves)) {
            for (FieldElement leaf: leaves)
                assertTrue(tree.append(leaf));

            assertTrue("Merkle Tree finalization must succeed", tree.finalizeTreeInPlace());
            treeRoot = tree.root();
            assertNotNull("Must be able to get root from original tree", treeRoot);

            try (
                ByteArrayOutputStream bos = new ByteArrayOutputStream();
                ObjectOutputStream out = new ObjectOutputStream(bos)
            ) {   
                out.writeObject(tree);
                treeBytes =  bos.toByteArray();
            }
        }
        System.out.println("Tree object len: " + treeBytes.length);

        try (
            ByteArrayInputStream bis = new ByteArrayInputStream(treeBytes);
            ObjectInputStream in = new ObjectInputStream(bis);
            FieldBasedMerkleTree treeDeserialized = (FieldBasedMerkleTree)in.readObject()
        ) {
            FieldElement expectedRoot = treeDeserialized.root();
            assertNotNull("Must be able to get root from original tree", expectedRoot);
            assertEquals(expectedRoot, treeRoot);

            expectedRoot.freeFieldElement();
            treeRoot.freeFieldElement();

            for (FieldElement leaf: leaves)
                assertTrue(treeDeserialized.isLeafInTree(leaf));
        }
    }

    @Test
    public void testMerklePaths() {
        List<FieldElement> testLeaves = new ArrayList<>();
        FieldBasedMerkleTree mht = FieldBasedMerkleTree.init(6, numLeaves);
        assertNotNull("Merkle Tree initialization must succeed", mht);

        int numLeaves = 64;

        // Append leaves to the tree
        for (int i = 0; i < numLeaves/2; i ++) {
            FieldElement leaf = FieldElement.createRandom(i);
            testLeaves.add(leaf);
            assertTrue("Leaf append must be successfull", mht.append(leaf));
        }
        for (int i = numLeaves/2; i < numLeaves; i ++) {
            FieldElement leaf = FieldElement.createFromLong(0L);
            testLeaves.add(leaf);
        }

        //Finalize the tree and get the root
        assertTrue("Merkle Tree finalization must succeed", mht.finalizeTreeInPlace());
        FieldElement mhtRoot = mht.root();
        assertNotNull("Root must not be NULL", mhtRoot);

        for (int i = 0; i < numLeaves; i ++) {

            // Get/Verify Merkle Path
            FieldBasedMerklePath path = mht.getMerklePath((long)i);
            assertNotNull("Path must not be NULL", path);
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
    public void testAreRightLeavesEmpty() {
        FieldBasedMerkleTree mht = FieldBasedMerkleTree.init(6, numLeaves);
        assertNotNull("Merkle Tree initialization must succeed", mht);

        int numLeaves = 64;

        // Generate random leaves
        for (int i = 0; i < numLeaves; i ++) {
            FieldElement leaf = FieldElement.createRandom(i);
            assertTrue("Leaf append must be successfull", mht.append(leaf));

            FieldBasedMerkleTree mhtCopy = mht.finalizeTree();
            assertNotNull("Merkle Tree finalization must succeed", mhtCopy);

            FieldBasedMerklePath path = mhtCopy.getMerklePath((long)i);
            assertNotNull("Path must not be NULL", path);
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
