package io.horizen.common.librustsidechains;

/**
 * Failed to bootstrap a given routine/data structure
 */
public class InitializationException extends Exception {

    public InitializationException(String message) {
        super(message);
    }

    public InitializationException(String message, Throwable cause) {
        super(message, cause);
    }
    
}
