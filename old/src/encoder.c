 #include "encoder.h"


uint16_t encoder_get_angle(void)
{
 return 0;
}
encoder_t encoder_init(uint16_t sample_rate,
                       uint16_t resolution,
                       uint16_t starting_position)
{
 encoder_t new_enc;
 new_enc.sample_rate = sample_rate;
 new_enc.resolution = resolution;
 new_enc.starting_position = starting_position;
 
 return new_enc;
 }

void encoder_resetCounter(encoder_t* enc)
{
}

uint16_t encoder_getCounter(encoder_t* enc)
{
 return 0;
}

uint16_t encoder_updateSubscribers(encoder_t* enc)
{
 return 0;
}
