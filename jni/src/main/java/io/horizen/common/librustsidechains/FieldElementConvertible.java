package io.horizen.common.librustsidechains;

/**
 * Utility interface providing the implementer with the capability to convert
 * the self instance into a FieldElement.
 */
public interface FieldElementConvertible {
    /**
     * Convert this instance into a FieldElement
     * @return the FieldElement conversion result starting from this instance
     * @throws FieldElementConversionException if the conversion fails
     */
    FieldElement toFieldElement() throws FieldElementConversionException;
}