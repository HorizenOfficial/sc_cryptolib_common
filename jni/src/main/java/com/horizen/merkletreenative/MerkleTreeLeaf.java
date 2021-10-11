package com.horizen.merkletreenative;

import com.horizen.librustsidechains.FieldElement;

public interface MerkleTreeLeaf extends AutoCloseable {
    public abstract FieldElement getLeafAsFieldElement();
}