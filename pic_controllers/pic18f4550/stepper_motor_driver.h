/* 
 * File:   stepper_motor_driver.h
 * Author: smgon
 *
 * Created on December 17, 2022, 4:59 PM
 */

#include <xc.h>

#ifndef STEPPER_MOTOR_DRIVER_H
#define	STEPPER_MOTOR_DRIVER_H

// Motor driver for TI DRV8825 chip
typedef struct{
    uint8_t modes;
    uint8_t enable;
    uint8_t direction;
    uint8_t sleep;
    uint8_t reset;
    uint8_t fault;
}motor_control_t;

enum drv8825_step_modes {
    full_step           = 0,
    half_step           = 1,
    quarter_step        = 2,
    eighth_step         = 3,
    sixteenth_step      = 4,
    thrirthy_two_step   = 5,
};

/*
 Set position for the stepper motor
 */
void set_position(float angle);

/*
 Set nominal velocity for the stepper motor
 */
void set_velocity(float speed);

/*
 Send one step to the IC controller
 */
void send_step(void);

/*
 Set motor to default position 
 */
void reset(void);

/*
 Configure stepper driver
 */
void init_stepper_driver(motor_control_t driver);

/*
 Set DRV8825 stepping mode
 */
void set_mode(void);


#endif	/* STEPPER_MOTOR_DRIVER_H */

