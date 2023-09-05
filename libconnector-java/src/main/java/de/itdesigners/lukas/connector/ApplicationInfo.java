package de.itdesigners.lukas.connector;

import java.util.Objects;

import javax.annotation.Nonnull;

public class ApplicationInfo {

    private final @Nonnull Identity identity;
    private final @Nonnull Version  version;
    private final @Nonnull String   name;

    public ApplicationInfo(@Nonnull Identity identity, @Nonnull Version version, @Nonnull String name) {
        this.identity = identity;
        this.version  = version;
        this.name     = name;
    }

    @Nonnull
    public Identity getIdentity() {
        return identity;
    }

    @Nonnull
    public Version getVersion() {
        return version;
    }

    @Nonnull
    public String getName() {
        return name;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o)
            return true;
        if (o == null || getClass() != o.getClass())
            return false;
        ApplicationInfo that = (ApplicationInfo) o;
        return identity == that.identity && version.equals(that.version) && name.equals(that.name);
    }

    @Override
    public int hashCode() {
        return Objects.hash(identity, version, name);
    }

    @Override
    public String toString() {
        return "ApplicationInfo{" +
                "identity=" + identity +
                ", version=" + version +
                ", name='" + name + '\'' +
                "}@" + hashCode();
    }
}
