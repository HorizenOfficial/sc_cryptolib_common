package com.horizen.merkletreenative;

import com.horizen.librustsidechains.FieldElement;

public class FieldElementLeaf implements MerkleTreeLeaf {
    public FieldElement leaf;

    public FieldElementLeaf(FieldElement leaf) {
        this.leaf = leaf;
    }

    @Override
    public FieldElement getLeafAsFieldElement() {
        return this.leaf;
    }

    @Override
    public void close() throws Exception {
        this.leaf.close();
    }
}
