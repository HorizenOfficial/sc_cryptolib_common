package io.horizen.common.librustsidechains;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Random;
import java.lang.Cloneable;
import java.nio.charset.StandardCharsets;

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
     * Deserialize a FieldElement from "fieldElementBytes". Bytes are assumed to be in little-endian (as per output of serializeFieldElement)
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

    /**
     * Deserialize as many FieldElements as possible from bytes.
     * @param bytes - the bytes to be deserialized into FieldElements
     * @param strict - if set to true, the function tries to read a FieldElement out of a
     *                 chunk of size exactly FIELD_ELEMENT_LENGTH, thus assuming each chunk
     *                 of this size would contain a valid FieldElement; otherwise this
     *                 assumption is dropped and the chunk size will be set to FIELD_ELEMENT_LENGTH - 1,
     *                 from which it is always possible to deserialize a valid FieldElement (typical usage
     *                 would be this one)
     * @return the FieldElements deserialized from bytes
     * @throws DeserializationException - If any error occures during deserialization
     */
    public static List<FieldElement> deserializeMany(byte[] bytes, boolean strict) throws DeserializationException {
        List<FieldElement> deserializedFes = new ArrayList<>();
        int chunkSize = strict ? FIELD_ELEMENT_LENGTH: FIELD_ELEMENT_LENGTH - 1;
        int start = 0;
        while (start < bytes.length) {
            int end = Math.min(bytes.length, start + chunkSize);
            deserializedFes.add(FieldElement.deserialize(Arrays.copyOfRange(bytes, start, end)));
            start += chunkSize;
        }
        return deserializedFes;
    }

    /**
     * Deserialize as many FieldElements as possible from bytes in a safe way, i.e. reading a FieldElement
     * out of each chunk of size FIELD_ELEMENT_LENGTH - 1 (from which it is always possible to read a valid
     * FieldElement).
     * @param bytes - the bytes to be deserialized into FieldElements
     * @return the FieldElements deserialized from bytes
     * @throws DeserializationException - If any error occures during deserialization
     */
    public static List<FieldElement> deserializeMany(byte[] bytes) throws DeserializationException {
        return FieldElement.deserializeMany(bytes, false);
    }

    /**
     * Deserialize a FieldElement from a String. The same rule as per deserialize(byte[]) method apply.
     * @param s - the string to be deserialized into a FieldElement
     * @return the FieldElement deserialized from the input string
     * @throws DeserializationException 
     */
    public static FieldElement deserializeFromString(String s) throws DeserializationException {
        byte[] stringBytes = s.getBytes(StandardCharsets.UTF_8);
        return FieldElement.deserialize(stringBytes);
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
