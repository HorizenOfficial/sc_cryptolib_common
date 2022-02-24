package io.horizen.common.schnorrnative;

import io.horizen.common.librustsidechains.*;

public class SchnorrSecretKey implements AutoCloseable
{
    private long secretKeyPointer;

    static {
        Library.load();
    }

    private SchnorrSecretKey(long secretKeyPointer) {
        if (secretKeyPointer == 0)
            throw new IllegalArgumentException("Secret key pointer must be not null.");
        this.secretKeyPointer = secretKeyPointer;
    }

    private static native SchnorrSecretKey nativeDeserializeSecretKey(byte[] secretKeyBytes) throws DeserializationException;

    public static SchnorrSecretKey deserialize(byte[] secretKeyBytes) throws DeserializationException {
        if (secretKeyBytes.length != Constants.SCHNORR_SK_LENGTH())
            throw new IllegalArgumentException(String.format("Incorrect secret key length, %d expected, %d found", Constants.SCHNORR_SK_LENGTH(), secretKeyBytes.length));

        return nativeDeserializeSecretKey(secretKeyBytes);
    }

    private native byte[] nativeSerializeSecretKey();


    public byte[] serializeSecretKey() {
        if (secretKeyPointer == 0)
            throw new IllegalStateException("Secret key was freed.");

        return nativeSerializeSecretKey();
    }

    private native void nativeFreeSecretKey();

    public void freeSecretKey() {
        if (secretKeyPointer != 0) {
            nativeFreeSecretKey();
            secretKeyPointer = 0;
        }
    }

    private native SchnorrPublicKey nativeGetPublicKey();

    public SchnorrPublicKey getPublicKey() {
        if (secretKeyPointer == 0)
            throw new IllegalStateException("Secret key was freed.");

        return nativeGetPublicKey();
    }

    @Override
    public void close() {
        freeSecretKey();
    }
}
