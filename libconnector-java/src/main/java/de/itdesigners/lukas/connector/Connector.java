package de.itdesigners.lukas.connector;

import javax.annotation.Nonnull;
import javax.annotation.Nullable;

public interface Connector {

    @Nonnull
    Connection connect(@Nonnull ApplicationInfo info, @Nullable ConnectionConfig config);

    @Nonnull
    ConnectorInfo getInfo();

    @Nonnull
    Converter getConverter();
}
