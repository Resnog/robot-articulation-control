# PIC Controllers

I have only had the pleasure of using some 8bit PIC I forgot somewhere and some PIC18F4550 MCUs. I do have the
intention to eventually buy some newer PICs since they look interesting. But time will tell.

This stepper motor control is based on a PIC18F4550 I have lying around since college and wanted to do something
interesting from scratch with them, currently the code is under design. The following features are to be added into the
project as it develops:

- UART/USART serial communication with another device to receive commands.
- Rotary encoder integration for creating a feedback loop.
- Motor speed control.
- Motor position control.
- SPI communication to communicate to a master what the motor status is.

I have no external oscillator and have yet to experiment with the PLL oscillator as the main system clock, so for the
time being, the internal oscillator of 31kHz is the one used in this project.
