#include <stdio.h>

#include <connector.h>
#include <connector_helper.h>

#include "fusion.h"
#include "util.h"
#include "version.h"

static const CrApplicationInfo APPLICATION_INFO = {
        .identity = CR_IDENTITY_FUSION_MODULE,
        .version = CLIENT_VERSION,
        .name = "Sample Fusion"
};

void fusion_main(int argc, char *argv[]) {
    (void)argc; // ignore unused
    (void)argv; // ignore unused

    cr_configure_logger(CR_LOG_LEVEL_INFO);

    CrConnection *connection = NULL;
    CrConnectionInfo info;

    BAIL("creating a connection handle", cr_create_connection(&connection, &APPLICATION_INFO, NULL));
    BAIL("await the connection to be established", await_connection_established(connection, &info));

    while (true) {
        BAIL("loading connection info", cr_load_connection_info(connection, &info));

        if (info.status != CR_CONNECTION_STATUS_CONNECTED) {
            printf("No longer connected, exiting");
            exit(0);
        }

        DENM_t *denm = crh_alloc_denm();
        denm->header.protocolVersion = 6; // TODO doesn't matter (for now?)
        denm->header.messageID = ItsPduHeader__messageID_denm; // TODO doesn't matter (for now?)
        denm->header.stationID = 42;
        denm->denm.management.actionID.originatingStationID = 42;
        denm->denm.management.actionID.originatingStationID = 0;
        asn_long2INTEGER(&(denm->denm.management.detectionTime), TimestampIts_utcStartOf2004);
        asn_long2INTEGER(&(denm->denm.management.referenceTime), TimestampIts_oneMillisecAfterUTCStartOf2004);
        denm->denm.management.termination = NULL;
        denm->denm.management.eventPosition.latitude = 3;
        denm->denm.management.eventPosition.longitude = -3;
        denm->denm.management.eventPosition.positionConfidenceEllipse.semiMajorConfidence = 3;
        denm->denm.management.eventPosition.positionConfidenceEllipse.semiMinorConfidence = 2;
        denm->denm.management.eventPosition.positionConfidenceEllipse.semiMajorOrientation = 1;
        denm->denm.management.eventPosition.altitude.altitudeValue = 123;
        denm->denm.management.eventPosition.altitude.altitudeConfidence = 1;
        denm->denm.management.relevanceDistance = NULL;
        denm->denm.management.relevanceTrafficDirection = NULL;
        denm->denm.management.validityDuration = NULL;
        denm->denm.management.transmissionInterval = NULL;
        denm->denm.management.stationType = StationType_passengerCar;
        denm->denm.situation = NULL;
        denm->denm.location= NULL;
        denm->denm.alacarte = NULL;

        BAIL("printing denm", crh_print_denm(denm, stdout));
        BAIL("checking constraints", crh_check_denm(denm, stderr));
        BAIL("sending denm", crh_send_denm(connection, denm));

        crh_free_denm(&denm);
        cr_util_sleep_millis(100);
        print_available_messages(connection);
    }
}
