package io.horizen.common.merkletreenative;

import io.horizen.common.librustsidechains.FieldElement;

import org.junit.Test;
import org.junit.Before;
import org.junit.After;

import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;

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

        //Get BaseSparseMerkleTree
        BaseSparseMerkleTree smtLazy = BaseSparseMerkleTree.init(height);

        //Add leaves to BaseSparseMerkleTree
        Map<Long, FieldElement> positionLeaves = new HashMap<>();
        for(int i = 0; i < numLeaves; i++) {
            assertTrue("Position must be empty", smtLazy.isPositionEmpty(positions[i]));
            positionLeaves.put(positions[i], leaves.get(i));
        }
        smtLazy.addLeaves(positionLeaves);

        //Remove leaves from BaseSparseMerkleTree
        Set<Long> leavesToRemove = new HashSet<>(Arrays.asList(458L, 532L));
        smtLazy.removeLeaves(leavesToRemove);

        //Compute root and assert equality with the expected one
        smtLazy.finalizeTreeInPlace();
        FieldElement smtLazyRoot = smtLazy.root();
        assertEquals("BaseSparseMerkleTree root is not as expected", smtLazyRoot, expectedRoot);

        //Free memory
        smtLazy.freeMerkleTree();
        smtLazyRoot.freeFieldElement();

        //Get BaseAppendOnlyMerkleTree
        BaseAppendOnlyMerkleTree mht = BaseAppendOnlyMerkleTree.init(height, numLeaves);

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
            mht.append(leaf);

        //Finalize the tree
        mht.finalizeTreeInPlace();

        //Compute root and assert equality with the expected one
        FieldElement mhtRoot = mht.root();
        assertEquals("BaseAppendOnlyMerkleTree root is not as expected", mhtRoot, expectedRoot);

        //It is the same with finalizeTree()
        BaseAppendOnlyMerkleTree mhtCopy = mht.finalizeTree();

        FieldElement mhtRootCopy = mhtCopy.root();
        assertEquals("BaseAppendOnlyMerkleTree copy root is not as expected", mhtRootCopy, expectedRoot);

        //Free memory
        zero.freeFieldElement();
        mht.freeMerkleTree();
        mhtCopy.freeMerkleTree();
        mhtRoot.freeFieldElement();
        mhtRootCopy.freeFieldElement();
    }

    @Test
    public void testMerkleTreeReset() throws Exception {
        //Get BaseAppendOnlyMerkleTree
        BaseAppendOnlyMerkleTree mht = BaseAppendOnlyMerkleTree.init(height, numLeaves);

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

        for (int i = 0; i < 100; i++) {
            //Append all the leaves to mht
            for (FieldElement leaf: mhtLeaves)
                mht.append(leaf);

            //Finalize the tree
            mht.finalizeTreeInPlace();

            //Compute root and assert equality with the expected one
            FieldElement mhtRoot = mht.root();
            assertEquals("BaseAppendOnlyMerkleTree root is not as expected", mhtRoot, expectedRoot);

            mhtRoot.close();
            mht.reset();
        }
        mht.close();
    }

    @Test
    public void testMerklePaths() throws Exception {
        List<FieldElement> testLeaves = new ArrayList<>();
        Map<Long, FieldElement> testPositionLeaves = new HashMap<>();
        BaseAppendOnlyMerkleTree mht = BaseAppendOnlyMerkleTree.init(6, numLeaves);
        BaseSparseMerkleTree smt = BaseSparseMerkleTree.init(6);

        int numLeaves = 64;

        // Append leaves to mht
        for (int i = 0; i < numLeaves/2; i ++) {
            FieldElement leaf = FieldElement.createRandom(i);
            testLeaves.add(leaf);
            testPositionLeaves.put((long)i, leaf);
            mht.append(leaf);
        }
        for (int i = numLeaves/2; i < numLeaves; i ++) {
            FieldElement leaf = FieldElement.createFromLong(0L);
            testLeaves.add(leaf);
        }

        // Append leaves to SMT too
        smt.addLeaves(testPositionLeaves);
        smt.finalizeTreeInPlace();
        FieldElement smtRoot = smt.root();

        //Finalize the tree and get the root
        mht.finalizeTreeInPlace();
        FieldElement mhtRoot = mht.root();
        assertEquals("Sparse and Append Merkle Tree roots must be the same", smtRoot, mhtRoot);

        for (int i = 0; i < numLeaves; i ++) {

            // Get/Verify Merkle Path
            FieldBasedMerklePath path = mht.getMerklePath((long)i);
            FieldBasedMerklePath smtPath = smt.getMerklePath((long)i);

            assertEquals("Sparse and Append Merkle Tree paths must be the same", path, smtPath);

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
        smt.freeMerkleTree();
        mht.freeMerkleTree();
        mhtRoot.freeFieldElement();
        for (FieldElement leaf: testLeaves)
            leaf.freeFieldElement();
    }

    @Test
    public void testAreRightLeavesEmpty() throws Exception {
        BaseAppendOnlyMerkleTree mht = BaseAppendOnlyMerkleTree.init(6, numLeaves);

        int numLeaves = 64;

        // Generate random leaves
        for (int i = 0; i < numLeaves; i ++) {
            FieldElement leaf = FieldElement.createRandom(i);
            mht.append(leaf);

            BaseAppendOnlyMerkleTree mhtCopy = mht.finalizeTree();

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