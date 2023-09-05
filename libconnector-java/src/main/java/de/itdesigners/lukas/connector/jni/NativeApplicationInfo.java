package de.itdesigners.lukas.connector.jni;

import javax.annotation.Nonnull;

import de.itdesigners.lukas.connector.ApplicationInfo;

class NativeApplicationInfo {

    protected static long toNative(@Nonnull ApplicationInfo info) {
        return create(
                info.getIdentity().getNumeric(),
                info.getVersion().getMajor(),
                info.getVersion().getMinor(),
                info.getVersion().getPatch(),
                info.getVersion().getBuild(),
                info.getName()
        );
    }

    protected static native long create(
            int identity,
            byte versionMajor,
            byte versionMinor,
            byte versionPatch,
            String versionBuild,
            String name
    );
}
