package io.horizen.common.librustsidechains;

/**
 * Holds constant values used Rust side and statically initialized by it
 */
public final class Constants {

    private static int FIELD_ELEMENT_LENGTH;
    private static int SCHNORR_PK_LENGTH;
    private static int SCHNORR_SK_LENGTH;
    private static int SCHNORR_SIGNATURE_LENGTH;
    private static int VRF_PK_LENGTH;
    private static int VRF_SK_LENGTH;
    private static int VRF_PROOF_LENGTH;

    private static native void nativeInitializeAllConstants();

    static {
        Library.load();
        nativeInitializeAllConstants();
    }

    public static int FIELD_ELEMENT_LENGTH() {
        return FIELD_ELEMENT_LENGTH;
    }

    public static int SCHNORR_PK_LENGTH() {
        return SCHNORR_PK_LENGTH;
    }

    public static int SCHNORR_SK_LENGTH() {
        return SCHNORR_SK_LENGTH;
    }

    public static int SCHNORR_SIGNATURE_LENGTH() {
        return SCHNORR_SIGNATURE_LENGTH;
    }

    public static int VRF_PK_LENGTH() {
        return VRF_PK_LENGTH;
    }

    public static int VRF_SK_LENGTH() {
        return VRF_SK_LENGTH;
    }

    public static int VRF_PROOF_LENGTH() {
        return VRF_PROOF_LENGTH;
    }
}