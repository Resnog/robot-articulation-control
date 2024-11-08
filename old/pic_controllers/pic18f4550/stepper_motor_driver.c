/*
 * File:   stepper_motor_driver.c
 * Author: smgon
 *
 * Created on December 18, 2022, 2:39 PM
 */

#include <xc.h>
#include "stepper_motor_driver.h"


// TODO
// send_step;
// set_velocity
// set_position
// reset

void init_stepper_driver(motor_control_t driver) {
    driver.enable = 0xFF;
    driver.modes = full_step;
    driver.direction = 0x00;
}