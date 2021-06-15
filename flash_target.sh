#!/bin/sh

/home/alexis/Projets/STM32CubeProgrammer/bin/STM32_Programmer_CLI -c port=SWD ap=0 freq=16000 -w $1 0x08000000 -v -HardRst