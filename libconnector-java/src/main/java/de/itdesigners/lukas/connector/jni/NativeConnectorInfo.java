package de.itdesigners.lukas.connector.jni;

import javax.annotation.Nonnull;

import de.itdesigners.lukas.connector.ConnectorInfo;

class NativeConnectorInfo {

    @Nonnull
    protected static ConnectorInfo load() throws NativeResultException {
        long obj = NativeResultException.fromErrorCodeAndPtr(nativeLoad());

        try {
            long versionPtr = NativeResultException.fromErrorCodeAndPtr(nativeGetVersionPtr(obj));
            long protocolPtr = NativeResultException.fromErrorCodeAndPtr(nativeGetProtocolPtr(obj));

            return new ConnectorInfo(
                    NativeVersion.loadFromPtr(versionPtr),
                    NativeVersion.loadFromPtr(protocolPtr)
            );
        } finally {
            nativeDelete(obj);
        }
    }

    private static native long[] nativeLoad();
    private static native void nativeDelete(long obj);
    private static native long[] nativeGetVersionPtr(long obj);
    private static native long[] nativeGetProtocolPtr(long obj);
}
