package de.itdesigners.lukas.connector;

import java.io.Closeable;
import java.util.Optional;

import javax.annotation.Nonnull;

public interface Connection extends Closeable, AutoCloseable {

    @Nonnull
    ConnectionInfo getInfo();

    /**
     * @param timeoutMillis The time in millis to wait. Zero will return immediately.
     * @return An {@link Optional} wrapped {@link Message} that was received.
     */
    @Nonnull
    Optional<Message> receiveMessage(long timeoutMillis, @Nonnull MessageId...accept);

    /**
     * @param timeoutMillis The time in millis to wait. Zero will return immediately.
     * @return An {@link Optional} wrapped {@link DetailedMessage} that was received.
     */
    @Nonnull
    Optional<DetailedMessage> receiveDetailedMessage(long timeoutMillis, @Nonnull MessageId...accept);

    void sendMessage(@Nonnull Message message);
}
