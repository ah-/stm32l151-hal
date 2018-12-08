pub trait GpioExt {
    type Pins;

    fn split(self) -> Self::Pins;
}

pub struct Input;
pub struct Output;
pub struct Alternate;

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $gpioy:ident, $iopxenr:ident, $iopxrst:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty, $afr:ident),)+
    ]) => {
        pub mod $gpiox {
            use core::marker::PhantomData;

            use crate::hal::digital::{InputPin, OutputPin};
            use stm32l1::stm32l151::$GPIOX;

            use super::{Alternate, GpioExt, Input, Output};

            /// GPIO parts
            pub struct Pins {
                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl GpioExt for $GPIOX {
                type Pins = Pins;

                fn split(self) -> Pins {
                    Pins {
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    pub fn into_input(self) -> $PXi<Input> {
                        let offset = (2 * $i) % 32;
                        let input = 0b00;

                        unsafe {
                            &(*$GPIOX::ptr()).moder.modify(|r, w|
                                w.bits((r.bits() & !(0b11 << offset)) | (input << offset))
                            );
                        }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_output(self) -> $PXi<Output> {
                        let offset = (2 * $i) % 32;
                        let output = 0b01;

                        unsafe {
                            &(*$GPIOX::ptr()).moder.modify(|r, w|
                                w.bits((r.bits() & !(0b11 << offset)) | (output << offset))
                            );
                        }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_alternate(self, function: u8) -> $PXi<Alternate> {
                        let offset = (2 * $i) % 32;
                        let afr_offset = (4 * $i) % 32;
                        let alternate = 0b10;

                        unsafe {
                            &(*$GPIOX::ptr()).moder.modify(|r, w|
                                w.bits((r.bits() & !(0b11 << offset)) | (alternate << offset))
                            );

                            &(*$GPIOX::ptr()).$afr.modify(|r, w|
                                w.bits((r.bits() & !(0b1111 << afr_offset)) | (u32::from(function & 0b1111) << afr_offset))
                            );
                        }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn pull_up(self) -> Self {
                        let offset = (2 * $i) % 32;
                        let pullup = 0b01;

                        unsafe {
                            &(*$GPIOX::ptr()).pupdr.modify(|r, w|
                                w.bits((r.bits() & !(0b11 << offset)) | (pullup << offset))
                            );
                        }

                        self
                    }

                    pub fn pull_down(self) -> Self {
                        let offset = (2 * $i) % 32;
                        let pulldown = 0b10;

                        unsafe {
                            &(*$GPIOX::ptr()).pupdr.modify(|r, w|
                                w.bits((r.bits() & !(0b11 << offset)) | (pulldown << offset))
                            );
                        }

                        self
                    }
                }

                impl InputPin for $PXi<Input> {
                    fn is_high(&self) -> bool {
                        !self.is_low()
                    }

                    fn is_low(&self) -> bool {
                        unsafe { (*$GPIOX::ptr()).idr.read().bits() & (1 << $i) == 0 }
                    }
                }

                impl OutputPin for $PXi<Output> {
                    fn set_high(&mut self) {
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << $i)) }
                    }

                    fn set_low(&mut self) {
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << (16 + $i))) }
                    }
                }
            )+
        }
    }
}

gpio!(GPIOA, gpioa, gpioa, gpiopaen, gpioparst, [
    PA0: (pa0, 0, Input, afrl),
    PA1: (pa1, 1, Input, afrl),
    PA2: (pa2, 2, Input, afrl),
    PA3: (pa3, 3, Input, afrl),
    PA4: (pa4, 4, Input, afrl),
    PA5: (pa5, 5, Input, afrl),
    PA6: (pa6, 6, Input, afrl),
    PA7: (pa7, 7, Input, afrl),
    PA8: (pa8, 8, Input, afrh),
    PA9: (pa9, 9, Input, afrh),
    PA10: (pa10, 10, Input, afrh),
    PA11: (pa11, 11, Input, afrh),
    PA12: (pa12, 12, Input, afrh),
    PA13: (pa13, 13, Input, afrh),
    PA14: (pa14, 14, Input, afrh),
    PA15: (pa15, 15, Input, afrh),
]);

gpio!(GPIOB, gpiob, gpioa, gpiopben, gpiopbrst, [
    PB0: (pb0, 0, Input, afrl),
    PB1: (pb1, 1, Input, afrl),
    PB2: (pb2, 2, Input, afrl),
    PB3: (pb3, 3, Input, afrl),
    PB4: (pb4, 4, Input, afrl),
    PB5: (pb5, 5, Input, afrl),
    PB6: (pb6, 6, Input, afrl),
    PB7: (pb7, 7, Input, afrl),
    PB8: (pb8, 8, Input, afrh),
    PB9: (pb9, 9, Input, afrh),
    PB10: (pb10, 10, Input, afrh),
    PB11: (pb11, 11, Input, afrh),
    PB12: (pb12, 12, Input, afrh),
    PB13: (pb13, 13, Input, afrh),
    PB14: (pb14, 14, Input, afrh),
    PB15: (pb15, 15, Input, afrh),
]);

gpio!(GPIOC, gpioc, gpioa, gpiopcen, gpiopcrst, [
    PC13: (pc13, 13, Input, afrh),
    PC14: (pc14, 14, Input, afrh),
    PC15: (pc15, 15, Input, afrh),
]);
