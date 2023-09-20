package de.itdesigners.lukas.connectortest;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;

import java.io.Closeable;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

import de.itdesigners.lukas.connector.Connection;
import de.itdesigners.lukas.connector.ConnectionInfo;
import de.itdesigners.lukas.connector.Format;
import de.itdesigners.lukas.connector.Message;
import de.itdesigners.lukas.connector.MessageId;
import itd.ssdm.descriptions.ItdSsdmDescriptions;

public class DebugMessageSubscriber implements Closeable, AutoCloseable {

    private @NonNull final Connection                                      connection;
    private @NonNull final List<ItdSsdmDescriptions.DebugRequestMessageId> messageIds;
    private @NonNull final List<ItdSsdmDescriptions.DebugRequestMessageId> activeSubscriptions;

    private @Nullable ConnectionInfo connectionInfo;


    public DebugMessageSubscriber(
            @NonNull Connection connection,
            @NonNull List<ItdSsdmDescriptions.DebugRequestMessageId> messageIds) {
        this.connection          = connection;
        this.messageIds          = messageIds;
        this.activeSubscriptions = new ArrayList<>(messageIds.size());
    }

    public void checkForReconnect() {
        this.checkForReconnect(connection.getInfo());
    }

    public void checkForReconnect(@NonNull ConnectionInfo connectionInfo) {
        if (!connectionInfo.equals(this.connectionInfo)) {
            this.activeSubscriptions.clear();

            for (ItdSsdmDescriptions.DebugRequestMessageId mid : this.messageIds) {
                connection.sendMessage(new Message(
                        MessageId.DebugRequest,
                        Format.Protobuf,
                        ItdSsdmDescriptions.DebugRequest
                                .newBuilder()
                                .setMessageId(mid)
                                .setMode(ItdSsdmDescriptions.DebugRequestMode.DEBUG_REQUEST_MODE_SUBSCRIBE)
                                .build()
                                .toByteArray()
                ));

                this.activeSubscriptions.add(mid);
            }

            // set it after all the messages were sent, so if that fails
            // a poll based system will try again
            this.connectionInfo = connectionInfo;
        }
    }


    public static Builder newBuilder() {
        return new Builder();
    }

    @Override
    public void close() throws IOException {
        for (ItdSsdmDescriptions.DebugRequestMessageId mid : this.activeSubscriptions) {
            connection.sendMessage(new Message(
                    MessageId.DebugRequest,
                    Format.Protobuf,
                    ItdSsdmDescriptions.DebugRequest
                            .newBuilder()
                            .setMessageId(mid)
                            .setMode(ItdSsdmDescriptions.DebugRequestMode.DEBUG_REQUEST_MODE_UNSUBSCRIBE)
                            .build()
                            .toByteArray()
            ));
        }
    }

    public static class Builder {

        private @NonNull List<ItdSsdmDescriptions.DebugRequestMessageId> messageIds = new ArrayList<>();

        public Builder addSubscription(ItdSsdmDescriptions.DebugRequestMessageId messageId) {
            this.messageIds.add(messageId);
            return this;
        }

        @NonNull
        public DebugMessageSubscriber build(@NonNull Connection connection) {
            List<ItdSsdmDescriptions.DebugRequestMessageId> messageIds;

            if (this.messageIds.contains(ItdSsdmDescriptions.DebugRequestMessageId.DEBUG_REQUEST_MESSAGE_ID_ALL)) {
                messageIds = new ArrayList<>(1);
                messageIds.add(ItdSsdmDescriptions.DebugRequestMessageId.DEBUG_REQUEST_MESSAGE_ID_ALL);
            } else {
                messageIds = this.messageIds;
            }

            return new DebugMessageSubscriber(
                    connection,
                    messageIds
            );
        }
    }
}
