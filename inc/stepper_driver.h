/* 
 * File:   stepper_motor_driver.h
 * Author: smgon
 *
 * Created on December 17, 2022, 4:59 PM
 */


#ifndef STEPPER_MOTOR_DRIVER_H
#define	STEPPER_MOTOR_DRIVER_H

#include <stdint.h>

enum stepper_status_t
{
 stepper_off,
 stepper_idle,
 stepper_position_control,
 stepper_velocity_control,
 stepper_error,
}

// General abstracion for a stepper motor driver
typedef struct{
    stepper_status_t status;
    uint8_t modes;
    uint8_t enable;
    uint8_t direction;
    uint8_t sleep;
    uint8_t reset;
    uint8_t fault;
}stepper_t;

/*
 Send one step to the IC controller
 */
void stepper_step(stepper_t motor));

/*
 Reset option for IC controller
 */
void stepper_reset(stepper_t motor);

/*
 Configure IC controller
 */
void stepper_config(stepper_t motor);

/*
 Set  stepping mode
 */
void stepper_set_mode(uint8_t mode);

#endif	/* STEPPER_MOTOR_DRIVER_H */

