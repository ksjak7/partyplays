pub trait ClampAdd {
    fn clamp_add(self, value: Self) -> Self;
}

impl ClampAdd for i16 {
    fn clamp_add(self, value: Self) -> Self {
        let add_result = self.overflowing_add(value);
        if add_result.1 {
            if self < 0 {
                i16::MAX
            } else {
                i16::MIN
            }
        } else {
            add_result.0
        }
    }
}

impl ClampAdd for u8 {
    fn clamp_add(self, value: Self) -> Self {
        let add_result = self.overflowing_add(value);
        if add_result.1 {
            if self < u8::MAX / 2 {
                u8::MAX
            } else {
                u8::MIN
            }
        } else {
            add_result.0
        }
    }
}
