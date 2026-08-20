#----------------------------------------------------------------
# Generated CMake target import file.
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "hector_rviz_overlay::hector_rviz_overlay" for configuration ""
set_property(TARGET hector_rviz_overlay::hector_rviz_overlay APPEND PROPERTY IMPORTED_CONFIGURATIONS NOCONFIG)
set_target_properties(hector_rviz_overlay::hector_rviz_overlay PROPERTIES
  IMPORTED_LOCATION_NOCONFIG "${_IMPORT_PREFIX}/lib/libhector_rviz_overlay.so"
  IMPORTED_SONAME_NOCONFIG "libhector_rviz_overlay.so"
  )

list(APPEND _IMPORT_CHECK_TARGETS hector_rviz_overlay::hector_rviz_overlay )
list(APPEND _IMPORT_CHECK_FILES_FOR_hector_rviz_overlay::hector_rviz_overlay "${_IMPORT_PREFIX}/lib/libhector_rviz_overlay.so" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
