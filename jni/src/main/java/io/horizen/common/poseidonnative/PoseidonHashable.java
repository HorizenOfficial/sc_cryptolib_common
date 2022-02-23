
package io.horizen.common.poseidonnative;

import io.horizen.common.librustsidechains.FieldElement;
import io.horizen.common.librustsidechains.FinalizationException;
import io.horizen.common.librustsidechains.NativeParsingException;

/**
 * Interface for classes on which it is possible to compute PoseidonHash
 */
public interface PoseidonHashable {
    /**
     * Compute Poseidon Hash of this instance
     * @return a FieldElement corresponding to the Poseidon Hash of this instance
     */

     /**
      * Compute Poseidon Hash of this instance
      * @return a FieldElement corresponding to the Poseidon Hash of this instance
      * @throws NativeParsingException - if a failure happens while parsing the Object implementing this interface
      * @throws FinalizationException - if hash computation of the object fails
      */
    public FieldElement getHash() throws NativeParsingException, FinalizationException;
}