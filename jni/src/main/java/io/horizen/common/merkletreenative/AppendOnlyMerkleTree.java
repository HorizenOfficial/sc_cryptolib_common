package io.horizen.common.merkletreenative;

import io.horizen.common.librustsidechains.FieldElement;

/**
 * Interface for a MerkleTree with append-only logic
 */
public interface AppendOnlyMerkleTree extends MerkleTreeCommon {
    
    /**
     * Append a new leaf `input` to this instance.
     * @param input data to append to the tree
     * @throws MerkleTreeException if unable to append input leaf to this tree
     */
    void append(FieldElement input) throws MerkleTreeException;
}
