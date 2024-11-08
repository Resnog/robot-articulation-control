#include <stdint.h>
#include "stepper_control.h"

stepper_t stepper_init(void)
{
  stepper_t new_control;
  new_control.status = stepper_off;
  new_control.enable = 0;
  new_control.direction = 0;
  new_control.fault = 0;
  new_control.reset = 0;
  new_control.sleep = 0;
  new_control.modes = 0;
}

void stepper_set_position(stepper_t motor);

void stepper_set_velocity(stepper_t motor);

void stepper_predict_position(stepper_t motor);

void stepper_predict_velocity(stepper_t motor);

void stepper_subscribe_encoder(encoder_t enc);

void stepper_set_mode(uint8_t mode);
