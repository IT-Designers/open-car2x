#include <stdio.h>
#include <connector.h>
#include <connector_helper.h>

#include "echo.h"
#include "version.h"
#include "util.h"


static const CrApplicationInfo APPLICATION_INFO = {
        .identity = CR_IDENTITY_VEHICLE,
        .version = CLIENT_VERSION,
        .name = "Echo Sample"
};

void print_overloaded_message_ids(const char *message, CrMessageId message_id) {
    char buffer[128];
    cr_message_ids_to_string(buffer, sizeof(buffer), message_id);
    printf("%s: %s\n", message, buffer);
}

CrResult print_uper_message_as_json_pretty(CrMessageRef message) {
    char json[1024];
    CrResult result;
    if ((result = cr_message_uper_to_json_pretty(message, json, sizeof(json))) == 0) {
        printf("%s\n", json);
        fflush(stdout);
    }
    return result;
}


CrResult wait_for_connected(CrConnection *connection) {
    // todo spinning loop without sleep
    CrResult result = CR_RESULT_OK;
    CrConnectionInfo info = {.status = -1};

    do {
        CrConnectionStatus status = info.status;
        if ((result = cr_load_connection_info(connection, &info)) != CR_RESULT_OK) {
            fprintf(
                    stderr,
                    "Loading connection info failed: %d, text representation: '%s'\n",
                    result,
                    cr_result_str(result)
            );
            return result;
        }
        if (status != info.status) {
            printf("Connection status: %d, text representation: '%s'\n", info.status,
                   cr_connection_status_str(info.status));
        }
    } while (info.times_connected_counter == 0);

    return result;
}

CrResult demonstration_1_send_receive_successfully(CrConnection *connection) {
    CrResult result = CR_RESULT_OK;


    // sending message 1, supposed to succeed
    CPM_t *cpm_to_send = crh_alloc_cpm();
    // cpm_to_send->header.timestamp = -1; // provoke an encoding error
    if ((result = crh_send_cpm(connection, cpm_to_send)) == CR_RESULT_OK) {
        printf("Message 1: CPM sent\n");
        crh_free_cpm(&cpm_to_send);
    } else {
        fprintf(
                stderr,
                "Message 1: CPM not sent: '%s'\n",
                cr_result_str(result)
        );
        crh_free_cpm(&cpm_to_send);
        return result;
    }

    // receiving message 1, supposed to succeed
    CPM_t *cpm = NULL;
    if ((result = crh_receive_cpm(connection, &cpm, 100)) == 0) {
        printf("Message 1: CPM loaded\n");
        crh_free_cpm(&cpm);
    } else {
        fprintf(stderr, "Message 1: CPM not loaded: '%s'\n", cr_result_str(result));
        return result;
    }

    return result;
}

CrResult demonstration_2_send_receiving_raw(CrConnection *connection) {
    CrResult result = CR_RESULT_OK;


    // sending message 2, supposed to succeed (this is a raw + dummy + invalid message)
    CrMessage send_message = {.id = CR_MESSAGE_ID_CPM, .size = 14};
    if ((result = cr_send_message(connection, &send_message)) != CR_RESULT_OK) {
        fprintf(
                stderr,
                "Message 2: Sending a message failed with error: %d, text representation: '%s'\n",
                result,
                cr_result_str(result)
        );
        return result;
    } else {
        printf(
                "Message 2: Sending a message of type: '%s' and length: %lu\n",
                cr_message_id_str(send_message.id),
                send_message.size
        );
    }


    // receiving message 2, supposed to fail / timeout (this is a raw + dummy + invalid message)
    CrMessage receive_message = {.id = CR_MESSAGE_ID_DENM, .size = 0};
    if ((result = cr_receive_message(connection, &receive_message, 1000)) !=
        CR_RESULT_ERR_WORKER_REQUEST_TIMEOUT_REACHED) {
        fprintf(
                stderr,
                "Message 2: Receiving a message failed with error: %d, text representation: '%s'\n",
                result,
                cr_result_str(result)
        );
        return result;
    } else {
        printf("Message 2: Receiving a message with 0ms timeout successfully timed out\n");
    }

    for (int i = 0; i < 10; i++) {
        cr_util_sleep_millis(100);
        CrConnectionInfo info;

        if (CR_RESULT_OK == cr_load_connection_info(connection, &info)) {
            if (info.message_receiver_queue_size > 0) {
                char buffer[1024];
                cr_message_ids_to_string(buffer, sizeof(buffer), info.message_receiver_queue_types);
                printf("There is/are %d message(s) in the receiver buffer with type(s): %s\n",
                       info.message_receiver_queue_size, buffer);
                break;
            }
        }
    }

    // receiving message 2, supposed to succeed (remove the invalid message from the message queue)
    receive_message = (CrMessage) {.id = CR_MESSAGE_ID_CPM, .size = 0};
    if ((result = cr_receive_message(connection, &receive_message, 1000)) != CR_RESULT_OK) {
        fprintf(
                stderr,
                "Message 2: Receiving a message failed with error: %d, text representation: '%s'\n",
                result,
                cr_result_str(result)
        );
        return result;
    } else {
        printf(
                "Message 2: Received a message of type: '%s' and length: %lu\n",
                cr_message_id_str(receive_message.id),
                receive_message.size
        );
    }

    return result;
}

