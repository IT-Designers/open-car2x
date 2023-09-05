package de.itdesigners.lukas.connector.jni;

import javax.annotation.Nonnull;

import de.itdesigners.lukas.connector.ConnectionConfig;
import de.itdesigners.lukas.connector.MessageId;

class NativeConnectionConfig {

    protected static long toNative(@Nonnull ConnectionConfig config) {
        long obj = nativeCreateDefault();

        try {
            if (config.getAddress() != null) {
                NativeResultException.fromErrorCode(nativeSetAddress(obj, config.getAddress()));
            }

            if (config.getReconnectTimeoutMillis() != null) {
                NativeResultException.fromErrorCode(nativeSetReconnectTimeoutMillis(obj, config.getReconnectTimeoutMillis()));
            }

            if (config.getSendTimeoutMillis() != null) {
                NativeResultException.fromErrorCode(nativeSetSendTimeoutMillis(obj, config.getSendTimeoutMillis()));
            }

            if (config.getReceiveOwnMessage() != null) {
                NativeResultException.fromErrorCode(nativeSetReceiveOwnMessage(obj, config.getReceiveOwnMessage()));
            }

            if (config.getFilterOptions() != null) {
                long numericOverloaded = 0;
                for (MessageId id : config.getFilterOptions()) {
                    numericOverloaded |= id.getNumeric();
                }
                NativeResultException.fromErrorCode(nativeSetFilterOptions(obj, numericOverloaded));
            }

            if (config.getLoginUser() != null) {
                NativeResultException.fromErrorCode(nativeSetLoginUser(obj, config.getLoginUser()));
            }

            if (config.getLoginPassword() != null) {
                NativeResultException.fromErrorCode(nativeSetLoginPassword(obj, config.getLoginPassword()));
            }

            if (config.getAnonymous() != null) {
                NativeResultException.fromErrorCode(nativeSetLoginAnonymous(obj, config.getAnonymous()));
            }

            if (config.getTargetExchange() != null) {
                NativeResultException.fromErrorCode(nativeTargetExchange(obj, config.getTargetExchange()));
            }

            if (config.getSourceExchange() != null) {
                NativeResultException.fromErrorCode(nativeSourceExchange(obj, config.getSourceExchange()));
            }

            if (config.getStationId() != null) {
                NativeResultException.fromErrorCode(nativeStationId(obj, (int) config.getStationId()));
            }

            if (config.getStationIdReceiveFilter() != null) {
                NativeResultException.fromErrorCode(nativeStationIdReceiveFilter(obj, (int) config.getStationIdReceiveFilter()));
            }

        } catch (RuntimeException | Error e) {
            nativeDelete(obj);
            throw e;
        }

        return obj;
    }

    private static native long nativeCreateDefault();

    private static native void nativeDelete(long obj);

    private static native long nativeSetAddress(long obj, @Nonnull String address);

    private static native long nativeSetReconnectTimeoutMillis(long obj, long millis);

    private static native long nativeSetSendTimeoutMillis(long obj, long millis);

    private static native long nativeSetReceiveOwnMessage(long obj, boolean receive);

    private static native long nativeSetFilterOptions(long obj, long filterOverloaded);

    private static native long nativeSetLoginUser(long obj, String user);

    private static native long nativeSetLoginPassword(long obj, String user);

    private static native long nativeSetLoginAnonymous(long obj, boolean anonymous);

    private static native long nativeTargetExchange(long obj, String exchange);

    private static native long nativeSourceExchange(long obj, String exchange);

    private static native long nativeStationId(long obj, int stationId);

    private static native long nativeStationIdReceiveFilter(long obj, int stationId);

}
