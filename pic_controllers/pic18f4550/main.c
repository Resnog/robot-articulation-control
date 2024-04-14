/*
 * File:   main.c
 * Author: smgon
 *
 * Created on December 5, 2022, 8:11 PM
 */


#include <xc.h>
#include "main.h"
#include "stepper_motor_driver.h"
#include "timer.h"

motor_control_t drv8825;

void setup(void) {
    
    init_stepper_driver(drv8825);
    configureTimer0();

}

void main(void) {
    
// Main loop

    
    return;
}
