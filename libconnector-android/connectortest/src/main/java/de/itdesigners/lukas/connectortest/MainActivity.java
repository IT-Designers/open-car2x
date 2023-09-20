package de.itdesigners.lukas.connectortest;

import android.os.Bundle;
import android.util.Log;
import android.widget.TextView;

import androidx.appcompat.app.AppCompatActivity;

import java.io.IOException;
import java.util.Arrays;
import java.util.Optional;

import de.itdesigners.lukas.connector.ApplicationInfo;
import de.itdesigners.lukas.connector.Connection;
import de.itdesigners.lukas.connector.ConnectionConfig;
import de.itdesigners.lukas.connector.ConnectionInfo;
import de.itdesigners.lukas.connector.Connector;
import de.itdesigners.lukas.connector.ConnectorInfo;
import de.itdesigners.lukas.connector.DetailedMessage;
import de.itdesigners.lukas.connector.Format;
import de.itdesigners.lukas.connector.Identity;
import de.itdesigners.lukas.connector.Message;
import de.itdesigners.lukas.connector.MessageId;
import de.itdesigners.lukas.connector.Version;
import de.itdesigners.lukas.connector.jni.NativeConnector;
import iso.standard.signalized_intersection.profilec.dsrc.version2.Dsrc;
import itd.ssdm.descriptions.ItdSsdmDescriptions;
import itu_t.identified_organization.etsi.its_domain.wg1.tr.cpm.version.CpmPduDescriptions;

public class MainActivity extends AppCompatActivity {

