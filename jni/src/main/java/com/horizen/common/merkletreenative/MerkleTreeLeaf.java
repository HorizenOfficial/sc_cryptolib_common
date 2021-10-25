package com.horizen.common.merkletreenative;

import com.horizen.common.librustsidechains.FieldElement;

public interface MerkleTreeLeaf extends AutoCloseable {
    /**
     * Convert this instance to a FieldElement.
     * It's caller responsibility to free the FieldElement after using it. 
     * @return FieldElement representation of this instance
     * @throws MerkleTreeLeafException if some error occurs during conversion between this and FieldElement
     */
    FieldElement getLeafAsFieldElement() throws MerkleTreeLeafException;
}