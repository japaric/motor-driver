//! Crate to interface full H-bridge motor drivers

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate embedded_hal;

pub mod ic;

use core::marker::PhantomData;

use embedded_hal::digital::OutputPin;
use embedded_hal::PwmPin;

/// A full H-bridge motor driver
pub struct Motor<IN1, IN2, PWM, IC>
where
    IN1: OutputPin,
    IN2: OutputPin,
    PWM: PwmPin,
{
    in1: IN1,
    in2: IN2,
    pwm: PWM,
    _ic: PhantomData<IC>,
}

impl<IN1, IN2, PWM, IC> Motor<IN1, IN2, PWM, IC>
where
    IN1: OutputPin,
    IN2: OutputPin,
    PWM: PwmPin,
{
    /// Brakes the motor
    pub fn brake(&mut self) -> &mut Self {
        self.in1.set_high();
        self.in2.set_high();
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

    /// Returns the maximum
    pub fn get_max_duty(&mut self) -> PWM::Duty {
        self.pwm.get_max_duty()
    }

    /// Changes the motor speed
    pub fn duty(&mut self, duty: PWM::Duty) -> &mut Self {
        self.pwm.set_duty(duty);
        self
    }
}

impl<IN1, IN2, PWM> Motor<IN1, IN2, PWM, ic::L298>
where
    IN1: OutputPin,
    IN2: OutputPin,
    PWM: PwmPin,
{
    /// Creates a new `Motor`
    pub fn l298(mut in1: IN1, mut in2: IN2, mut pwm: PWM) -> Self {
        // initial state: brake
        in1.set_high();
        in2.set_high();

        pwm.enable();

        Motor {
            in1,
            in2,
            pwm,
            _ic: PhantomData,
        }
    }
}

impl<IN1, IN2, PWM> Motor<IN1, IN2, PWM, ic::TB6612FNG>
where
    IN1: OutputPin,
    IN2: OutputPin,
    PWM: PwmPin,
{
    /// Creates a new `Motor`
    pub fn tb6612fng(mut in1: IN1, mut in2: IN2, mut pwm: PWM) -> Self {
        // initial state: brake
        in1.set_high();
        in2.set_high();

        pwm.enable();

        Motor {
            in1,
            in2,
            pwm,
            _ic: PhantomData,
        }
    }

    /// Lets the motor coast
    pub fn coast(&mut self) -> &mut Self {
        self.in1.set_low();
        self.in2.set_low();
        self
    }
}

/// A motor driver with phase (0/1) and enable (PWM) inputs
pub struct PhaseEnableMotor<IN, PWM, IC>
where
    IN: OutputPin,
    PWM: PwmPin,
{
    phase: IN,
    enable: PWM,
    _ic: PhantomData<IC>,
}

impl<IN, PWM, IC> PhaseEnableMotor<IN, PWM, IC>
where
    IN: OutputPin,
    PWM: PwmPin,
{
    /// Makes the motor spin in CounterClockWise direction
    pub fn ccw(&mut self) -> &mut Self {
        self.phase.set_low();
        self
    }

    /// Makes the motor spin in ClockWise direction
    pub fn cw(&mut self) -> &mut Self {
        self.phase.set_high();
        self
    }

    /// Returns the maximum
    pub fn get_max_duty(&mut self) -> PWM::Duty {
        self.enable.get_max_duty()
    }

    /// Changes the motor speed
    ///
    /// If duty is zero, the motor brakes (both motor lines are shorted to
    /// ground)
    pub fn duty(&mut self, duty: PWM::Duty) -> &mut Self {
        self.enable.set_duty(duty);
        self
    }
}

impl<IN, PWM> PhaseEnableMotor<IN, PWM, ic::DRV8835PE>
where
    IN: OutputPin,
    PWM: PwmPin,
    PWM::Duty: From<u8>,
{
    /// Creates a new `PhaseEnableMotor`
    pub fn drv8835pe(mut phase: IN, mut enable: PWM) -> Self {
        // initial state: brake, phase low
        enable.set_duty(0u8.into());
        phase.set_low();

        enable.enable();

        PhaseEnableMotor {
            phase,
            enable,
            _ic: PhantomData,
        }
    }
}