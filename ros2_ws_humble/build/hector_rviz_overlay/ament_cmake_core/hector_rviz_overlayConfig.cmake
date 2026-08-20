# generated from ament/cmake/core/templates/nameConfig.cmake.in

# prevent multiple inclusion
if(_hector_rviz_overlay_CONFIG_INCLUDED)
  # ensure to keep the found flag the same
  if(NOT DEFINED hector_rviz_overlay_FOUND)
    # explicitly set it to FALSE, otherwise CMake will set it to TRUE
    set(hector_rviz_overlay_FOUND FALSE)
  elseif(NOT hector_rviz_overlay_FOUND)
    # use separate condition to avoid uninitialized variable warning
    set(hector_rviz_overlay_FOUND FALSE)
  endif()
  return()
endif()
set(_hector_rviz_overlay_CONFIG_INCLUDED TRUE)

# output package information
if(NOT hector_rviz_overlay_FIND_QUIETLY)
  message(STATUS "Found hector_rviz_overlay: 1.0.0 (${hector_rviz_overlay_DIR})")
endif()

# warn when using a deprecated package
if(NOT "" STREQUAL "")
  set(_msg "Package 'hector_rviz_overlay' is deprecated")
  # append custom deprecation text if available
  if(NOT "" STREQUAL "TRUE")
    set(_msg "${_msg} ()")
  endif()
  # optionally quiet the deprecation message
  if(NOT ${hector_rviz_overlay_DEPRECATED_QUIET})
    message(DEPRECATION "${_msg}")
  endif()
endif()

# flag package as ament-based to distinguish it after being find_package()-ed
set(hector_rviz_overlay_FOUND_AMENT_PACKAGE TRUE)

# include all config extra files
set(_extras "hector_rviz_overlay-extras.cmake;ament_cmake_export_dependencies-extras.cmake;ament_cmake_export_include_directories-extras.cmake;ament_cmake_export_libraries-extras.cmake;ament_cmake_export_targets-extras.cmake")
foreach(_extra ${_extras})
  include("${hector_rviz_overlay_DIR}/${_extra}")
endforeach()
