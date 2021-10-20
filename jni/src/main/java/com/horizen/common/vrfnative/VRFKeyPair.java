package com.horizen.common.vrfnative;

import com.horizen.common.librustsidechains.FieldElement;
import com.horizen.common.librustsidechains.Library;


public class VRFKeyPair implements AutoCloseable {
    private VRFSecretKey secretKey;
    private VRFPublicKey publicKey;

    static {
        Library.load();
    }

    public VRFKeyPair(VRFSecretKey secretKey, VRFPublicKey publicKey) {
        this.secretKey = secretKey;
        this.publicKey = publicKey;
    }

    public VRFKeyPair(VRFSecretKey secretKey) {
        this.secretKey = secretKey;
        this.publicKey = secretKey.getPublicKey();
    }

    private static native VRFKeyPair nativeGenerate();

    public static VRFKeyPair generate() {
        return nativeGenerate();
    }

    private native VRFProveResult nativeProve(FieldElement message) throws VRFException;

    public VRFProveResult prove(FieldElement message) throws VRFException {
        return nativeProve(message);
    }

    public VRFSecretKey getSecretKey() {
        return this.secretKey;
    }

    public VRFPublicKey getPublicKey() {
        return this.publicKey;
    }

    @Override
    public void close() throws VRFException {
        this.publicKey.close();
        this.secretKey.close();
    }
}