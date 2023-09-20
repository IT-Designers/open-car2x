#include <stdio.h>

// #define CRH_DEBUG_PRINT_ENCODED_DATA

#include <connector.h>
#include <connector_helper.h>

#include "vehicle.h"
#include "util.h"
#include "version.h"


static const CrApplicationInfo APPLICATION_INFO = {
        .identity = CR_IDENTITY_VEHICLE,
        .version = CLIENT_VERSION,
        .name = "Sample Vehicle"
};

bool check_for_reconnect(const CrConnection *connection, CrConnectionInfo *info);

void print_available_component_status(CrConnection *connection);

void subscribe_for_component_status_updates(CrConnection *connection);

void vehicle_main(int argc, char *argv[]) {
    // cr_configure_logger(CR_LOG_LEVEL_WARN);

    bool component_status_debugging_flag = is_component_status_debugging_flag_set(argc, argv);
    char* hostname = get_host_argv(argc, argv);

    CrConnectionInfo connectionInfo = {};
    CrConnectionConfig *config = NULL;
    CrConnection *connection = NULL;
    CrConnectionInfo info = {};

    BAIL("creating a config", cr_create_config(&config));
    BAIL("updating host address", cr_config_set_address(config, hostname != NULL ? hostname : "localhost:5672"));
    BAIL("updating reconnect timout", cr_config_set_reconnect_timeout_millis(config, 5000));
    BAIL("updating send timeout", cr_config_set_send_timeout_millis(config, 2500));
    BAIL("updating login method", cr_config_set_login_anonymous(config, true));
    // BAIL("updating user", cr_config_set_login_user(config, "prod1"));
    // BAIL("updating password", cr_config_set_login_password(config, "password1"));
    BAIL("updating source", cr_config_set_source_exchange(config, "messages"));
    BAIL("updating target", cr_config_set_target_exchange(config, "messages"));
    // BAIL("updating filter", cr_config_set_receive_filter(config, CR_MESSAGE_ID_COMPONENT_STATUS | CR_MESSAGE_ID_CPM));
    BAIL("updating filter", cr_config_set_receive_own(config, true));
    BAIL("updating station_id", cr_config_set_station_id(config, 42)); // for this demo we pretend to be the edge-server
    BAIL("updating station_id_receive_filter", cr_config_set_station_id_receive_filter(config, 42)); // only receive messages from the edge-server

    BAIL("creating a connection handle", cr_create_connection_with_config(&connection, &APPLICATION_INFO, config));
    BAIL("destroying the config", cr_destroy_config(&config));
    BAIL("await the connection to be established", await_connection_established(connection, &info));

    // cr_util_sleep_millis(500);
    if (component_status_debugging_flag) {
        subscribe_for_component_status_updates(connection);
    }

    cr_util_sleep_millis(1000);
    for (int i = 0; i < 500; ++i) {
        cr_util_sleep_millis(100);
        print_available_component_status(connection);

        if (check_for_reconnect(connection, &info) && component_status_debugging_flag) {
            subscribe_for_component_status_updates(connection);
        }

        CPM_t *cpm = crh_alloc_cpm();
        cpm->header.protocolVersion = 6; // TODO doesn't matter (for now?)
        cpm->header.messageID = 1;       // TODO doesn't matter (for now?)
        cpm->header.stationID = 1337;
        cpm->cpm.generationDeltaTime = 123;
        cpm->cpm.cpmParameters.managementContainer.stationType = 1;
        cpm->cpm.cpmParameters.managementContainer.perceivedObjectContainerSegmentInfo = NULL;
        cpm->cpm.cpmParameters.managementContainer.referencePosition.latitude = 2;
        cpm->cpm.cpmParameters.managementContainer.referencePosition.longitude = 3;
        cpm->cpm.cpmParameters.managementContainer.referencePosition.positionConfidenceEllipse.semiMajorConfidence = 4;
        cpm->cpm.cpmParameters.managementContainer.referencePosition.positionConfidenceEllipse.semiMinorConfidence = 5;
        cpm->cpm.cpmParameters.managementContainer.referencePosition.positionConfidenceEllipse.semiMajorOrientation = 6;
        cpm->cpm.cpmParameters.managementContainer.referencePosition.altitude.altitudeValue = 7;
        cpm->cpm.cpmParameters.managementContainer.referencePosition.altitude.altitudeConfidence = 15;
        cpm->cpm.cpmParameters.stationDataContainer = NULL;
        cpm->cpm.cpmParameters.sensorInformationContainer = NULL;
        cpm->cpm.cpmParameters.perceivedObjectContainer = NULL;
        cpm->cpm.cpmParameters.freeSpaceAddendumContainer = calloc(1, sizeof(FreeSpaceAddendumContainer_t));
        cpm->cpm.cpmParameters.numberOfPerceivedObjects = 42;
        cpm->cpm.cpmParameters.timeReference = NULL;

        FreeSpaceAddendum_t *fsa = calloc(1, sizeof(FreeSpaceAddendum_t));
        fsa->freeSpaceConfidence = 4;
        fsa->freeSpaceArea.present = FreeSpaceArea_PR_freeSpaceCircular;
        fsa->freeSpaceArea.choice.freeSpaceCircular.nodeCenterPoint = NULL;
        fsa->freeSpaceArea.choice.freeSpaceCircular.radius = 2;
        fsa->sensorIDList = NULL;
        fsa->shadowingApplies = NULL;
        asn_sequence_add(&(cpm->cpm.cpmParameters.freeSpaceAddendumContainer->list), fsa);

        cpm->cpm.cpmParameters.perceivedObjectContainer = calloc(1, sizeof(PerceivedObjectContainer_t));

        PerceivedObject_t* obj = calloc(1, sizeof(PerceivedObject_t));
        // obj->xSpeed.confidence = SpeedConfidence_equalOrWithinOneDecimeterPerSec; // default/zero: invalid
        // obj->ySpeed.confidence = SpeedConfidence_equalOrWithinOneDecimeterPerSec; // default/zero: invalid
        obj->planarSpeed.present = SpeedContainer_PR_polarSpeed; // default/zero: 'SpeedContainer_PR_NOTHING'
        obj->planarSpeed.choice.polarSpeed.confidence = SpeedConfidence_equalOrWithinOneDecimeterPerSec; // default/zero: invalid
        asn_sequence_add(&(cpm->cpm.cpmParameters.perceivedObjectContainer->list), obj);

        //BAIL("printing cpm", crh_print_cpm(cpm, stdout));
        BAIL("checking constraints", crh_check_cpm(cpm, stderr));
        BAIL("sending cpm", crh_send_cpm(connection, cpm));


        crh_free_cpm(&cpm);

        CAM_t *cam = crh_alloc_cam();
        cam->header.protocolVersion = 6; // TODO doesn't matter (for now?)
        cam->header.messageID = 2;       // TODO doesn't matter (for now?)
        cam->header.stationID = 1337;
        cam->cam.generationDeltaTime = 123;
        cam->cam.camParameters.highFrequencyContainer.present = HighFrequencyContainer_PR_basicVehicleContainerHighFrequency;
        cam->cam.camParameters.highFrequencyContainer.choice.basicVehicleContainerHighFrequency.heading.headingConfidence = 1;
        cam->cam.camParameters.highFrequencyContainer.choice.basicVehicleContainerHighFrequency.speed.speedConfidence = 1;
        cam->cam.camParameters.highFrequencyContainer.choice.basicVehicleContainerHighFrequency.vehicleLength.vehicleLengthConfidenceIndication = 1;
        cam->cam.camParameters.highFrequencyContainer.choice.basicVehicleContainerHighFrequency.vehicleLength.vehicleLengthValue = 1;
        cam->cam.camParameters.highFrequencyContainer.choice.basicVehicleContainerHighFrequency.vehicleWidth = 1;
        BAIL("printing cam", crh_print_cam(cam, stdout));
        BAIL("checking constraints", crh_check_cam(cam, stderr));
        BAIL("sending cam", crh_send_cam(connection, cam));

        crh_free_cam(&cam);

        // check for early abort (regression test for the connector)
        if (i % 10 == 0) {
            BAIL("retrieving connection info", cr_load_connection_info(connection, &connectionInfo));
            if (connectionInfo.message_sender_queue_size > 5) {
                break;
            }
        }
    }

    BAIL("retrieving connection info", cr_load_connection_info(connection, &connectionInfo));
    printf("message_sender_queue_size = %d\n", connectionInfo.message_sender_queue_size);
    if (connectionInfo.message_sender_queue_size > 0) {
        printf("Waiting for 1000ms for the sender queue to be flushed\n");
        cr_util_sleep_millis(1000);
        BAIL("retrieving connection info", cr_load_connection_info(connection, &connectionInfo));
        printf("message_sender_queue_size = %d\n", connectionInfo.message_sender_queue_size);

        // this should not happen, otherwise the connector failed/deadlocked internally
        if (connectionInfo.message_sender_queue_size > 0) {
            exit(1);
        }
    }

    if (is_component_status_debugging_flag_set(argc, argv)) {
        for (int i = 0; i < 50; ++i) {
            cr_util_sleep_millis(100);
            print_available_component_status(connection);
            print_available_messages(connection);

            if (check_for_reconnect(connection, &info) && component_status_debugging_flag) {
                subscribe_for_component_status_updates(connection);
            }
        }
    }

    BAIL("destroying the connection", cr_destroy_connection(&connection));
}

