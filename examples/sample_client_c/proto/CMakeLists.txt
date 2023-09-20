file(GLOB ASN1_FILES "${CMAKE_CURRENT_SOURCE_DIR}/*.asn" "${CMAKE_CURRENT_SOURCE_DIR}/*.asn1")

#execute_process(COMMAND asn1c -fcompound-names -Werror -gen-PER ${ASN1_FILES} WORKING_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}")
#execute_process(COMMAND "${CMAKE_CURRENT_SOURCE_DIR}/../../github_riebel_asn1c/asn1c/asn1c" -S "${CMAKE_CURRENT_SOURCE_DIR}/../../github_riebel_asn1c/skeletons/" -fcompound-names -Werror -no-gen-BER -no-gen-XER -no-gen-OER -no-gen-APER ${ASN1_FILES} WORKING_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}")
#execute_process(COMMAND "${CMAKE_CURRENT_SOURCE_DIR}/../../github_mouse07410_asn1c/asn1c/asn1c" -S "${CMAKE_CURRENT_SOURCE_DIR}/../../github_mouse07410_asn1c/skeletons/" -fcompound-names -Werror -no-gen-BER -no-gen-XER -no-gen-OER -no-gen-APER ${ASN1_FILES} WORKING_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}")
#execute_process(COMMAND "${CMAKE_CURRENT_SOURCE_DIR}/../asn1c/asn1c/asn1c" -S "${CMAKE_CURRENT_SOURCE_DIR}/../asn1c/skeletons/" -fcompound-names -Werror -no-gen-BER -no-gen-XER -no-gen-OER -no-gen-APER ${ASN1_FILES} WORKING_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}")

execute_process(COMMAND "${CMAKE_CURRENT_SOURCE_DIR}/../asn1c/asn1c/asn1c" -S "${CMAKE_CURRENT_SOURCE_DIR}/../asn1c/skeletons/" -fcompound-names -Werror -gen-PER ${ASN1_FILES} WORKING_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}")

file(REMOVE "${CMAKE_CURRENT_BINARY_DIR}/converter-example.c")

file(GLOB ASN1_SRC "${CMAKE_CURRENT_BINARY_DIR}/*.c")
file(GLOB ASN1_HDR "${CMAKE_CURRENT_BINARY_DIR}/*.h")

add_library(proto STATIC ${ASN1_SRC})
target_include_directories(proto PUBLIC ${CMAKE_CURRENT_BINARY_DIR})
