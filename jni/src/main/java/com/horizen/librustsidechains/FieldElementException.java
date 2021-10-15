package com.horizen.librustsidechains;

public class FieldElementException extends Exception {
    public FieldElementException(String message) {
        super(message);
    }

    public FieldElementException(String message, Throwable cause) {
        super(message, cause);
    }
}