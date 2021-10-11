package com.horizen.librustsidechains;

import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.ExpectedException;

import static org.junit.Assert.*;

import java.util.Arrays;

public class FieldElementTest {

    @Test
    public void testRandomSerializeDeserialize() throws Exception {

        int samples = 100;

        for( int i = 0; i < samples; i++ ) {
            try
            (
                FieldElement fieldElement1 = FieldElement.createRandom();
                FieldElement fieldElement2 = FieldElement.createRandom()
            )
            {
                byte[] serialized1 = fieldElement1.serializeFieldElement();
                byte[] serialized2 = fieldElement2.serializeFieldElement();

                assertEquals("fieldElement1 size must be - " + FieldElement.FIELD_ELEMENT_LENGTH,
                        FieldElement.FIELD_ELEMENT_LENGTH,
                        serialized1.length);
                assertEquals("fieldElement2 size must be - " + FieldElement.FIELD_ELEMENT_LENGTH,
                        FieldElement.FIELD_ELEMENT_LENGTH,
                        serialized2.length);
                try
                (
                    FieldElement fieldElementDeserialized1 = FieldElement.deserialize(serialized1);
                    FieldElement fieldElementDeserialized2 = FieldElement.deserialize(serialized2)
                )
                {
                    assertNotNull("fieldElement1 deserialization must not fail", fieldElementDeserialized1);
                    assertNotNull("fieldElement2 deserialization must not fail", fieldElementDeserialized2);

                    assertEquals("Field element 1 must be the same.", fieldElement1, fieldElementDeserialized1);
                    assertEquals("Field element 2 must be the same.", fieldElement2, fieldElementDeserialized2);
                }
            }
        }
    }

    @Rule
    public ExpectedException exceptionRule = ExpectedException.none();

    @Test
    public void testDeserializeInvalid() throws Exception {
        exceptionRule.expect(FieldElementException.class);
        exceptionRule.expectMessage("Attempt to deserialize a field element over the modulus");

        byte[] invalidFeBytes = new byte[FieldElement.FIELD_ELEMENT_LENGTH];
        Arrays.fill(invalidFeBytes, (byte)255);

        FieldElement.deserialize(invalidFeBytes);
    }
}