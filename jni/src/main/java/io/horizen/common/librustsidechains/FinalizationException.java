package io.horizen.common.librustsidechains;

/**
 * Failed to finalize a given routine/data structure construction
 */
public class FinalizationException extends Exception {

    public FinalizationException(String message) {
        super(message);
    }

    public FinalizationException(String message, Throwable cause) {
        super(message, cause);
    }
    
}
