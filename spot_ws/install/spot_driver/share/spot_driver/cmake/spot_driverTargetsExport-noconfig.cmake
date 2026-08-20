#----------------------------------------------------------------
# Generated CMake target import file.
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "spot_driver::image_stitcher" for configuration ""
set_property(TARGET spot_driver::image_stitcher APPEND PROPERTY IMPORTED_CONFIGURATIONS NOCONFIG)
set_target_properties(spot_driver::image_stitcher PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_NOCONFIG "CXX"
  IMPORTED_LOCATION_NOCONFIG "${_IMPORT_PREFIX}/lib/libimage_stitcher.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS spot_driver::image_stitcher )
list(APPEND _IMPORT_CHECK_FILES_FOR_spot_driver::image_stitcher "${_IMPORT_PREFIX}/lib/libimage_stitcher.a" )

# Import target "spot_driver::lease_manager_component" for configuration ""
set_property(TARGET spot_driver::lease_manager_component APPEND PROPERTY IMPORTED_CONFIGURATIONS NOCONFIG)
set_target_properties(spot_driver::lease_manager_component PROPERTIES
  IMPORTED_LOCATION_NOCONFIG "${_IMPORT_PREFIX}/lib/liblease_manager_component.so"
  IMPORTED_SONAME_NOCONFIG "liblease_manager_component.so"
  )

list(APPEND _IMPORT_CHECK_TARGETS spot_driver::lease_manager_component )
list(APPEND _IMPORT_CHECK_FILES_FOR_spot_driver::lease_manager_component "${_IMPORT_PREFIX}/lib/liblease_manager_component.so" )

# Import target "spot_driver::spot_api" for configuration ""
set_property(TARGET spot_driver::spot_api APPEND PROPERTY IMPORTED_CONFIGURATIONS NOCONFIG)
set_target_properties(spot_driver::spot_api PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_NOCONFIG "CXX"
  IMPORTED_LOCATION_NOCONFIG "${_IMPORT_PREFIX}/lib/libspot_api.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS spot_driver::spot_api )
list(APPEND _IMPORT_CHECK_FILES_FOR_spot_driver::spot_api "${_IMPORT_PREFIX}/lib/libspot_api.a" )

# Import target "spot_driver::spot_image_publisher_component" for configuration ""
set_property(TARGET spot_driver::spot_image_publisher_component APPEND PROPERTY IMPORTED_CONFIGURATIONS NOCONFIG)
set_target_properties(spot_driver::spot_image_publisher_component PROPERTIES
  IMPORTED_LOCATION_NOCONFIG "${_IMPORT_PREFIX}/lib/libspot_image_publisher_component.so"
  IMPORTED_SONAME_NOCONFIG "libspot_image_publisher_component.so"
  )

list(APPEND _IMPORT_CHECK_TARGETS spot_driver::spot_image_publisher_component )
list(APPEND _IMPORT_CHECK_FILES_FOR_spot_driver::spot_image_publisher_component "${_IMPORT_PREFIX}/lib/libspot_image_publisher_component.so" )

# Import target "spot_driver::spot_inverse_kinematics_component" for configuration ""
set_property(TARGET spot_driver::spot_inverse_kinematics_component APPEND PROPERTY IMPORTED_CONFIGURATIONS NOCONFIG)
set_target_properties(spot_driver::spot_inverse_kinematics_component PROPERTIES
  IMPORTED_LOCATION_NOCONFIG "${_IMPORT_PREFIX}/lib/libspot_inverse_kinematics_component.so"
  IMPORTED_SONAME_NOCONFIG "libspot_inverse_kinematics_component.so"
  )

list(APPEND _IMPORT_CHECK_TARGETS spot_driver::spot_inverse_kinematics_component )
list(APPEND _IMPORT_CHECK_FILES_FOR_spot_driver::spot_inverse_kinematics_component "${_IMPORT_PREFIX}/lib/libspot_inverse_kinematics_component.so" )

# Import target "spot_driver::state_publisher_component" for configuration ""
set_property(TARGET spot_driver::state_publisher_component APPEND PROPERTY IMPORTED_CONFIGURATIONS NOCONFIG)
set_target_properties(spot_driver::state_publisher_component PROPERTIES
  IMPORTED_LOCATION_NOCONFIG "${_IMPORT_PREFIX}/lib/libstate_publisher_component.so"
  IMPORTED_SONAME_NOCONFIG "libstate_publisher_component.so"
  )

list(APPEND _IMPORT_CHECK_TARGETS spot_driver::state_publisher_component )
list(APPEND _IMPORT_CHECK_FILES_FOR_spot_driver::state_publisher_component "${_IMPORT_PREFIX}/lib/libstate_publisher_component.so" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
