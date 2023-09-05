# Getting Started

A very basic example of how to connect, receive and send messages:

```java
public class SomeActivity extends AppCompatActivity {

    private Connector  connector;
    private Connection connection;
    
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        // usual android stuff ...

        NativeLogger.configure(NativeLoger.LogLevel.Info);

        connector  = new NativeConnector();
        connection = connector.connect(
                new ApplicationInfo(
                        Identity.NomadicDevice,
                        new Version((byte) 0, (byte) 1, (byte) 0, "local"),
                        "connectortest"
                ),
                new ConnectionConfig()
                        .setAddress("127.0.0.1:5672")
                        .setReconnectTimeoutMillis(5_000L)
                        .setSendTimeoutMillis(5_000L)
                        .setReceiveOwnMessage(false)
                        .setFilterOptions(Arrays.asList(
                                MessageId.Denm,
                                MessageId.Cpm
                        ))
                        .setLoginUser("user")
                        .setLoginPassword("password")
                        .setAnonymous(true)
                        .setTargetExchange("messages")
                        .setSourceExchange("messages")
        );
        Log.i("main", "connector=" + connection);
        Log.i("main", "connectionInfo=" + connection.getInfo());

        new Thread(() -> {
            while (true) {
    
                ConnectionInfo    connectionInfo = connection.getInfo();
                Optional<Message> message        = connection.receiveMessage(
                        1000,
                        MessageId.Cpm,
                        MessageId.ComponentStatus
                );
    
                Log.i("main", "message=" + message);
                Log.i("main", "connectionInfo=" + connectionInfo);
    
    
                if (message.isPresent()) {
                    Message msg = message.get();
    
                    if (Format.Protobuf == msg.getFormat() && MessageId.Cpm == msg.getId()) {
                        CpmPduDescriptions.Cpm cpm = CpmPduDescriptions.Cpm.parseFrom(msg.getData());
                        Log.i("main", "cpm=" + cpm);
    
                        Message response = new Message(
                                MessageId.Cpm,
                                Format.Protobuf,
                                cpm.toByteArray()
                        );
    
                        connection.sendMessage(response);
    
                    } else if (Format.Protobuf == msg.getFormat() && MessageId.ComponentStatus == msg.getId()) {
                        ItdSsdmDescriptions.ComponentStatus status = ItdSsdmDescriptions.ComponentStatus.parseFrom(msg.getData());
                        Log.i("main", "component_status="+status);
                    }
                } 
    
            }
        }).start();
    }
    
}
    
```

# Changelog

The changelog can be found [here](CHANGELOG.md)
