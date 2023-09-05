package de.itdesigners.lukas.connector.jni;

import javax.annotation.Nonnull;

import de.itdesigners.lukas.connector.Format;
import de.itdesigners.lukas.connector.Message;
import de.itdesigners.lukas.connector.MessageId;

class NativeMessage {

    @Nonnull
    public static Message loadAndFree(long message, @Nonnull Format format) throws NativeResultException {
        try {
            return load(message, format);
        } finally {
            nativeDelete(message);
        }
    }

    @Nonnull
    public static Message load(long message, @Nonnull Format format) throws NativeResultException {
        int    size = (int) NativeResultException.fromErrorCodeAndValue(nativeGetDataSize(message));
        byte[] data = new byte[size];

        int sizeCopied = (int) NativeResultException.fromErrorCodeAndValue(nativeGetData(message, data));

        if (sizeCopied != size) {
            throw new RuntimeException("Copied unexpected amount of bytes, expected=" + size + " but copied=" + sizeCopied);
        }

        return new Message(
                MessageId.fromNumericOrThrow(NativeResultException.fromErrorCodeAndValue(nativeGetMessageId(message))),
                format,
                data
        );
    }

    private static native void nativeDelete(long ptr);
    private static native long[] nativeGetMessageId(long ptr);
    private static native long[] nativeGetDataSize(long ptr);
    private static native long[] nativeGetData(long ptr, byte[] target);
}
