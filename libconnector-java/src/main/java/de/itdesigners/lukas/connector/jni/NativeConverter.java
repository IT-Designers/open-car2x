package de.itdesigners.lukas.connector.jni;

import javax.annotation.Nonnull;

import de.itdesigners.lukas.connector.Converter;
import de.itdesigners.lukas.connector.Format;
import de.itdesigners.lukas.connector.MessageId;

class NativeConverter implements Converter {
    public static final int DESTINATION_BUFFER_MAX_LEN = 80_000;

    @Nonnull
    public byte[] convert(
            @Nonnull MessageId mid,
            @Nonnull Format sourceFormat,
            @Nonnull byte[] source,
            @Nonnull Format destinationFormat) throws NativeResultException {
        byte[] buffer = new byte[DESTINATION_BUFFER_MAX_LEN];
        int    length = (int) NativeResultException.fromErrorCodeAndValue(nativeConvert(
                mid.getNumeric(),
                sourceFormat.getNumeric(),
                source,
                destinationFormat.getNumeric(),
                buffer
        ));
        byte[] result = new byte[length];
        System.arraycopy(buffer, 0, result, 0, length);
        return result;
    }

    @Nonnull
    protected static byte[] convertFromMessageRef(long ptr, @Nonnull Format sourceFormat, @Nonnull Format destinationFormat) throws NativeResultException {
        byte[] buffer = new byte[DESTINATION_BUFFER_MAX_LEN];
        int    length = (int) NativeResultException.fromErrorCodeAndValue(nativeConvertFromMessageRef(
                ptr,
                sourceFormat.getNumeric(),
                destinationFormat.getNumeric(),
                buffer
        ));
        byte[] result = new byte[length];
        System.arraycopy(buffer, 0, result, 0, length);;
        return result;
    }

    private static native long[] nativeConvert(
            long mid,
            int sourceFormat,
            byte[] source,
            int destinationFormat,
            byte[] destinationBuffer
    );

    private static native long[] nativeConvertFromMessageRef(
            long messageRefPtr,
            int messageFormat,
            int destinationFormat,
            byte[] destinationBuffer
    );
}
