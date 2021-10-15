package com.horizen.schnorrnative;

import com.horizen.librustsidechains.*;

public class SchnorrPublicKey implements AutoCloseable
{

  public static final int PUBLIC_KEY_LENGTH;

  private long publicKeyPointer;

  private static native int nativeGetPublicKeySize();

  static {
    Library.load();
    PUBLIC_KEY_LENGTH = nativeGetPublicKeySize();
  }

  private SchnorrPublicKey(long publicKeyPointer) {
    if (publicKeyPointer == 0)
      throw new IllegalArgumentException("Public key pointer must be not null.");
    this.publicKeyPointer = publicKeyPointer;
  }

  private static native SchnorrPublicKey nativeDeserializePublicKey(byte[] publicKeyBytes, boolean checkPublicKey, boolean compressed) throws SchnorrSignatureException;

  public static SchnorrPublicKey deserialize(byte[] publicKeyBytes, boolean checkPublicKey, boolean compressed) throws SchnorrSignatureException {
    if (publicKeyBytes.length != PUBLIC_KEY_LENGTH)
      throw new IllegalArgumentException(String.format("Incorrect public key length, %d expected, %d found", PUBLIC_KEY_LENGTH, publicKeyBytes.length));

    return nativeDeserializePublicKey(publicKeyBytes, checkPublicKey, compressed);
  }

  public static SchnorrPublicKey deserialize(byte[] publicKeyBytes, boolean checkPublicKey) throws SchnorrSignatureException {
    return deserialize(publicKeyBytes, checkPublicKey, true);
  }

  public static SchnorrPublicKey deserialize(byte[] publicKeyBytes) throws SchnorrSignatureException {
    return deserialize(publicKeyBytes, true, true);
  }

  private native byte[] nativeSerializePublicKey(boolean compressed);


  public byte[] serializePublicKey(boolean compressed) {
    if (publicKeyPointer == 0)
      throw new IllegalStateException("Public key was freed.");

    return nativeSerializePublicKey(compressed);
  }

  public byte[] serializePublicKey() {
    return serializePublicKey(true);
  }

  private native void nativeFreePublicKey();

  public void freePublicKey() {
    if (publicKeyPointer != 0) {
      nativeFreePublicKey();
      publicKeyPointer = 0;
    }
  }

  private native boolean nativeVerifySignature(SchnorrSignature signature, FieldElement message) throws SchnorrSignatureException; // jni call to Rust impl

  private native boolean nativeVerifyKey(); // jni call to Rust impl

  public boolean verifySignature(SchnorrSignature signature, FieldElement message) throws SchnorrSignatureException {
    if (publicKeyPointer == 0)
      throw new IllegalStateException("Public key was freed.");

    return nativeVerifySignature(signature, message);
  }

  public boolean verifyKey() {
    if (publicKeyPointer == 0)
      throw new IllegalStateException("Public key was freed.");

    return nativeVerifyKey();
  }

  @Override
  public void close() throws SchnorrSignatureException {
    freePublicKey();
  }
}

