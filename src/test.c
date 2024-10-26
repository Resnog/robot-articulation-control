#include <stdio.h>
#include "stepper_control.h"
#include "encoder.h"

int main(){

    // Pre-loop sequence

    // Configuration
    encoder_t enc = encoder_init(1000,
                                 600,
                                 0);

    stepper_t control = stepper_init();

    // Initial condition

    while(1){
        
    }
    // Process termination
    return 0x00;
}
