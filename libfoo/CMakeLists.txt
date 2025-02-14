cmake_minimum_required(VERSION 3.25)
project(foo VERSION 0.1.0)

add_library(${PROJECT_NAME}
    foo.c
)

include(CMakePackageConfigHelpers)
include(GNUInstallDirs)

install(TARGETS ${PROJECT_NAME} DESTINATION .)