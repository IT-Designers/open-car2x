#ifndef SAMPLE_CLIENT_CONNECTOR_HELPER_H
#define SAMPLE_CLIENT_CONNECTOR_HELPER_H

#include <connector.h>
#include <asn_application.h>
#include <DENM.h>
#include <CPM.h>
#include <CAM.h>
#include <VAM.h>
#include <MCM.h>
#include <DebugRequest.h>
#include <ComponentStatus.h>

#ifndef CRH_ERROR_BUFFER_SIZE
#define CRH_ERROR_BUFFER_SIZE 1024
#endif

inline static asn_TYPE_descriptor_t *crh_get_asn_type_descriptor_for_message_id(CrMessageId mid) {
    switch (mid) {
        case CR_MESSAGE_ID_DENM:
            return &asn_DEF_DENM;
        case CR_MESSAGE_ID_CPM:
            return &asn_DEF_CPM;
        case CR_MESSAGE_ID_CAM:
            return &asn_DEF_CAM;
        case CR_MESSAGE_ID_VAM:
            return &asn_DEF_VAM;
        case CR_MESSAGE_ID_MCM:
            return &asn_DEF_MCM;

        case CR_MESSAGE_ID_COMPONENT_STATUS:
            return &asn_DEF_ComponentStatus;
        case CR_MESSAGE_ID_DEBUG_REQUEST:
            return &asn_DEF_DebugRequest;

        default:
            return NULL;
    }
}

inline static CrResult crh_send(CrConnection *connection, CrMessageId mid, void *message) {
    asn_TYPE_descriptor_t *descriptor = crh_get_asn_type_descriptor_for_message_id(mid);

    if (descriptor == NULL) {
        return CR_RESULT_ERR_PARAMETER_MESSAGE_ID_IS_INVALID;
    } else if (message == NULL) {
        return CR_RESULT_ERR_PARAMETER_MESSAGE_IS_NULL;
    }

#ifndef CRH_NO_CONSTRAINT_CHECK
    char buffer[CRH_ERROR_BUFFER_SIZE];
    size_t buffer_len = sizeof(buffer);
    if (asn_check_constraints(descriptor, message, buffer, &buffer_len) != 0) {
#ifndef CRH_NO_CONSTRAINT_CHECK_LOG
        fflush(stdout);
        fflush(stderr);
        fprintf(stderr, "Constraint check failed for '%s' because '%s'\n", descriptor->name, buffer);
        fflush(stderr);
#endif
        return CR_RESULT_ERR_CONSTRAINT_CHECK_VIOLATION_FOUND;
    }
#endif

    CrMessage container = {.id = mid};
    asn_enc_rval_t r = uper_encode_to_buffer(descriptor, NULL, message, container.body, sizeof(container.body));

    if (r.encoded >= 0) {
        container.size = (r.encoded + 7) / 8;

#ifdef CRH_DEBUG_PRINT_ENCODED_DATA
        for (int i = 0; i < container.size; ++i) {
            printf("%02x", container.body[i]);
        }
        printf("\n");
#endif

        return cr_send_message(connection, &container);
    } else {
#ifndef CRH_NO_PRINT_ERROR_DETAILS
        if (r.failed_type) {
            fprintf(stderr, "Encoding failed because of a constraint violation in: %s\n", r.failed_type->name);
            fflush(stderr);
        } else {
            fprintf(stderr, "No failed_type details for encoding error\n");
            fflush(stderr);
        }
#endif
        return CR_RESULT_ERR_UPER_ENCODING_FAILED;
    }
}


inline static CrResult crh_decode_uper_from_message_ref(const CrMessageRef *msg, void **message) {
    asn_TYPE_descriptor_t *descriptor = crh_get_asn_type_descriptor_for_message_id(msg->id);

    if (message == NULL) {
        return CR_RESULT_ERR_PARAMETER_MESSAGE_IS_NULL;
    } else if (descriptor == NULL) {
        return CR_RESULT_ERR_PARAMETER_MESSAGE_ID_IS_INVALID;
    }

    asn_dec_rval_t r = uper_decode(0, descriptor, message, msg->body, msg->size, 0, 0);

    if (r.code == RC_OK) {
        return CR_RESULT_OK;
    } else {
        return CR_RESULT_ERR_UPER_DECODING_FAILED;
    }
}

inline static CrResult crh_receive(CrConnection *connection, CrMessageIdOverloaded *filter, void **target, uint64_t timeout_ms) {
    return cr_receive_message_by_ref(connection, filter, crh_decode_uper_from_message_ref, target, timeout_ms);
}

inline static bool crh_free(CrMessageId mid, void **message) {
    asn_TYPE_descriptor_t *descriptor = crh_get_asn_type_descriptor_for_message_id(mid);
    if (descriptor != NULL) {
        descriptor->op->free_struct(descriptor, *message, ASFM_FREE_EVERYTHING);
        *message = NULL;
        return true;
    } else {
        return false;
    }
}

