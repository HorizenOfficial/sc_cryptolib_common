package io.horizen.common.librustsidechains;

/**
 * Failed to deserialize object from raw data
 */
public class DeserializationException extends Exception {

    public DeserializationException(String message) {
        super(message);
    }

    public DeserializationException(String message, Throwable cause) {
        super(message, cause);
    }
    
}
