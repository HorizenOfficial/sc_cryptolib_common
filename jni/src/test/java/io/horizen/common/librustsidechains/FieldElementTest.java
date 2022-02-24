package io.horizen.common.librustsidechains;

import org.junit.Test;

import static org.junit.Assert.*;

import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Random;

public class FieldElementTest {

    static int samples = 100;

    @Test
    public void testRandomSerializeDeserialize() throws Exception {

        for( int i = 0; i < samples; i++ ) {
            
            // Serialize and deserialize a random field element
            FieldElement fieldElement1 = FieldElement.createRandom();
            FieldElement fieldElement2 = FieldElement.createRandom();
            
            byte[] serialized1 = fieldElement1.serializeFieldElement();
            byte[] serialized2 = fieldElement2.serializeFieldElement();

            assertEquals("fieldElement1 size must be - " + Constants.FIELD_ELEMENT_LENGTH(),
                    Constants.FIELD_ELEMENT_LENGTH(),
                    serialized1.length);
            assertEquals("fieldElement2 size must be - " + Constants.FIELD_ELEMENT_LENGTH(),
                    Constants.FIELD_ELEMENT_LENGTH(),
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
    public void testRandomSerializeManyDeserializeMany() throws Exception {

        for( int i = 1; i <= samples; i++ ) {
            
            // Generate random FieldElements and the corresponding bytes
            List<FieldElement> randomFes = new ArrayList<>();
            List<Byte> randomFesBytes = new ArrayList<>();

            int numFeToGenerate = new Random().nextInt(samples);
            for(int j = 0; j < numFeToGenerate; j++) {
                FieldElement randomFe = FieldElement.createRandom();
                randomFes.add(randomFe);

                byte[] serializedFe = randomFe.serializeFieldElement();
                assertEquals("serializedFe size must be - " + Constants.FIELD_ELEMENT_LENGTH(),
                    Constants.FIELD_ELEMENT_LENGTH(),
                    serializedFe.length
                );

                for (byte b: serializedFe)
                    randomFesBytes.add((Byte)b);
            }

            // Deserialize FieldElements out of randomFesBytes and compare them with the original List
            Byte[] randomFesBytesArray = randomFesBytes.toArray(new Byte[0]);
            byte[] randomFesPrimitiveByteArray = new byte[randomFesBytesArray.length];
            for (int j = 0; j < randomFesBytesArray.length; j++)
                randomFesPrimitiveByteArray[j] = (byte)randomFesBytesArray[j];
            
            List<FieldElement> deserializedRandomFes =
                FieldElement.deserializeMany(randomFesPrimitiveByteArray, true);

            assertTrue("Original and deserialized FieldElements must be the same", randomFes.equals(deserializedRandomFes));
    
            // Should be able also to deserialize in a non strict way
            List<FieldElement> deserializedNonStrictFes = FieldElement.deserializeMany(randomFesPrimitiveByteArray, false);

            // Free memory
            for (FieldElement fe: randomFes)
                fe.close();
            for (FieldElement fe: deserializedRandomFes)
                fe.close();
            for (FieldElement fe: deserializedNonStrictFes)
                fe.close();
        }
    }

    @Test
    public void testNonStrictDeserializeManyFromRandomByteArray() throws Exception {
        int maxByteArraySize = 1024;

        for(int i = 0; i < samples; i++) {
            Random r = new Random();

            // Generate random byte array of random size
            int size = r.nextInt(maxByteArraySize);
            byte[] randomBytes = new byte[size];
            r.nextBytes(randomBytes);

            // Deserialize FieldElements from it
            List<FieldElement> deserializedRandomFes = FieldElement.deserializeMany(randomBytes);

            // Assert deserialization of expected number of FieldElements
            assertEquals(deserializedRandomFes.size(), (size + Constants.FIELD_ELEMENT_LENGTH() - 2)/(Constants.FIELD_ELEMENT_LENGTH() - 1));
        }
    }

    @Test
    public void testRandomDeserializeFromString() throws Exception {
        for(int i = 1; i < Constants.FIELD_ELEMENT_LENGTH(); i++) {
            // Generate random UTF-8 String
            byte[] stringBytes = new byte[i];
            new Random().nextBytes(stringBytes);
            String generatedString = new String(stringBytes, StandardCharsets.US_ASCII);
            assertEquals(generatedString.length(), i);

            // FieldElement deserialization must be successfull
            FieldElement fe = FieldElement.deserializeFromString(generatedString);
            fe.close();
        }
    }

    @Test
    public void testSplitPositive() throws Exception {
        // Positive case
        for(int i = 1; i < Constants.FIELD_ELEMENT_LENGTH(); i++) {
            // Generate random FieldElement and split it into two FieldElements at index i
            FieldElement feToBeSplit = FieldElement.createRandom(); 
            List<FieldElement> splitFes = feToBeSplit.splitAt(i);

            // Assert that the resulting FieldElement are 2
            assertEquals(splitFes.size(), 2);

            // Rejoin the two FieldElements at the same index and check equality with the original one
            FieldElement restoredFe = FieldElement.joinAt(splitFes.get(0), i, splitFes.get(1), Constants.FIELD_ELEMENT_LENGTH() - i, true);
            assertEquals("Must be able to reconstruct the original FieldElement split ad index:" + i, feToBeSplit, restoredFe);

            // Free memory
            feToBeSplit.close();
            restoredFe.close();
            splitFes.get(0).close();
            splitFes.get(1).close();
        }
    }

    @Test
    public void testSplitNegative() throws Exception {
        // Split then rejoin at wrong index and assert we are not able to reconstruct the original FieldElement
        for(int i = 1; i < Constants.FIELD_ELEMENT_LENGTH() - 1; i++) {
            // Generate random FieldElement and split it into two FieldElements at index i
            FieldElement feToBeSplit = FieldElement.createRandom(); 
            List<FieldElement> splitFes = feToBeSplit.splitAt(i);

            // Assert that the resulting FieldElement are 2
            assertEquals(splitFes.size(), 2);

            // Rejoin the two FieldElements at an index shifted by one with respect to the original and
            // assert reconstruction of a different FieldElement
            try {
                FieldElement restoredFe = FieldElement.joinAt(splitFes.get(0), i + 1, splitFes.get(1), Constants.FIELD_ELEMENT_LENGTH() - i - 1);
                assertNotEquals("Mustn't be able to reconstruct the original FieldElement split ad index:" + i, feToBeSplit, restoredFe);
                restoredFe.close();
            } catch (DeserializationException dex) {
                // Since we combine the two FieldElements incorrectly, might also happen that we generate an invalid (i.e. over the modulus) FieldElement
            }
            
            // Free memory
            feToBeSplit.close();
            splitFes.get(0).close();
            splitFes.get(1).close();
        }
    }

    @Test
    public void testSplitExceptions() throws Exception {
        FieldElement fe1 = FieldElement.createRandom();
        FieldElement fe2 = FieldElement.createRandom();

        // Try to split at 0
        try {
            fe1.splitAt(0);
            assertTrue(false); // Mustn't be able to reach this point
        } catch (IndexOutOfBoundsException ex) {
            assertTrue(ex.getMessage().contains("Invalid split index"));
        };

        // Try to split at FIELD_ELEMENT_LENGTH
        try {
            fe1.splitAt(Constants.FIELD_ELEMENT_LENGTH());
            assertTrue(false); // Mustn't be able to reach this point
        } catch (IndexOutOfBoundsException ex) {
            assertTrue(ex.getMessage().contains("Invalid split index"));
        };

        // Try to join by forming a non valid FieldElement
        try {
            FieldElement.joinAt(fe1, 20, fe2, 20);
            assertTrue(false); // Mustn't be able to reach this point
        } catch (IllegalArgumentException ex) {
            assertTrue(
                ex.getMessage().contains(
                    "Invalid values for index1 + index2: the resulting array would overflow FIELD_ELEMENT_LENGTH"
                )
            );
        }

        // Try to join not passing the zero check
        try {
            FieldElement.joinAt(fe1, 1, fe2, 1, true);
            assertTrue(false); // Mustn't be able to reach this point
        } catch (IllegalArgumentException ex) {
            // Zero check failed on bytes of fe1
            assertTrue(
                ex.getMessage().contains(
                    "Zero check failed on bytes of fe1"
                )
            );
        }

        FieldElement zero = FieldElement.createFromLong(0L);
        try {
            FieldElement.joinAt(zero, 1, fe2, 1, true);
            assertTrue(false); // Mustn't be able to reach this point
        } catch (IllegalArgumentException ex) {
            // Zero check failed on bytes of fe2
            assertTrue(
                ex.getMessage().contains(
                    "Zero check failed on bytes of fe2"
                )
            );
            zero.close();
        }

        // Free data
        fe1.close();
        fe2.close();
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
            byte[] invalidFeBytes = new byte[Constants.FIELD_ELEMENT_LENGTH()];
            Arrays.fill(invalidFeBytes, (byte)255);
            FieldElement.deserialize(invalidFeBytes);

            assertFalse("Must be unable to deserialize a FieldElement over the modulus", true);
        } catch (DeserializationException fee) {
            assertTrue(fee.getMessage().contains("Attempt to deserialize a field element over the modulus"));
        }

        // Attempt to deserialize a byte array bigger than FIELD_ELEMENT_LENGTH
        try {
            byte[] invalidFeBytes = new byte[Constants.FIELD_ELEMENT_LENGTH() + 1];
            Arrays.fill(invalidFeBytes, (byte)0);
            FieldElement.deserialize(invalidFeBytes);

            assertFalse("Must be unable to deserialize a FieldElement from a byte array bigger than FIELD_ELEMENT_LENGTH", true);
        } catch (DeserializationException fee) {
            assertTrue(fee.getMessage().contains("Field element length exceeded"));
        }
    }
}