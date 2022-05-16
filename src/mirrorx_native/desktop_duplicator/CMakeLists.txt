list(APPEND SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/desktop_duplicator.h ${CMAKE_CURRENT_SOURCE_DIR}/desktop_duplicator_callback.h)

if (WIN32 OR MSVC)
    aux_source_directory(${CMAKE_CURRENT_SOURCE_DIR}/windows CAPTURE_SESSION_SOURCE)
elseif (APPLE)
    aux_source_directory(${CMAKE_CURRENT_SOURCE_DIR}/macos CAPTURE_SESSION_SOURCE)
endif ()

list(APPEND SOURCES ${CAPTURE_SESSION_SOURCE})
set(SOURCES ${SOURCES} PARENT_SCOPE)