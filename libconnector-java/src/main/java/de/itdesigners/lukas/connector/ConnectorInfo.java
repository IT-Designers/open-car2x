package de.itdesigners.lukas.connector;

import javax.annotation.Nonnull;

public class ConnectorInfo {

    private final @Nonnull Version version;
    private final @Nonnull Version protocol;

    public ConnectorInfo(@Nonnull Version version, @Nonnull Version protocol) {
        this.version  = version;
        this.protocol = protocol;
    }

    @Nonnull
    public Version getVersion() {
        return version;
    }

    @Nonnull
    public Version getProtocolVersion() {
        return protocol;
    }

    @Override
    public String toString() {
        return "ConnectorInfo{" +
                "version=" + version +
                ", protocol=" + protocol +
                "}@" + hashCode();
    }
}
