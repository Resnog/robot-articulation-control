/*
 * File:   main.c
 * Author: smgon
 *
 * Created on December 5, 2022, 8:11 PM
 */

#pragma config WDT = OFF // Set watchdog timer off

#include <xc.h>
#include "stepper_motor_driver.h"

motor_control_t drv8825;

void setup(void) {
// Configure PortA with 4 outputs for interacting with the stepper motor driver
TRISAbits.TRISA0 = 0;
TRISAbits.TRISA1 = 0;
TRISAbits.TRISA2 = 0;
TRISAbits.TRISA3 = 0;
drv8825.enable = 1;
// Set clock 
// Set PWM
}

void main(void) {
    
// Main loop

    
    return;
}
