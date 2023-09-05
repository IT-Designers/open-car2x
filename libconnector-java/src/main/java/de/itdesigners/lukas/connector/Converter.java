package de.itdesigners.lukas.connector;

import javax.annotation.Nonnull;

public interface Converter {

    @Nonnull
    byte[] convert(
            @Nonnull MessageId mid,
            @Nonnull Format sourceFormat,
            @Nonnull byte[] source,
            @Nonnull Format destinationFormat);
}
