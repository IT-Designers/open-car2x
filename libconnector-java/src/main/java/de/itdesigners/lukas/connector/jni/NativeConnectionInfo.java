package de.itdesigners.lukas.connector.jni;

import javax.annotation.Nonnull;

import de.itdesigners.lukas.connector.ConnectionInfo;
import de.itdesigners.lukas.connector.ConnectionStatus;
import de.itdesigners.lukas.connector.MessageId;

class NativeConnectionInfo {

    @Nonnull
    protected static ConnectionInfo loadAndFree(long ptr) throws NativeResultException {
        try {
            return load(ptr);
        } finally {
            nativeDelete(ptr);
        }
    }

    @Nonnull
    protected static ConnectionInfo load(long ptr) throws NativeResultException {
        return new ConnectionInfo(
                ConnectionStatus.fromNumericOrThrow((int) NativeResultException.fromErrorCodeAndValue(nativeGetStatus(ptr))),
                (int) NativeResultException.fromErrorCodeAndValue(nativeGetTimesConnectedCounter(ptr)),
                NativeResultException.fromErrorCodeAndValue(nativeGetConnectionEpochMillis(ptr)),
                (int) NativeResultException.fromErrorCodeAndValue(nativeGetMessageReceiverQueueSize(ptr)),
                MessageId.fromNumericArrayOrThrow(
                        NativeResultException.fromErrorCodeAndValues(nativeGetMessageReceiverQueueTypes(ptr))
                ),
                (int) NativeResultException.fromErrorCodeAndValue(nativeGetMessageSenderQueueSize(ptr))
        );
    }

    private static native void nativeDelete(long ptr);
    private static native long[] nativeGetStatus(long ptr);
    private static native long[] nativeGetTimesConnectedCounter(long ptr);
    private static native long[] nativeGetConnectionEpochMillis(long ptr);
    private static native long[] nativeGetMessageReceiverQueueSize(long ptr);
    private static native long[] nativeGetMessageReceiverQueueTypes(long ptr);
    private static native long[] nativeGetMessageSenderQueueSize(long ptr);
}
