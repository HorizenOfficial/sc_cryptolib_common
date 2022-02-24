package io.horizen.common.librustsidechains;

/**
 * Failed to convert Java object data to its corresponding native representation
 */
public class NativeParsingException extends Exception {

    public NativeParsingException(String message) {
        super(message);
    }

    public NativeParsingException(String message, Throwable cause) {
        super(message, cause);
    }
    
}
