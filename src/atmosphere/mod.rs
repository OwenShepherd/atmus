mod constants;

pub fn get_pressure(height: f32) -> f32 {
    let p0 = constants::P0;
    let lmb0 = constants::TEMPERATURE_HEIGHT_GRADIENT[0].1;
    let h0 = constants::TEMPERATURE_HEIGHT_GRADIENT[0].0;
    let mut temp_l = lmb0;
    let mut temp_hb = h0;
    let mut temp_p = p0;
    let mut temp_t = constants::T0;
    for i in 1..constants::NUM_GRADIENTS {
        let (hb, lmb) = constants::TEMPERATURE_HEIGHT_GRADIENT[i];
        if height < hb {
            return pressure(temp_t, temp_l, height, temp_hb, temp_p);
        } else {
            let temp_tt = temp_t;
            temp_t = temperature(temp_t, temp_l, hb, temp_hb);
            temp_p = pressure(temp_tt, temp_l, hb, temp_hb, temp_p);
            temp_l = lmb;
            temp_hb = hb;
        }
    }
    return 0.0;
}

pub fn pressure(tmb: f32, lmb: f32, h: f32, hb: f32, pb: f32) -> f32 {
    if lmb == 0.0 {
        return pb*(f32::exp((-1.0 * constants::G0 * constants::M0 * (h-hb))/(constants::RSTAR*tmb)));
    } else {
        println!("tmb: {}\nlmb: {}\nh: {}\nhb: {}\npb: {}\nG0: {}\nM0: {}\nRSTAR: {}",tmb,lmb,h,hb,pb,constants::G0,constants::M0,constants::RSTAR);
        return pb*((tmb/(tmb+(lmb*(h-hb))))).powf((constants::G0*constants::M0)/(constants::RSTAR*lmb));
    }
}

pub fn get_temperature(h: f32) -> f32 {
    // Tm = Tmb + Lmb * (H - Hb) Equation 23 from NASA Document ID 19770009539
    let h0 = constants::TEMPERATURE_HEIGHT_GRADIENT[0].0;
    let lmb0 = constants::TEMPERATURE_HEIGHT_GRADIENT[0].1;
    let mut temp_t = constants::T0;
    let mut temp_l = lmb0;
    let mut temp_hb = h0;
    for i in 1..constants::NUM_GRADIENTS {
        let (hb, lmb) = constants::TEMPERATURE_HEIGHT_GRADIENT[i];
        if h < hb {
            return temperature(temp_t, temp_l, h, temp_hb);
        } else {
            temp_t = temperature(temp_t, temp_l, hb, temp_hb);
            temp_l = lmb;
            temp_hb = hb;
        }
    }
    return 0.0;
}

fn temperature(tmb: f32, lmb: f32, h: f32, hb: f32) -> f32 {
    tmb + (lmb * (h-hb))
}

#[cfg(test)]
mod tests {
    use crate::atmosphere::get_pressure;
    #[test]
    fn pressure_at_3km() {
        let some_result = get_pressure(3000.0);
        assert_eq!(some_result, 1.0);
    }
}
