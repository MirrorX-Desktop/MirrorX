find_package(Corrosion REQUIRED)

corrosion_import_crate(MANIFEST_PATH ../../../../../../../mirrorx_core/Cargo.toml)

set(CRATE_NAME "mirrorx_core")

target_link_libraries(${BINARY_NAME} PUBLIC ${CRATE_NAME})

list(APPEND PLUGIN_BUNDLED_LIBRARIES $<TARGET_FILE:${CRATE_NAME}-shared>)