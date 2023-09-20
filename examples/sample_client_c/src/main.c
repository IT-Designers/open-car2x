#include <stdio.h>
#include <connector.h>
#include <connector_helper.h>

#include "vehicle.h"
#include "echo.h"
#include "fusion.h"
#include "util.h"

void print_connector_info() {
    char buffer[128];
    CrConnectorInfo connector_info;

    BAIL("loading connector info", cr_load_connector_info(&connector_info));
    BAIL("stringifying connector version",
         cr_version_to_string(buffer, sizeof(buffer), connector_info.connector_version));
    printf("Application started, found Connector %s with ", buffer);
    BAIL("stringifying protocol version",
         cr_version_to_string(buffer, sizeof(buffer), connector_info.protocol_version));
    printf("protocol %s\n", buffer);
}

int main(int argc, char *argv[]) {
    print_connector_info();

    if (argc > 1) {
        if (strcmp("--demo-vehicle", argv[1]) == 0) {
            vehicle_main(argc - 2, &argv[2]);
        } else if (strcmp("--demo-fusion", argv[1]) == 0) {
            fusion_main(argc - 2, &argv[2]);
        } else if (strcmp("--help", argv[1]) == 0 || strcmp("-h", argv[1]) == 0) {
            printf("Sample Impl for the BMWi project LUKAS\n");
            printf("Available arguments\n");
            printf("    --demo-vehicle [--debug-component-status] [--hostname]\n");
            printf("    --demo-fusion\n");
            printf("    --help\n");
        } else {
            printf("Unknown argument '%s'\n", argv[1]);
            printf("Available options are --demo-vehicle, --debug-component-status, --debug, ...\n");
            exit(1);
        }
        exit(0);
    } else {
        printf("Starting in standalone-demo-mode.\n");
        cr_util_sleep_millis(2000);
        return echo_main(argc, argv);
    }
}