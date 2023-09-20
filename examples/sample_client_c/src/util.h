#ifndef SAMPLE_CLIENT_UTIL_H
#define SAMPLE_CLIENT_UTIL_H

#include <stdio.h>
#include <connector.h>


inline static void BAIL(char *message, CrResult result) {
    if (result != CR_RESULT_OK) {
        fprintf(
                stderr,
                "Encountered an error, bailing out of %s: %d, text representation: '%s'\n",
                message,
                result,
                cr_result_str(result)
        );
        exit(result);
    }
}

inline static CrResult await_connection_established(CrConnection *connection, CrConnectionInfo *info) {
    CrResult result;
    info->status = -1;
    uint16_t connected_counter = info->times_connected_counter;

    do {
        CrConnectionStatus status = info->status;
        if ((result = cr_load_connection_info(connection, info)) != CR_RESULT_OK) {
            fprintf(
                    stderr,
                    "Loading connection info failed: %d, text representation: '%s'\n",
                    result,
                    cr_result_str(result)
            );
            return result;
        }
        if (status != info->status) {
            printf("Connection status: %d, text representation: '%s'\n", info->status,
                   cr_connection_status_str(info->status));
        }
        if (info->status != CR_CONNECTION_STATUS_CONNECTED) {
            cr_util_sleep_millis(100);
        }
    } while (info->times_connected_counter == connected_counter);

    return result;
}

inline static bool is_component_status_debugging_flag_set(int argc, char *argv[]) {
    static const char *const POSSIBLE_FLAG[] = {"--debug-component-status", "--debug"};
    static const size_t POSSIBLE_FLAG_COUNT = sizeof(POSSIBLE_FLAG) / sizeof(size_t);

    for (int i = 0; i < argc; ++i) {
        for (size_t n = 0; n < POSSIBLE_FLAG_COUNT; ++n) {
            if (0 == strcmp(POSSIBLE_FLAG[n], argv[i])) {
                return true;
            }
        }
    }

    return false;
}

inline static char* get_host_argv(int argc, char *argv[]) {
    static const char *const POSSIBLE_FLAG[] = {"--hostname", "--host"};
    static const size_t POSSIBLE_FLAG_COUNT = sizeof(POSSIBLE_FLAG) / sizeof(size_t);

    for (int i = 0; i < argc; ++i) {
        for (size_t n = 0; n < POSSIBLE_FLAG_COUNT; ++n) {
            if (0 == strcmp(POSSIBLE_FLAG[n], argv[i]) && i + 1 < argc) {
                return argv[i + 1];
            }
        }
    }

    return NULL;
}

inline static void print_available_messages(CrConnection *connection) {
    CrMessage container = {.id = 0, .size = 0};
    while (cr_receive_message(connection, &container, 0) == CR_RESULT_OK) {
        void *message = NULL;
        CrMessageRef ref = crh_message_to_ref(&container);

        if (crh_decode_uper_from_message_ref(&ref, &message) == CR_RESULT_OK) {
            crh_print(container.id, message, stdout);
            crh_free(container.id, &message);
        } else {
            fprintf(stderr, "Failed to decode message");
        }
    }

}

#endif //SAMPLE_CLIENT_UTIL_H
