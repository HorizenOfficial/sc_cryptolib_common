package com.horizen.schnorrnative;

import com.horizen.librustsidechains.Library;

public class SchnorrSignature implements AutoCloseable
{
  public static final int SIGNATURE_LENGTH;

  private long signaturePointer;

  private static native int nativeGetSignatureSize();

  static {
    Library.load();
    SIGNATURE_LENGTH = nativeGetSignatureSize();
  }

  private SchnorrSignature(long signaturePointer) {
    if (signaturePointer == 0)
      throw new IllegalArgumentException("Signature pointer must be not null.");
    this.signaturePointer = signaturePointer;
  }

  public SchnorrSignature() {
    this.signaturePointer = 0;
  }

  private native byte[] nativeSerializeSignature();

  private static native SchnorrSignature nativeDeserializeSignature(byte[] signatureBytes, boolean checkSignature) throws SchnorrSignatureException;

  private native void nativefreeSignature();

  public static SchnorrSignature deserialize(byte[] signatureBytes, boolean checkSignature) throws SchnorrSignatureException {
    if (signatureBytes.length != SIGNATURE_LENGTH)
      throw new IllegalArgumentException(String.format("Incorrect signature length, %d expected, %d found", SIGNATURE_LENGTH, signatureBytes.length));

    return nativeDeserializeSignature(signatureBytes, checkSignature);
  }

  public static SchnorrSignature deserialize(byte[] signatureBytes) throws SchnorrSignatureException {
    return deserialize(signatureBytes, true);
  }


  public byte[] serializeSignature() {
    return nativeSerializeSignature();
  }

  private native boolean nativeIsValidSignature(); // jni call to Rust impl

  public boolean isValidSignature() {
    if (signaturePointer == 0)
      throw new IllegalArgumentException("Schnorr signature was freed.");

    return nativeIsValidSignature();
  }

  public void freeSignature() {
    if (signaturePointer != 0) {
      nativefreeSignature();
      signaturePointer = 0;
    }
  }

  @Override
  public void close() throws SchnorrSignatureException {
    freeSignature();
  }
}

