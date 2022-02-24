package io.horizen.common.librustsidechains;

/**
 * An error occured while performing a certain operation native side
 */
public class NativeOperationException extends Exception {

    public NativeOperationException(String message) {
        super(message);
    }

    public NativeOperationException(String message, Throwable cause) {
        super(message, cause);
    }
    
}