void subscribe_for_component_status_updates(CrConnection *connection) {
    printf("Subscribing to debug information for system components...\n");
    DebugRequest_t *request = crh_alloc_debug_request();
    request->message_id = DebugRequest__message_id_component_status;
    request->mode = DebugRequest__mode_subscribe;
    BAIL("sending a debug request", crh_send_debug_request(connection, request));
    crh_free_debug_request(&request);
}

void print_available_component_status(CrConnection *connection) {
    ComponentStatus_t *component_status = NULL;
    while (crh_receive_component_status(connection, &component_status, 0) == CR_RESULT_OK) {
        if (component_status == NULL) {
            break;
        }
        printf("ComponentStatus.other = 0x%02x\n", *(component_status->other.buf));
        crh_print_component_status(component_status, stdout);
        crh_free_component_status(&component_status);
    }
}

bool check_for_reconnect(const CrConnection *connection, CrConnectionInfo *info) {
    CrConnectionInfo info_old = *info;
    BAIL("retrieving connection info", cr_load_connection_info(connection, info));

    if (info->times_connected_counter != info_old.times_connected_counter ||
        (info_old.status != CR_CONNECTION_STATUS_CONNECTED &&
         info->status == CR_CONNECTION_STATUS_CONNECTED)) {
        printf("Connector reconnected at %ld ms (unix epoch time)\n", info->connection_epoch_millis_timestamp);
        return true;
    } else {
        return false;
    }
}
