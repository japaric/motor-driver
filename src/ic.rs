//! Supported ICs (Integrated Circuits)

/// TB6612FNG, dual DC motor driver
///
/// # Connections
///
/// - IN1 = xIN1
/// - IN2 = xIN2
/// - PWM = PWMx
///
/// where x = A or B
///
/// **NOTE** The STANDBY (STBY) pin needs to be driven high
pub struct TB6612FNG;

/// L298, dual full-bridge driver
///
/// # Connections
///
/// (IN1, IN2, PWM) = (In1, In2, EnA) OR (In3, In4, EnB)
pub struct L298;
