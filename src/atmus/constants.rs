// Constants taken from reference, page 2.
pub const RSTAR: f32 = 8.31432; // [N*m/kmol/K]; K, The gas constant.
pub const P0: f32 = 101325.0; // [N/m/m]; P_0, Pressure at sea level.
pub const G0: f32 = 9.80665; // [m/s/s]; g_0, Acceleration due to gravity at sea-level.
pub const T0: f32 = 288.15; // [K] T_0, Temperature at sea level.

// Constants taken from reference, table 4.
pub const NUM_GRADIENTS: usize = 8;
pub const TEMPERATURE_HEIGHT_GRADIENT: [(f32, f32); NUM_GRADIENTS] =
    [(0.0, -6.5),
     (11.0, 0.0),
     (20.0, 1.0),
     (32.0, 2.8),
     (47.0, 0.0),
     (51.0, -2.8),
     (71.0, -2.0),
     (84.8520, 0.0)]; // Hb [km'], Lmb [K/km]

// Molecular weight of the atmosphere, from reference table 3.
pub const M0: f32 =
    28.0134*0.78084 +
    31.9988*0.209476 +
    39.948*0.00934 +
    44.00995*0.000314 +
    20.183*0.00001818 +
    4.0026*0.00000524 +
    83.80*0.00000114 +
    131.30*0.000000087 +
    16.04303*0.000002 +
    2.01594*0.0000005;
