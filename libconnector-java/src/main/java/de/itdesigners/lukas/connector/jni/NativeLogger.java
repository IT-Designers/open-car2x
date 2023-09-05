package de.itdesigners.lukas.connector.jni;

import javax.annotation.Nonnull;

public class NativeLogger {

    public enum LogLevel {
        None(0),
        Error(1),
        Warn(2),
        Info(3),
        Debug(4),
        Trace(5);

        protected final int numeric;

        LogLevel(int numeric) {
            this.numeric = numeric;
        }
    }

    /**
     * Enables logging for internal events using the given {@link LogLevel} as filter level. This operation only
     * succeeds, if the logger is not configured yet. Changing the {@link LogLevel} is not possible.
     *
     * @param level Minimum {@link LogLevel} to print
     * @throws NativeResultException If setting the {@link LogLevel} failed
     */
    public static void configure(@Nonnull LogLevel level) throws NativeResultException {
        NativeResultException.fromErrorCode(nativeConfigure(level.numeric));
    }

    private static native long nativeConfigure(int logLevel);
}
