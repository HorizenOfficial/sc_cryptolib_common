package com.horizen.common.librustsidechains;

public class DeserializationException extends Exception {

    public DeserializationException(String message) {
        super(message);
    }

    public DeserializationException(String message, Throwable cause) {
        super(message, cause);
    }
    
}
