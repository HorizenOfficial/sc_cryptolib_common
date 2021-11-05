package io.horizen.common.librustsidechains;

import org.junit.Test;

import static org.junit.Assert.*;

import java.util.Arrays;

public class FieldElementTest {

    @Test
    public void testRandomSerializeDeserialize() throws Exception {

        int samples = 100;

        for( int i = 0; i < samples; i++ ) {
            
            // Serialize and deserialize a random field element
            FieldElement fieldElement1 = FieldElement.createRandom();
            FieldElement fieldElement2 = FieldElement.createRandom();
            
            byte[] serialized1 = fieldElement1.serializeFieldElement();
            byte[] serialized2 = fieldElement2.serializeFieldElement();

            assertEquals("fieldElement1 size must be - " + FieldElement.FIELD_ELEMENT_LENGTH,
                    FieldElement.FIELD_ELEMENT_LENGTH,
                    serialized1.length);
            assertEquals("fieldElement2 size must be - " + FieldElement.FIELD_ELEMENT_LENGTH,
                    FieldElement.FIELD_ELEMENT_LENGTH,
                    serialized2.length);

            FieldElement fieldElementDeserialized1 = FieldElement.deserialize(serialized1);
            FieldElement fieldElementDeserialized2 = FieldElement.deserialize(serialized2);

            assertEquals("Field element 1 must be the same.", fieldElement1, fieldElementDeserialized1);
            assertEquals("Field element 2 must be the same.", fieldElement2, fieldElementDeserialized2);

            fieldElement1.close();
            fieldElement2.close();
            fieldElementDeserialized1.close();
            fieldElementDeserialized2.close();

            // Test deserialization succeeds when passing a byte array smaller than FIELD_ELEMENT_LENGTH
            FieldElement smallFieldElement = FieldElement.createFromLong((long)i);
            byte[] smallFieldElementBytes = { (byte)i };
            FieldElement smallFieldElementDeserialized = FieldElement.deserialize(smallFieldElementBytes);
            assertEquals("Must be able to deserialize a FieldElement from a byte array smaller than FIELD_ELEMENT_LENGTH",
                smallFieldElement,
                smallFieldElementDeserialized
            );
        }
    }

    @Test
    public void testFieldElementClone() {
        try {
            FieldElement fe = FieldElement.createRandom();
            FieldElement feClone;

            // Clone fe
            feClone = fe.clone();
            
            // Assert two fe are equal in terms of values
            assertEquals("Original and cloned FieldElement must be equal", fe, feClone);

            // Assert that they are actually two different ones
            assertTrue("Original and cloned FieldElement pointers must be different",
                fe.getFieldElementPointer() != feClone.getFieldElementPointer());

            // Free original field element
            fe.freeFieldElement();

            // Check the clone is still usable by calling a function
            feClone.serializeFieldElement();

            // Free clone
            feClone.freeFieldElement();
        } catch (IllegalStateException ise) {
            assertTrue("Must be able to use cloned FieldElement after the original has been freed", false);
        }
    }

    @Test
    public void testDeserializeInvalid() {
        
        // Attempt to deserialize a FieldElement over the modulus
        try {
            byte[] invalidFeBytes = new byte[FieldElement.FIELD_ELEMENT_LENGTH];
            Arrays.fill(invalidFeBytes, (byte)255);
            FieldElement.deserialize(invalidFeBytes);

            assertFalse("Must be unable to deserialize a FieldElement over the modulus", true);
        } catch (DeserializationException fee) {
            assertTrue(fee.getMessage().contains("Attempt to deserialize a field element over the modulus"));
        }

        // Attempt to deserialize a byte array bigger than FIELD_ELEMENT_LENGTH
        try {
            byte[] invalidFeBytes = new byte[FieldElement.FIELD_ELEMENT_LENGTH + 1];
            Arrays.fill(invalidFeBytes, (byte)0);
            FieldElement.deserialize(invalidFeBytes);

            assertFalse("Must be unable to deserialize a FieldElement from a byte array bigger than FIELD_ELEMENT_LENGTH", true);
        } catch (DeserializationException fee) {
            assertTrue(fee.getMessage().contains("Field element length exceeded"));
        }

    }
}