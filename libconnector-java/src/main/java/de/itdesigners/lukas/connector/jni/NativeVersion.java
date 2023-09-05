package de.itdesigners.lukas.connector.jni;

import javax.annotation.Nonnull;

import de.itdesigners.lukas.connector.Version;

class NativeVersion {

    @Nonnull
    protected static Version loadFromPtr(long ptr) throws NativeResultException {
        long[] values = NativeResultException.fromErrorCodeAndValues(nativeGetMajorMinorPatch(ptr));
        String build  = nativeGetBuildOrNull(ptr);

        return new Version(
                (byte) values[0],
                (byte) values[1],
                (byte) values[2],
                build
        );
    }

    private static native long[] nativeGetMajorMinorPatch(long ptr);
    private static native String nativeGetBuildOrNull(long ptr);
}
