//! Crate to interface the TB6612FNG, a dual H-bridge motor driver

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate embedded_hal;

use embedded_hal::digital::OutputPin;
use embedded_hal::PwmPin;

/// A single H-bridge
pub struct Motor<IN1, IN2, PWM> {
    in1: IN1,
    in2: IN2,
    pwm: PWM,
}

impl<IN1, IN2, PWM> Motor<IN1, IN2, PWM>
where
    IN1: OutputPin,
    IN2: OutputPin,
    PWM: PwmPin,
{
    /// Creates a new `Motor`
    pub fn new(mut in1: IN1, mut in2: IN2, mut pwm: PWM) -> Self {
        // initial state: brake
        in1.set_high();
        in2.set_high();

        pwm.enable();

        Motor { in1, in2, pwm }
    }

    /// Brakes the motor
    pub fn brake(&mut self) -> &mut Self {
        self.in1.set_high();
        self.in2.set_high();
        self
    }

    /// Lets the motor coast
    pub fn coast(&mut self) -> &mut Self {
        self.in1.set_low();
        self.in2.set_low();
        self
    }

    /// Makes the motor spin in CounterClockWise direction
    pub fn ccw(&mut self) -> &mut Self {
        self.in1.set_low();
        self.in2.set_high();
        self
    }

    /// Makes the motor spin in ClockWise direction
    pub fn cw(&mut self) -> &mut Self {
        self.in1.set_high();
        self.in2.set_low();
        self
    }

    /// Changes the motor speed
    pub fn speed(&mut self, duty: PWM::Duty) -> &mut Self {
        self.pwm.set_duty(duty);
        self
    }
}
