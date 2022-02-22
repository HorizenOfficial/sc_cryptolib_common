
package io.horizen.common.poseidonnative;

import io.horizen.common.librustsidechains.FieldElement;

/**
 * Interface for classes on which it is possible to compute PoseidonHash
 */
public interface PoseidonHashable {
    /**
     * Compute Poseidon Hash of this instance
     * @return a FieldElement corresponding to the Poseidon Hash of this instance
     */
    public FieldElement getHash();
}