CrResult demonstration_3_send_receive_successfully_print_json(CrConnection *connection) {
    CrResult result = CR_RESULT_OK;



    // sending message 3, supposed to succeed
    CPM_t *cpm_to_send = crh_alloc_cpm();
    if ((result = crh_send_cpm(connection, cpm_to_send)) == CR_RESULT_OK) {
        printf("Message 3: CPM sent\n");
        crh_free_cpm(&cpm_to_send);
    } else {
        fprintf(
                stderr,
                "Message 3: CPM not sent: '%s'\n",
                cr_result_str(result)
        );
        crh_free_cpm(&cpm_to_send);
        return result;
    }

    // receiving message 3, supposed to succeed
    CrMessage receive_message = {.id = CR_MESSAGE_ID_CPM | CR_MESSAGE_ID_DENM, .size = 0};
    print_overloaded_message_ids("Receive CrMessageId filter", receive_message.id);
    if ((result = cr_receive_message(connection, &receive_message, 1000)) != 0) {
        fprintf(
                stderr,
                "Message 3: Receiving a message failed with error: %d, text representation: '%s'\n",
                result,
                cr_result_str(result)
        );
        return result;
    } else {
        printf(
                "Message 3: Received a message of type: '%s' and length: %lu\n",
                cr_message_id_str(receive_message.id),
                receive_message.size
        );
        if ((result = print_uper_message_as_json_pretty(crh_message_to_ref(&receive_message))) != 0) {
            return result;
        }
    }


    return result;
}

