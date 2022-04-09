include_directories(include)

if (APPLE)
aux_source_directory(macos DUPLICATOR_SOURCE)
elseif (MSVC)
aux_source_directory(windows DUPLICATOR_SOURCE)
endif()