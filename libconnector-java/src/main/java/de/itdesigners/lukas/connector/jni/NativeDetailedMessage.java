package de.itdesigners.lukas.connector.jni;

import javax.annotation.Nonnull;

import de.itdesigners.lukas.connector.DetailedMessage;
import de.itdesigners.lukas.connector.Format;
import de.itdesigners.lukas.connector.Message;
import de.itdesigners.lukas.connector.MessageId;

public class NativeDetailedMessage implements DetailedMessage  {

    private final @Nonnull Message content;
    private                long    ptr;

    protected NativeDetailedMessage(@Nonnull Message content, long ptr) {
        this.content = content;
        this.ptr     = ptr;
    }

    @Nonnull
    protected static NativeDetailedMessage load(long ptr) throws NativeResultException {
        return new NativeDetailedMessage(
                new Message(
                        MessageId.fromNumericOrThrow(
                                NativeResultException.fromErrorCodeAndValue(nativeGetContentMessageId(ptr))
                        ),
                        Format.Protobuf,
                        NativeConverter.convertFromMessageRef(
                                NativeResultException.fromErrorCodeAndValue(nativeGetContent(ptr)),
                                Format.Uper,
                                Format.Protobuf
                        )
                ),
                ptr
        );
    }

    @Nonnull
    @Override
    public Message getContent() {
        return content;
    }

    @Override
    public long getCreationTimeMillis() {
        return NativeResultException.fromErrorCodeAndValue(nativeGetCreationTimeMillis(ptr));
    }

    @Override
    public long getReceptionTimeMillis() {
        return NativeResultException.fromErrorCodeAndValue(nativeGetReceptionTimeMillis(ptr));
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
                NativeResultException.fromErrorCode(nativeDelete(ptr));
            } catch (RuntimeException e) {
                // recover the pointer
                this.ptr = ptr;
                throw e;
            }
        }
    }

    private static native long nativeDelete(long ptr);
    private static native long[] nativeGetContent(long ptr);
    private static native long[] nativeGetContentMessageId(long ptr);
    private static native long[] nativeGetCreationTimeMillis(long ptr);
    private static native long[] nativeGetReceptionTimeMillis(long ptr);
}
