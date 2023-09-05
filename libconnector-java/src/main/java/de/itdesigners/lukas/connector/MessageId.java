package de.itdesigners.lukas.connector;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

import javax.annotation.Nonnull;

public enum MessageId {
    Denm(0b1),
    Cpm(0b10),
    Cam(0b100),
    Vam(0b1000),
    Mcm(0b10000),

    ApplicationInfo(0b1___00000000_00000000),
    DebugRequest(0b10___00000000_00000000),
    ComponentStatus(0b100___00000000_00000000);

    protected final long numeric;

    MessageId(long numeric) {
        this.numeric = numeric;
    }

    public long getNumeric() {
        return numeric;
    }

    @Nonnull
    public static Optional<MessageId> fromNumeric(long numeric) {
        for (MessageId id : MessageId.values()) {
            if (id.numeric == numeric) {
                return Optional.of(id);
            }
        }
        return Optional.empty();
    }

    @Nonnull
    public static MessageId fromNumericOrThrow(long numeric) {
        return fromNumeric(numeric)
                .orElseThrow(() -> new RuntimeException("Invalid numeric value(" + numeric + ") for " + MessageId.class.getSimpleName()));
    }

    @Nonnull
    public static List<MessageId> fromNumericArrayOrThrow(@Nonnull long[] numerics) {
        List<MessageId> list = new ArrayList<>(numerics.length);
        for (long numeric : numerics) {
            list.add(fromNumericOrThrow(numeric));
        }
        return list;
    }

    public static long toNumericOverloaded(@Nonnull MessageId[] numerics) {
        long numeric = 0;
        for (MessageId id : numerics) {
            numeric |= id.numeric;
        }
        return numeric;
    }
}
