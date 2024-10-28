#include <stdio.h>
#include "stepper_control.h"
#include "encoder.h"

#define SAMPLE_RATE_1KHZ (1000)
#define PULSES_PER_REV (600)
#define REF_POSITION_IN_RAD (0)
int main(){

    // Pre-loop sequence

    // Configuration
    encoder_t enc = encoder_init(SAMPLE_RATE_1KHZ,
                                 PULSES_PER_REV,
                                 REF_POSITION_IN_RAD);

    stepper_t control = stepper_init();

    // Initial condition

    while(1){
        
    }
    // Process termination
    return 0x00;
}
