package de.itdesigners.lukas.connector;

import java.util.Collections;
import java.util.List;
import java.util.Objects;

import javax.annotation.Nonnull;

public class ConnectionInfo {

    private final @Nonnull ConnectionStatus status;
    private final          int              timesConnectedCounter;
    private final          long             connectionEpochMillis;
    private final          int              messageReceiverQueueSize;
    private final @Nonnull List<MessageId>  messageReceiverQueueTypes;
    private final          int              messageSenderQueueSize;

    public ConnectionInfo(
            @Nonnull ConnectionStatus status,
            int timesConnectedCounter,
            long connectionEpochMillis,
            int messageReceiverQueueSize,
            List<MessageId> messageReceiverQueueTypes,
            int messageSenderQueueSize) {
        this.status                    = status;
        this.timesConnectedCounter     = timesConnectedCounter;
        this.connectionEpochMillis     = connectionEpochMillis;
        this.messageReceiverQueueSize  = messageReceiverQueueSize;
        this.messageReceiverQueueTypes = Collections.unmodifiableList(messageReceiverQueueTypes);
        this.messageSenderQueueSize    = messageSenderQueueSize;
    }

    @Nonnull
    public ConnectionStatus getStatus() {
        return status;
    }

    public int getTimesConnectedCounter() {
        return timesConnectedCounter;
    }

    public long getConnectionEpochMillis() {
        return connectionEpochMillis;
    }

    public int getMessageReceiverQueueSize() {
        return messageReceiverQueueSize;
    }

    @Nonnull
    public List<MessageId> getMessageReceiverQueueTypes() {
        return messageReceiverQueueTypes;
    }

    public int getMessageSenderQueueSize() {
        return messageSenderQueueSize;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o)
            return true;
        if (o == null || getClass() != o.getClass())
            return false;
        ConnectionInfo that = (ConnectionInfo) o;
        return timesConnectedCounter == that.timesConnectedCounter && connectionEpochMillis == that.connectionEpochMillis && messageReceiverQueueSize == that.messageReceiverQueueSize && status == that.status && messageReceiverQueueTypes
                .equals(that.messageReceiverQueueTypes);
    }

    @Override
    public int hashCode() {
        return Objects.hash(
                status,
                timesConnectedCounter,
                connectionEpochMillis,
                messageReceiverQueueSize,
                messageReceiverQueueTypes
        );
    }

    @Nonnull
    @Override
    public String toString() {
        return "ConnectionInfo{" +
                "status=" + status +
                ", timesConnectedCounter=" + timesConnectedCounter +
                ", connectionEpochMillis=" + connectionEpochMillis +
                ", messageReceiverQueueSize=" + messageReceiverQueueSize +
                ", messageReceiverQueueTypes=" + messageReceiverQueueTypes +
                "}@" + hashCode();
    }
}
