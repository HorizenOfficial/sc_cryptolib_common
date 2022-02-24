package io.horizen.common.librustsidechains;

/**
 * Failed to convert Object to Field Element(s)
 */
public class FieldElementConversionException extends Exception {

    public FieldElementConversionException(String message) {
        super(message);
    }

    public FieldElementConversionException(String message, Throwable cause) {
        super(message, cause);
    }
    
}
