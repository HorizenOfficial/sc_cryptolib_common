package com.horizen.common.merkletreenative;

public class MerkleTreeLeafException extends Exception {
    public MerkleTreeLeafException(String message) {
        super(message);
    }

    public MerkleTreeLeafException(String message, Throwable cause) {
        super(message, cause);
    }
}