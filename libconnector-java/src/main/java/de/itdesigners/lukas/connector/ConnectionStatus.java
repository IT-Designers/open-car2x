package de.itdesigners.lukas.connector;

import java.util.Optional;

import javax.annotation.Nonnull;

public enum ConnectionStatus {
    Connected(0),
    Disconnected(1),
    Initializing(2),
    Connecting(3),
    OpeningSession(4),
    OpeningSender(5),
    OpeningReceiver(6);

    protected final int numeric;

    ConnectionStatus(int numeric) {
        this.numeric = numeric;
    }

    public int getNumeric() {
        return numeric;
    }

    @Nonnull
    public static Optional<ConnectionStatus> fromNumeric(int numeric) {
        for (ConnectionStatus status : ConnectionStatus.values()) {
            if (status.numeric == numeric) {
                return Optional.of(status);
            }
        }
        return Optional.empty();
    }

    @Nonnull
    public static ConnectionStatus fromNumericOrThrow(int numeric) {
        return fromNumeric(numeric)
                .orElseThrow(() -> new RuntimeException("Invalid numeric value(" + numeric + ") for " + ConnectionStatus.class.getSimpleName()));
    }
}
