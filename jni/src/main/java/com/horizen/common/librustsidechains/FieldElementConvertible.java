package com.horizen.common.librustsidechains;

/**
 * Utility interface providing the implementer with the capability to convert
 * the self instance into a FieldElement.
 */
public interface FieldElementConvertible {
    /**
     * Convert this instance into a FieldElement
     * @return the FieldElement conversion result starting from this instance
     * @throws ConversionException if the conversion fails
     */
    FieldElement toFieldElement() throws ConversionException;
}