CrResult demonstration_4_send_receive_detailed_message(CrConnection *connection) {
    CrResult result = CR_RESULT_OK;

    // sending message 4, supposed to succeed (this is a raw + dummy + invalid message)
    CPM_t *cpm_to_send = crh_alloc_cpm();
    if ((result = crh_send_cpm(connection, cpm_to_send)) == CR_RESULT_OK) {
        printf("Message 4: CPM sent\n");
        crh_free_cpm(&cpm_to_send);
    } else {
        fprintf(
                stderr,
                "Message 4: Sending a message failed with error: %d, text representation: '%s'\n",
                result,
                cr_result_str(result)
        );
        crh_free_cpm(&cpm_to_send);
        return result;
    }

    CrDetailedMessage message_detailed = (CrDetailedMessage) {
        .content = {.id = CR_MESSAGE_ID_CPM, .size = 0, .body = NULL},
        .details = NULL
    };

    if ((result = cr_receive_detailed_message(connection, &message_detailed, 1000)) != CR_RESULT_OK) {
        fprintf(
                stderr,
                "Message 4: Receiving a message failed with error: %d, text representation: '%s'\n",
                result,
                cr_result_str(result)
        );
        return result;
    } else {
        printf(
                "Message 4: Received a message of type: '%s' and length: %lu\n",
                cr_message_id_str(message_detailed.content.id),
                message_detailed.content.size
        );
    }

    uint64_t creation_time = 0;
    uint64_t reception_time = 0;
    uint64_t now_time = cr_util_unix_epoch_time_millis();

    if ((result = cr_detailed_message_creation_time(&message_detailed, &creation_time)) != CR_RESULT_OK) {
        fprintf(
                stderr,
                "Message 4: Retrieving the creation time failed with error: %d, text representation: '%s'\n",
                result,
                cr_result_str(result)
        );
        return result;
    } else if ((result = cr_detailed_message_reception_time(&message_detailed, &reception_time)) != CR_RESULT_OK) {
        fprintf(
                stderr,
                "Message 4: Retrieving the reception time failed with error: %d, text representation: '%s'\n",
                result,
                cr_result_str(result)
        );
        return result;
    } else {
        printf(
                "Message 4: Was created at %ld and received at %ld => diff %ld ms\n",
                creation_time,
                reception_time,
                reception_time - creation_time
        );
        printf(
                "Message 4: Now is %ld and received at %ld => diff %ld ms\n",
                now_time,
                reception_time,
                now_time - reception_time
        );
    }


    CPM_t *cpm_received = NULL;
    if ((result = crh_decode_uper_from_message_ref(&message_detailed.content, (void**) &cpm_received)) == CR_RESULT_OK) {
        printf("Message 4: CPM decoded\n");
        crh_free_cpm(&cpm_received);
    } else {
        fprintf(
                stderr,
                "Message 4: CPM failed to decode with error: %d, text representation: '%s'\n",
                result,
                cr_result_str(result)
        );
        return result;
    }

    if ((result = cr_detailed_message_free(&message_detailed)) != CR_RESULT_OK) {
        fprintf(
                stderr,
                "Message 4: Freeing message failed with error: %d, text representation: '%s'\n",
                result,
                cr_result_str(result)
        );
        return result;
    } else {
        printf("Message 4: Freed\n");
    }

    return result;
}

int echo_main(int argc, char *argv[]) {
    (void)argc; // ignore unused
    (void)argv; // ignore unused

    cr_configure_logger(CR_LOG_LEVEL_DEBUG);

    CrConnectionConfig *config = NULL;
    CrConnection *connection = NULL;
    CrResult result = CR_RESULT_OK;

    if ((result = cr_create_config(&config)) != CR_RESULT_OK) {
        fprintf(
                stderr,
                "Creating a config failed with errod: %d, text representation: '%s'\n",
                result,
                cr_result_str(result)
        );
        return result;
    } else {
        // normally, the connector creates a filter that prevents the sender from receiving its own messages again,
        // but for this example we want to see our own messages again!
        cr_config_set_receive_own(config, true);

        // forwarding NULL is fine here
        cr_config_set_address(config, getenv("AMQP_HOST"));
    }

    // there also exists `cr_create_connection(connection, application_info, address)`
    if ((result = cr_create_connection_with_config(&connection, &APPLICATION_INFO, config)) != CR_RESULT_OK) {
        fprintf(
                stderr,
                "Create failed with error: %d, text representation: '%s'\n",
                result,
                cr_result_str(result)
        );
        return result;
    } else {
        printf("Connection created\n");
    }


    // await connection
    if ((result = wait_for_connected(connection)) != CR_RESULT_OK) {
        return result;
    }

    if ((result = demonstration_1_send_receive_successfully(connection)) != CR_RESULT_OK) {
        return result;
    }

    if ((result = demonstration_2_send_receiving_raw(connection)) != CR_RESULT_OK) {
        return result;
    }

    if ((result = demonstration_3_send_receive_successfully_print_json(connection)) != CR_RESULT_OK) {
        return result;
    }

    if ((result = demonstration_4_send_receive_detailed_message(connection)) != CR_RESULT_OK) {
        return result;
    }


    printf("Going to destroy connection\n");
    if ((result = cr_destroy_connection(&connection)) != 0) {
        fprintf(
                stderr,
                "Destruction failed with error: %d, text representation: '%s'\n",
                result,
                cr_result_str(result)
        );
        return result;
    }

    printf("Connection destroyed\n");
    return 0;
}
