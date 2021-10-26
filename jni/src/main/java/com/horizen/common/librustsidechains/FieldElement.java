package com.horizen.common.librustsidechains;

import java.util.Random;
import java.lang.Cloneable;

public class FieldElement implements AutoCloseable, Cloneable {

    public static final int FIELD_ELEMENT_LENGTH;

    private long fieldElementPointer;

    private static native int nativeGetFieldElementSize();

    static {
        Library.load();
        FIELD_ELEMENT_LENGTH = nativeGetFieldElementSize();
    }

    // Declared protected for testing purposes
    protected FieldElement(long fieldElementPointer) {
        this.fieldElementPointer = fieldElementPointer;
    }

    protected long getFieldElementPointer() {
        return this.fieldElementPointer;
    }

    private static native FieldElement nativeCreateFromLong(long value);

    public static FieldElement createFromLong(long value) {
        return nativeCreateFromLong(value);
    }

    private static native FieldElement nativeCreateSecureRandom();

    public static FieldElement createSecureRandom() {
        return nativeCreateSecureRandom();
    }

    private static native FieldElement nativeCreateRandom(long seed);

    /*  NOTE: This function relies on a non-cryptographically safe RNG, therefore it
     *  must be used ONLY for testing purposes
     */
    public static FieldElement createRandom(long seed) { return nativeCreateRandom(seed); }

    public static FieldElement createRandom() {
        long seed = new Random().nextLong();
        return nativeCreateRandom(seed);
    }

    // Declared protected for testing purposes
    protected native byte[] nativeSerializeFieldElement();


    public byte[] serializeFieldElement() {
        if (fieldElementPointer == 0)
            throw new IllegalStateException("Field element was freed.");

        return nativeSerializeFieldElement();
    }

    private static native FieldElement nativeDeserializeFieldElement(byte[] fieldElementBytes) throws DeserializationException;

    /**
     * Deserialize a FieldElement from "fieldElementBytes"
     * @param fieldElementBytes bytes of the FieldElement to be deserialized
     * @return The deserialized FieldElement
     * @throws DeserializationException If fieldElementBytes.len() > FIELD_ELEMENT_LENGTH or if the bytes represent an invalid FieldElement
     */
    public static FieldElement deserialize(byte[] fieldElementBytes) throws DeserializationException {
        if (fieldElementBytes.length > FIELD_ELEMENT_LENGTH)
            throw new DeserializationException(String.format("Field element length exceeded: limit %d , %d found",
                    FIELD_ELEMENT_LENGTH, fieldElementBytes.length));

        return nativeDeserializeFieldElement(fieldElementBytes);
    }

    private native FieldElement nativeClone();

    @Override
    public FieldElement clone() {
        if (fieldElementPointer == 0)
            throw new IllegalStateException("Field element was freed.");
        FieldElement clone = nativeClone();
        return clone;
    }

    // Declared protected for testing purposes
    protected native void nativePrintFieldElementBytes();

    public void printFieldElementBytes() {
        if (fieldElementPointer == 0)
            throw new IllegalStateException("Field element was freed.");
        nativePrintFieldElementBytes();
    }

    private native void nativeFreeFieldElement();

    public void freeFieldElement() {
        if (fieldElementPointer != 0) {
            nativeFreeFieldElement();
            fieldElementPointer = 0;
        }
    }

    private native boolean nativeEquals(FieldElement fe);

    @Override
    public boolean equals(Object o) {

        if (o == this) {
            return true;
        }

        if (!(o instanceof FieldElement)) {
            return false;
        }

        return nativeEquals((FieldElement) o);
    }

    @Override
    public void close() {
        freeFieldElement();
    }
}
