package de.itdesigners.lukas.connector;

import java.util.Arrays;
import java.util.Objects;

import javax.annotation.Nonnull;

public class Message {

    private final @Nonnull MessageId id;
    private final @Nonnull Format    format;
    private final @Nonnull byte[]    data;

    public Message(@Nonnull MessageId id, @Nonnull Format format, @Nonnull byte[] data) {
        this.id     = id;
        this.format = format;
        this.data   = data;
    }

    @Nonnull
    public MessageId getId() {
        return id;
    }

    @Nonnull
    public Format getFormat() {
        return format;
    }

    @Nonnull
    public byte[] getData() {
        return data;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o)
            return true;
        if (o == null || getClass() != o.getClass())
            return false;
        Message message = (Message) o;
        return id == message.id && format == message.format && Arrays.equals(data, message.data);
    }

    @Override
    public int hashCode() {
        int result = Objects.hash(id, format);
        result = 31 * result + Arrays.hashCode(data);
        return result;
    }

    @Nonnull
    @Override
    public String toString() {
        return "Message{" +
                "id=" + id +
                ", format=" + format +
                ", data=" + Arrays.toString(data) +
                "}@" + hashCode();
    }
}
