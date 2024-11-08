/*
 * File:   timer.c
 * Author: smgon
 *
 * Created on December 21, 2022, 7:02 PM
 */


#include <xc.h>
#include "timer.h"

// Configure Timer0
void configureTimer0(void) {
    T0CONbits.T08BIT = 1;     // Configure Timer0 as an 8-bit timer
    T0CONbits.T0CS= 0;        // Internal instruction cycle clock (CLKO)
    INTCONbits.TMR0IE = 1;    // Enable Timer0 interrupts 
}
