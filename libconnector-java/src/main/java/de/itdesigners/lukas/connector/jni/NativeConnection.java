package de.itdesigners.lukas.connector.jni;

import java.util.Optional;

import javax.annotation.Nonnull;
import javax.annotation.Nullable;

import de.itdesigners.lukas.connector.ApplicationInfo;
import de.itdesigners.lukas.connector.Connection;
import de.itdesigners.lukas.connector.ConnectionConfig;
import de.itdesigners.lukas.connector.ConnectionInfo;
import de.itdesigners.lukas.connector.DetailedMessage;
import de.itdesigners.lukas.connector.Format;
import de.itdesigners.lukas.connector.Message;
import de.itdesigners.lukas.connector.MessageId;

class NativeConnection implements Connection {

    private long ptr;

    protected NativeConnection(@Nonnull ApplicationInfo info, @Nullable ConnectionConfig config) throws NativeResultException {
        ptr = NativeResultException.fromErrorCodeAndPtr(nativeCreate(
                NativeApplicationInfo.toNative(info),
                config != null ? NativeConnectionConfig.toNative(config) : 0
        ));
    }

    @Nonnull
    @Override
    public ConnectionInfo getInfo() throws NativeResultException {
        long ptr = NativeResultException.fromErrorCodeAndPtr(nativeCreateConnectionInfo(this.ptr));
        return NativeConnectionInfo.loadAndFree(ptr);
    }

    @Nonnull
    @Override
    public Optional<Message> receiveMessage(long timeoutMillis, @Nonnull MessageId[] accept) throws NativeResultException {
        long[] result = nativeReceiveMessageProtobuf(ptr, timeoutMillis, MessageId.toNumericOverloaded(accept));
        if (result[0] == NativeResultException.ErrWorkerRequestTimeoutReached) {
            return Optional.empty();
        } else {
            long message = NativeResultException.fromErrorCodeAndPtr(result);
            return Optional.of(NativeMessage.loadAndFree(message, Format.Protobuf));
        }
    }

    @Nonnull
    @Override
    public Optional<DetailedMessage> receiveDetailedMessage(long timeoutMillis, @Nonnull MessageId... accept) {
        long[] result = nativeReceiveDetailedMessage(ptr, timeoutMillis, MessageId.toNumericOverloaded(accept));
        if (result[0] == NativeResultException.ErrWorkerRequestTimeoutReached) {
            return Optional.empty();
        } else {
            long message = NativeResultException.fromErrorCodeAndPtr(result);
            return Optional.of(NativeDetailedMessage.load(message));
        }
    }

    @Override
    public void sendMessage(@Nonnull Message message) {
        NativeResultException.fromErrorCode(nativeSendMessage(
                ptr,
                message.getId().getNumeric(),
                message.getFormat().getNumeric(),
                message.getData()
        ));
    }

    @Override
    public void close() throws NativeResultException {
        this.delete();
    }

    @Override
    protected void finalize() throws Throwable {
        super.finalize();
        this.delete();
    }

    private void delete() throws NativeResultException {
        if (this.ptr != 0) {
            long ptr = this.ptr;
            this.ptr = 0;

            try {
                NativeResultException.fromErrorCode(nativeStopAndDelete(ptr));
            } catch (RuntimeException e) {
                // recover the pointer
                this.ptr = ptr;
                throw e;
            }
        }
    }

    private static native long[] nativeCreate(long info, long config);
    private static native long nativeStopAndDelete(long ptr);
    private static native long[] nativeCreateConnectionInfo(long ptr);
    private static native long[] nativeReceiveMessageProtobuf(long ptr, long timeoutMillis, long messageIdsOverloaded);
    private static native long[] nativeReceiveDetailedMessage(long ptr, long timeoutMillis, long messageIdsOverloaded);
    private static native long nativeSendMessage(long ptr, long messageId, long  format, byte[] data);
}