    private Connector  connector;
    private Connection connection;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        new Thread(() -> {
            try {
                Thread.sleep(1000);
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
            Log.i("main", "creating connector");

            connector  = new NativeConnector();
            connection = connector.connect(
                    new ApplicationInfo(
                            Identity.NomadicDevice,
                            new Version((byte) 0, (byte) 1, (byte) 0, "local"),
                            "connectortest"
                    ),
                    new ConnectionConfig()
                            // .setAddress("192.168.3.55:5672")
                            //.setAddress("10.150.42.171:5672")
                            .setAddress("192.168.3.112:5672")
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

            final ConnectorInfo info = connector.getInfo();
            final DebugMessageSubscriber subscriber = DebugMessageSubscriber
                    .newBuilder()
                    .addSubscription(ItdSsdmDescriptions.DebugRequestMessageId.DEBUG_REQUEST_MESSAGE_ID_COMPONENT_STATUS)
                    .build(connection);

            runOnUiThread(() -> {
                TextView view = findViewById(R.id.textViewHelloWorld);
                view.setText(info.getVersion().toStringPretty());
            });

            // connection.sendMessage(new Message(
            //         MessageId.DebugRequest,
            //         Format.Protobuf,
            //         ItdSsdmDescriptions.DebugRequest
            //                 .newBuilder()
            //                 .setMessageId(ItdSsdmDescriptions.DebugRequestMessageId.DEBUG_REQUEST_MESSAGE_ID_COMPONENT_STATUS)
            //                 .setMode(ItdSsdmDescriptions.DebugRequestMode.DEBUG_REQUEST_MODE_SUBSCRIBE)
            //                 .build()
            //                 .toByteArray()
            // ));

            byte[] container = CpmPduDescriptions.StationDataContainer
                    .newBuilder()
                    .setOriginatingVehicleContainer(CpmPduDescriptions.OriginatingVehicleContainer.newBuilder().build())
                    .build()
                    .toByteArray();

            Log.i("main", "bytes=" + Arrays.toString(container));

            try {
                while (true) {

                    ConnectionInfo            connectionInfo = connection.getInfo();
                    Optional<DetailedMessage> message        = connection.receiveDetailedMessage(
                            1000,
                            MessageId.Cpm,
                            MessageId.ComponentStatus
                    );

                    subscriber.checkForReconnect(connectionInfo);

                    Log.i("main", "message=" + message);
                    Log.i("main", "connectionInfo=" + connectionInfo);

                    if (message.isPresent()) {
                        DetailedMessage detailed = message.get();
                        Message msg = detailed.getContent();
                        long now = System.currentTimeMillis();

                        Log.i("main.detailed", "Created at " + detailed.getCreationTimeMillis() + ", received at " + detailed.getReceptionTimeMillis() + ", diff => " + (detailed.getReceptionTimeMillis() - detailed.getCreationTimeMillis()));
                        Log.i("main.detailed", "Received at " + detailed.getReceptionTimeMillis() + ", now is " + now + ", diff =>" + (now - detailed.getReceptionTimeMillis()));

                        if (Format.Protobuf == msg.getFormat() && MessageId.Cpm == msg.getId()) {
                            CpmPduDescriptions.Cpm cpm = CpmPduDescriptions.Cpm.parseFrom(msg.getData());
                            Log.i("main", "cpm=" + cpm);


                            runOnUiThread(() -> {
                                TextView view = findViewById(R.id.textViewHelloWorld);
                                view.setText(cpm.toString());
                            });


                            CpmPduDescriptions.Cpm cpmWithChoice = cpm
                                    .toBuilder()
                                    .setCpm(cpm
                                                    .getCpm()
                                                    .toBuilder()
                                                    .setCpmParameters(cpm
                                                                              .getCpm()
                                                                              .getCpmParameters()
                                                                              .toBuilder()
                                                                              .setStationDataContainer(
                                                                                      CpmPduDescriptions.StationDataContainer
                                                                                              .newBuilder()
                                                                                              .setOriginatingRsuContainer(
                                                                                                      CpmPduDescriptions.OriginatingRsuContainer
                                                                                                              .newBuilder()
                                                                                                              .setRoadSegmentReferenceId(
                                                                                                                      Dsrc.RoadSegmentReferenceId
                                                                                                                              .newBuilder()
                                                                                                                              .setId(
                                                                                                                                      Dsrc.RoadSegmentId
                                                                                                                                              .newBuilder()
                                                                                                                                              .setValue(
                                                                                                                                                      100)
                                                                                                                                              .build())
                                                                                                                              .build())
                                                                                                              .build()
                                                                                              )
                                                                                              .build()
                                                                              )))
                                    .build();


                            runOnUiThread(() -> {
                                TextView view = findViewById(R.id.textViewHelloWorld);
                                view.setText(cpmWithChoice.toString());
                            });

                            Log.i("main", "cpmWithChoice=" + cpmWithChoice);

                            Message response = new Message(
                                    MessageId.Cpm,
                                    Format.Protobuf,
                                    cpmWithChoice.toByteArray()
                            );

                            byte[] protobuf = connector.getConverter().convert(
                                    response.getId(),
                                    Format.Uper,
                                    connector.getConverter().convert(
                                            response.getId(),
                                            response.getFormat(),
                                            response.getData(),
                                            Format.Uper
                                    ),
                                    response.getFormat()
                            );

                            Log.i("main", "proto.bin.eq=" + (response.getData() == protobuf));
                            Log.i(
                                    "main",
                                    "proto.des.eq=" + (cpmWithChoice.equals(CpmPduDescriptions.Cpm.parseFrom(protobuf)))
                            );
                            Log.i("main", "proto.org=" + cpmWithChoice);
                            Log.i("main", "proto.des=" + CpmPduDescriptions.Cpm.parseFrom(protobuf));


                            connection.sendMessage(response);

                        }  else if (Format.Protobuf == msg.getFormat() && MessageId.ComponentStatus == msg.getId()) {
                            ItdSsdmDescriptions.ComponentStatus status = ItdSsdmDescriptions.ComponentStatus.parseFrom(msg.getData());
                            Log.i("main", "component_status="+status);
                        }
                    }
                }
            } catch (Throwable t) {
                t.printStackTrace();
            }
        }).start();
    }

    @Override
    protected void onDestroy() {
        super.onDestroy();
        if (connection != null) {
            try {
                Log.i("connection", "closing");
                connection.close();
                connection = null;
                connector  = null;
                Log.i("connection", "closed");
            } catch (IOException e) {
                Log.e("connection", "close failed", e);
            }
        }
    }
}
