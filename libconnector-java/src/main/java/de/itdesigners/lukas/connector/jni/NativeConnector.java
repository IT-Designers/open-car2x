package de.itdesigners.lukas.connector.jni;

import java.util.concurrent.atomic.AtomicBoolean;

import javax.annotation.Nonnull;
import javax.annotation.Nullable;

import de.itdesigners.lukas.connector.ApplicationInfo;
import de.itdesigners.lukas.connector.Connection;
import de.itdesigners.lukas.connector.ConnectionConfig;
import de.itdesigners.lukas.connector.Connector;
import de.itdesigners.lukas.connector.ConnectorInfo;
import de.itdesigners.lukas.connector.Converter;

public class NativeConnector implements Connector {

    private static final AtomicBoolean loaded = new AtomicBoolean(false);

    public NativeConnector() {
        ensureLibraryLoaded();
    }

    @Nonnull
    public Connection connect(@Nonnull ApplicationInfo info, @Nullable ConnectionConfig config) throws NativeResultException {
        return new NativeConnection(info, config);
    }

    @Nonnull
    public ConnectorInfo getInfo() throws NativeResultException {
        return NativeConnectorInfo.load();
    }

    @Nonnull
    @Override
    public Converter getConverter() {
        return new NativeConverter();
    }

    private synchronized static void ensureLibraryLoaded() {
        if (!loaded.get()) {
            System.loadLibrary("connector_jni");
            nativeInit();
            loaded.set(true);
        }
    }

    private static native void nativeInit();
}
