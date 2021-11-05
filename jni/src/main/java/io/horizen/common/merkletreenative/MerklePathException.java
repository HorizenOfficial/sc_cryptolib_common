package io.horizen.common.merkletreenative;

public class MerklePathException extends Exception {
    public MerklePathException(String message) {
        super(message);
    }

    public MerklePathException(String message, Throwable cause) {
        super(message, cause);
    }
}
