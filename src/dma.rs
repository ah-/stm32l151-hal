pub trait DmaExt {
    type Channels;

    fn split(self) -> Self::Channels;
}

macro_rules! dma {
    ($($DMAX:ident: ($dmaX:ident, $dmaXen:ident, $dmaXrst:ident, {
            $($CX:ident: (
                $ccrX:ident,
                $CCRX:ident,
                $cndtrX:ident,
                $CNDTRX:ident,
                $cparX:ident,
                $CPARX:ident,
                $cmarX:ident,
                $CMARX:ident,
                $htifX:ident,
                $tcifX:ident,
                $chtifX:ident,
                $ctcifX:ident,
                $cgifX:ident
            ),)+
    }),)+) => {
        $(
            pub mod $dmaX {
                use stm32l151::{$DMAX, dma1};
                use super::DmaExt;

                pub struct Channels((), $(pub $CX),+);

                $(
                    pub struct $CX { _0: () }

                    impl $CX {
                        pub fn tcif(&self) -> bool {
                            unsafe { (*$DMAX::ptr()).isr.read().$tcifX().bit_is_set() }
                        }

                        pub fn htif(&self) -> bool {
                            unsafe { (*$DMAX::ptr()).isr.read().$htifX().bit_is_set() }
                        }

                        pub fn ctcif(&mut self) {
                            unsafe { (*$DMAX::ptr()).ifcr.write(|w| w.$ctcifX().set_bit()) }
                        }

                        pub fn chtif(&mut self) {
                            unsafe { (*$DMAX::ptr()).ifcr.write(|w| w.$chtifX().set_bit()) }
                        }

                        pub fn cgif(&mut self) {
                            unsafe { (*$DMAX::ptr()).ifcr.write(|w| w.$cgifX().set_bit()) }
                        }

                        pub fn ccr(&mut self) -> &dma1::$CCRX {
                            unsafe { &(*$DMAX::ptr()).$ccrX }
                        }

                        pub fn cndtr(&mut self) -> &dma1::$CNDTRX {
                            unsafe { &(*$DMAX::ptr()).$cndtrX }
                        }

                        pub fn cpar(&mut self) -> &dma1::$CPARX {
                            unsafe { &(*$DMAX::ptr()).$cparX }
                        }

                        pub fn cmar(&mut self) -> &dma1::$CMARX {
                            unsafe { &(*$DMAX::ptr()).$cmarX }
                        }
                    }
                )+

                impl DmaExt for $DMAX {
                    type Channels = Channels;

                    fn split(self) -> Channels {
                        Channels((), $($CX { _0: () }),+)
                    }
                }
            }
        )+
    }
}



dma! {
    DMA1: (dma1, dma1en, dma1rst, {
        C1: (
            ccr1, CCR1,
            cndtr1, CNDTR1,
            cpar1, CPAR1,
            cmar1, CMAR1,
            htif1, tcif1,
            chtif1, ctcif1, cgif1
        ),
        C2: (
            ccr2, CCR2,
            cndtr2, CNDTR2,
            cpar2, CPAR2,
            cmar2, CMAR2,
            htif2, tcif2,
            chtif2, ctcif2, cgif2
        ),
        C3: (
            ccr3, CCR3,
            cndtr3, CNDTR3,
            cpar3, CPAR3,
            cmar3, CMAR3,
            htif3, tcif3,
            chtif3, ctcif3, cgif3
        ),
        C4: (
            ccr4, CCR4,
            cndtr4, CNDTR4,
            cpar4, CPAR4,
            cmar4, CMAR4,
            htif4, tcif4,
            chtif4, ctcif4, cgif4
        ),
        C5: (
            ccr5, CCR5,
            cndtr5, CNDTR5,
            cpar5, CPAR5,
            cmar5, CMAR5,
            htif5, tcif5,
            chtif5, ctcif5, cgif5
        ),
        C6: (
            ccr6, CCR6,
            cndtr6, CNDTR6,
            cpar6, CPAR6,
            cmar6, CMAR6,
            htif6, tcif6,
            chtif6, ctcif6, cgif6
        ),
        C7: (
            ccr7, CCR7,
            cndtr7, CNDTR7,
            cpar7, CPAR7,
            cmar7, CMAR7,
            htif7, tcif7,
            chtif7, ctcif7, cgif7
        ),
    }),
}
