package de.itdesigners.lukas.connector;

import java.io.Closeable;

import javax.annotation.Nonnull;

public interface DetailedMessage extends Closeable, AutoCloseable {

    @Nonnull
    Message getContent();

    /**
     * @return The time the related {@link Message} was created in millis since the unix epoch time
     */
    long getCreationTimeMillis();

    /**
     * @return The time the related {@link Message} was received in millis since the unix epoch time
     */
    long getReceptionTimeMillis();

}
