package com.horizen.common.schnorrnative;

public class SchnorrSignatureException extends Exception {
    public SchnorrSignatureException(String message) {
        super(message);
    }

    public SchnorrSignatureException(String message, Throwable cause) {
        super(message, cause);
    }
}
