cmake_minimum_required(VERSION 3.25)

project(MotorControl VERSION 0.1
    DESCRIPTION "Motor Control for MCUs"
                     LANGUAGES C)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_library(motorcontrol SHARED 
            src/encoder.c
            src/stepper_control.c)

target_include_directories(motorcontrol PUBLIC inc/)

add_executable(utest src/test.c)
target_link_libraries(utest PUBLIC motorcontrol)

add_custom_target(run
    COMMAND utest
    WORKING_DIRECTORY ${CMAKE_PROJECT_DIR}
)
