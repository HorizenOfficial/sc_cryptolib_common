package com.horizen.merkletreenative;

import com.horizen.librustsidechains.FieldElement;

public interface MerkleTreeLeaf extends AutoCloseable {
    /**
     * Convert this instance to a FieldElement.
     * It's caller responsibility to free the FieldElement after using it. 
     * @return FieldElement representation of this instance
     */
    public abstract FieldElement getLeafAsFieldElement();
}