package com.horizen.common.librustsidechains;

public class FinalizationException extends Exception {

    public FinalizationException(String message) {
        super(message);
    }

    public FinalizationException(String message, Throwable cause) {
        super(message, cause);
    }
    
}
