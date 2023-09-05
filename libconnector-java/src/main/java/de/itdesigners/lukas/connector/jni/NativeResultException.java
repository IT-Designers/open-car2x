package de.itdesigners.lukas.connector.jni;

import javax.annotation.Nullable;

public class NativeResultException extends RuntimeException {

    protected static final int ErrWorkerRequestTimeoutReached = 0x0020_0003;

    private final           long   errorCode;
    private final           long   pointer;
    private final @Nullable String errorMessage;

    private NativeResultException(long errorCode, long pointer, @Nullable String errorMessage) {
        super(
                "The error-code(" + errorCode + ") and pointer(" + pointer + ") indicate an error"
                        + (errorMessage != null ? (": " + errorMessage) : "")
        );
        this.errorCode    = errorCode;
        this.pointer      = pointer;
        this.errorMessage = errorMessage;
    }

    public long getErrorCode() {
        return errorCode;
    }

    public long getPointer() {
        return pointer;
    }

    @Nullable
    public String getErrorMessage() {
        return errorMessage;
    }

    public static void fromErrorCode(long errorCode) throws NullPointerException, NativeResultException {
        if (errorCode != 0) {
            throw new NativeResultException(
                    errorCode,
                    0,
                    getMessage(errorCode)
            );
        }
    }

    public static long fromErrorCodeAndPtr(long[] result) throws NullPointerException, NativeResultException {
        if (result == null) {
            throw new NullPointerException();
        } else {
            long errorCode = result[0];
            long pointer   = result[1];

            if (errorCode != 0 || pointer == 0) {
                throw new NativeResultException(
                        errorCode,
                        pointer,
                        getMessage(errorCode)
                );
            } else {
                return pointer;
            }
        }
    }

    public static long fromErrorCodeAndValue(long[] result) throws NullPointerException, NativeResultException {
        if (result == null) {
            throw new NullPointerException();
        } else {
            long errorCode = result[0];
            long value     = result[1];

            if (errorCode != 0) {
                throw new NativeResultException(
                        errorCode,
                        0,
                        getMessage(errorCode)
                );
            } else {
                return value;
            }
        }
    }

    public static long[] fromErrorCodeAndValues(long[] result) throws NullPointerException, NativeResultException {
        if (result == null) {
            throw new NullPointerException();
        } else {
            long errorCode = result[0];

            if (errorCode != 0) {
                throw new NativeResultException(
                        errorCode,
                        0,
                        getMessage(errorCode)
                );
            } else {
                long[] values = new long[result.length - 1];
                System.arraycopy(result, 1, values, 0, values.length);
                return values;
            }
        }
    }

    private static native String getMessage(long errorCode);
}
