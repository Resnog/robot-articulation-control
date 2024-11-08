#ifndef ENCODER_H
#define	ENCODER_H

#include <stdint.h>

typedef struct{
 uint16_t sample_rate;
 uint16_t resolution;
 uint16_t starting_position;
 uint16_t noise;
 void* subscribers;
 uint8_t subs_count;
}encoder_t;

/*
 Create and initialize a new encoder
*/
encoder_t encoder_init(uint16_t sample_rate,
                       uint16_t resolution,
                       uint16_t starting_position);

/*
 Reset encoder values
*/
void encoder_resetCounter(encoder_t* enc);

/*
 Get the value of that encoder counter
*/
uint16_t encoder_getCounter(encoder_t* enc);

/*
 Update the values for the encoder subscribers
*/
uint16_t encoder_updateSubscribers(encoder_t* enc);

#endif	/* ENCODER_H */

