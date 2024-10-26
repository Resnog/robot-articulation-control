#ifndef STEPPER_MOTOR_CONTROL_H
#define	STEPPER_MOTOR_CONTROL_H

#include <stdint.h>
#include "encoder.h"

typedef enum 
{
 stepper_off,
 stepper_idle,
 stepper_position_control,
 stepper_velocity_control,
 stepper_error,
} stepper_status_t;

// General abstracion for a stepper motor driver
typedef struct{
    stepper_status_t status;
    uint8_t modes;
    uint8_t enable;
    uint8_t direction;
    uint8_t sleep;
    uint8_t reset;
    uint8_t fault;
    encoder_t enc*;
}stepper_t;

/*
 Init stepper controller
 */
stepper_t stepper_init(void);

/*
 Set the target position for the controller
 */
void stepper_set_position(stepper_t motor);

/*
 Set the target velocity for the controller
 */
void stepper_set_velocity(stepper_t motor);

/*
 Predict the estimated position
 */
void stepper_predict_position(stepper_t motor);

/*
 Predict the estimated velocity
 */
void stepper_predict_velocity(stepper_t motor);

/*
 Predict the estimated velocity
 */
void stepper_subscribe_encoder(encoder_t enc);

/*
 Set the IC step modes, depending of the specified precision of the movement
 */
void stepper_set_mode(uint8_t mode);

#endif	/* STEPPER_MOTOR_CONTROL_H */