inline static CrResult crh_print(CrMessageId mid, void *message, FILE *stream) {
    asn_TYPE_descriptor_t *descriptor = crh_get_asn_type_descriptor_for_message_id(mid);
    if (descriptor != NULL) {
        if (asn_fprint(stream, descriptor, message) == 0) {
            return CR_RESULT_OK;
        }
    }
    return CR_RESULT_ERR_TEXT_ENCODING_FAILED;
}

inline static CrResult crh_check(CrMessageId mid, void *message, FILE *stream) {
    asn_TYPE_descriptor_t *descriptor = crh_get_asn_type_descriptor_for_message_id(mid);

    if (descriptor != NULL) {
        char buffer[CRH_ERROR_BUFFER_SIZE];
        size_t buffer_len = sizeof(buffer);
        if (asn_check_constraints(descriptor, message, buffer, &buffer_len) != 0) {
            fprintf(stream, "%s\n", buffer);
            fflush(stream);
        } else {
            return CR_RESULT_OK;
        }
    }
    return CR_RESULT_ERR_CONSTRAINT_CHECK_VIOLATION_FOUND;
}

inline static CrMessageRef crh_message_to_ref(CrMessage* message) {
    return (CrMessageRef) {
            .id = message->id,
            .size = message->size,
            .body = (const uint8_t*) &(message->body),
    };
}




#define create_helper_functions(type_cc, type_sc, type_ssc) \
\
    inline static type_cc##_t *crh_alloc_##type_sc() {          \
        return calloc(1, sizeof(type_cc##_t));    \
    } \
\
    inline static bool crh_free_##type_sc(type_cc##_t **value) { \
        return crh_free(CR_MESSAGE_ID_##type_ssc, (void **) value); \
    } \
\
    inline static CrResult crh_print_##type_sc(type_cc##_t *value, FILE *stream) { \
        return crh_print(CR_MESSAGE_ID_##type_ssc, (void *) value, stream); \
    } \
\
    inline static CrResult crh_check_##type_sc(type_cc##_t *value, FILE *stream) { \
        return crh_check(CR_MESSAGE_ID_##type_ssc, (void *) value, stream); \
    } \
\
    inline static CrResult crh_send_##type_sc(CrConnection *connection, type_cc##_t *value) { \
        return crh_send(connection, CR_MESSAGE_ID_##type_ssc, value); \
    } \
\
    inline static CrResult crh_receive_##type_sc(CrConnection *connection, type_cc##_t **target, uint64_t timeout_ms) { \
        CrMessageIdOverloaded mid = CR_MESSAGE_ID_##type_ssc; \
        CrResult result = crh_receive(connection, &mid, (void **) target, timeout_ms); \
        assert(mid == CR_MESSAGE_ID_##type_ssc); \
        return result; \
    } \
\
    inline static CrResult crh_##type_sc##_to_json_pretty(type_cc##_t* src, char* target, size_t target_len) { \
        CrMessage message = {.id = CR_MESSAGE_ID_##type_ssc}; \
        asn_TYPE_descriptor_t *descriptor = crh_get_asn_type_descriptor_for_message_id(message.id); \
        asn_enc_rval_t r = uper_encode_to_buffer(descriptor, NULL, src, message.body, sizeof(message.body)); \
\
        if (r.encoded >= 0) { \
            message.size = (r.encoded + 7) / 8; \
            return cr_message_uper_to_json_pretty(crh_message_to_ref(&message), target, target_len); \
        } else { \
            return CR_RESULT_ERR_UPER_DECODING_FAILED; \
        } \
    } \
\
    inline static CrResult crh_##type_sc##_convert_to(type_cc##_t* src, CrFormat target_format, char* target, size_t* target_len) { \
        CrMessage message = {.id = CR_MESSAGE_ID_##type_ssc}; \
        asn_TYPE_descriptor_t *descriptor = crh_get_asn_type_descriptor_for_message_id(message.id); \
        asn_enc_rval_t r = uper_encode_to_buffer(descriptor, NULL, src, message.body, sizeof(message.body)); \
\
        if (r.encoded >= 0) { \
            message.size = (r.encoded + 7) / 8; \
            return cr_message_convert(                          \
                CR_MESSAGE_ID_##type_ssc,                       \
                CR_FORMAT_UPER,                                 \
                (const char*) message.body,                     \
                message.size,                                   \
                target_format,                                  \
                target,                                         \
                target_len                                      \
            );\
        } else { \
            return CR_RESULT_ERR_UPER_DECODING_FAILED; \
        } \
    }

create_helper_functions(DENM, denm, DENM)
create_helper_functions(CPM, cpm, CPM)
create_helper_functions(CAM, cam, CAM)
create_helper_functions(VAM, vam, VAM)
create_helper_functions(MCM, mcm, MCM)

create_helper_functions(DebugRequest, debug_request, DEBUG_REQUEST)
create_helper_functions(ComponentStatus, component_status, COMPONENT_STATUS)

#endif //SAMPLE_CLIENT_CONNECTOR_HELPER_H
