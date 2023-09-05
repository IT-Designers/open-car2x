package de.itdesigners.lukas.connector;


import java.util.Objects;

import javax.annotation.Nonnull;

public class Version {

    private final byte major;
    private final byte minor;
    private final byte patch;
    private final String build;

    public Version(byte major, byte minor, byte patch, @Nonnull String build) {
        this.major = major;
        this.minor = minor;
        this.patch = patch;
        this.build = build;
    }

    public byte getMajor() {
        return major;
    }

    public byte getMinor() {
        return minor;
    }

    public byte getPatch() {
        return patch;
    }

    @Nonnull
    public String getBuild() {
        return build;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Version version = (Version) o;
        return major == version.major && minor == version.minor && patch == version.patch && Objects.equals(build, version.build);
    }

    @Override
    public int hashCode() {
        return Objects.hash(major, minor, patch, build);
    }

    @Nonnull
    @Override
    public String toString() {
        return "Version{" +
                "major=" + major +
                ", minor=" + minor +
                ", patch=" + patch +
                ", build='" + build + '\'' +
                "}@" + hashCode();
    }

    @Nonnull
    public String toStringPretty() {
        return String.format("v%d.%d.%d-%s", major, minor, patch, build);
    }

}
