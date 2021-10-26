package com.horizen.common.vrfnative;

import com.horizen.common.librustsidechains.FieldElement;
import com.horizen.common.librustsidechains.Library;

public class VRFProveResult implements AutoCloseable {
    private VRFProof vrfProof;
    private FieldElement vrfOutput;

    static {
        Library.load();
    }

    public VRFProveResult(VRFProof vrfProof, FieldElement vrfOutput) {
        this.vrfProof = vrfProof;
        this.vrfOutput = vrfOutput;
    }

    public VRFProof getVRFProof() {
        return this.vrfProof;
    }

    public FieldElement getVRFOutput() {
        return this.vrfOutput;
    }

    @Override
    public void close() {
        this.vrfProof.close();
        this.vrfOutput.close();
    }
}