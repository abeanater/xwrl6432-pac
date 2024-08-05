#[doc = "Register `WIC_CTRL` reader"]
pub type R = crate::R<WicCtrlSpec>;
#[doc = "Register `WIC_CTRL` writer"]
pub type W = crate::W<WicCtrlSpec>;
#[doc = "Field `wicmask` reader - 31:0\\]
1 => The corresponding interrupt is Masked (interrupt will not be generated) 0 => The corresponding interrupt is UnMasked (interrupt will be generated) 0 : ESM_HI_IRQ (NMI) 1 : ESM_LO_IRQ (INT#1) 2 : FECSS_FRAMETIMER_FRAME_START (INT#33) 3 : MUXED_FECSS_FRAME_START_OFFSET_INTR_TIME1 (INT#35) 4 : FECSS_FRAME_START_OFFSET_INTR_TIME2 (INT#36) 5 : FECSS_FRAME_START_OFFSET_INTR_TIME3 (INT#37) 6 : FECSS_BURST_START_OFFSET_TIME(INT#38) 7 : MUXED_APPSS_RTI1_RTI2_INT_REQ0(INT#43) 8 : MUXED_APPSS_RTI1_RTI2_INT_REQ1(INT#44) 9 : MUXED_APPSS_RTI1_RTI2_INT_REQ2(INT#45) 10 : MUXED_APPSS_RTI1_RTI2_INT_REQ3(INT#46) 11 : APPSS_SPI_IRQ_REQ(INT#14) 12 : SPI2_IRQ_REQ (part of INT#28) 13 : APPSS_LIN_INT0 (INT#10) 14 : APPSS_LIN_INT0 (INT#11) 15 : APPSS_MCAN_INT0(INT#21) 16 : APPSS_MCAN_INT1(INT#22) 17 : APPSS_SCI2_INT0(INT#62) 18 : APPSS_SCI2_INT0(INT#63) 19 : APPSS_SPI_IRQ_REQ(INT#14) 20 : SPI2_IRQ_REQ (part of INT#28) 21 : APPSS_LIN_INT0 (INT#10) 22 : APPSS_LIN_INT0 (INT#11) 23 : APPSS_MCAN_INT0(INT#21) 24 : APPSS_MCAN_INT1(INT#22) 25 : APPSS_SCI2_INT0(INT#62) 26 : APPSS_SCI2_INT0(INT#63) 27 : SYNC_IN 28 : RADAR_DEVICESLEEP_WAKEUP_INTERRUPT 29 to 31 : Reserved"]
pub type WicmaskR = crate::FieldReader<u32>;
#[doc = "Field `wicmask` writer - 31:0\\]
1 => The corresponding interrupt is Masked (interrupt will not be generated) 0 => The corresponding interrupt is UnMasked (interrupt will be generated) 0 : ESM_HI_IRQ (NMI) 1 : ESM_LO_IRQ (INT#1) 2 : FECSS_FRAMETIMER_FRAME_START (INT#33) 3 : MUXED_FECSS_FRAME_START_OFFSET_INTR_TIME1 (INT#35) 4 : FECSS_FRAME_START_OFFSET_INTR_TIME2 (INT#36) 5 : FECSS_FRAME_START_OFFSET_INTR_TIME3 (INT#37) 6 : FECSS_BURST_START_OFFSET_TIME(INT#38) 7 : MUXED_APPSS_RTI1_RTI2_INT_REQ0(INT#43) 8 : MUXED_APPSS_RTI1_RTI2_INT_REQ1(INT#44) 9 : MUXED_APPSS_RTI1_RTI2_INT_REQ2(INT#45) 10 : MUXED_APPSS_RTI1_RTI2_INT_REQ3(INT#46) 11 : APPSS_SPI_IRQ_REQ(INT#14) 12 : SPI2_IRQ_REQ (part of INT#28) 13 : APPSS_LIN_INT0 (INT#10) 14 : APPSS_LIN_INT0 (INT#11) 15 : APPSS_MCAN_INT0(INT#21) 16 : APPSS_MCAN_INT1(INT#22) 17 : APPSS_SCI2_INT0(INT#62) 18 : APPSS_SCI2_INT0(INT#63) 19 : APPSS_SPI_IRQ_REQ(INT#14) 20 : SPI2_IRQ_REQ (part of INT#28) 21 : APPSS_LIN_INT0 (INT#10) 22 : APPSS_LIN_INT0 (INT#11) 23 : APPSS_MCAN_INT0(INT#21) 24 : APPSS_MCAN_INT1(INT#22) 25 : APPSS_SCI2_INT0(INT#62) 26 : APPSS_SCI2_INT0(INT#63) 27 : SYNC_IN 28 : RADAR_DEVICESLEEP_WAKEUP_INTERRUPT 29 to 31 : Reserved"]
pub type WicmaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
1 => The corresponding interrupt is Masked (interrupt will not be generated) 0 => The corresponding interrupt is UnMasked (interrupt will be generated) 0 : ESM_HI_IRQ (NMI) 1 : ESM_LO_IRQ (INT#1) 2 : FECSS_FRAMETIMER_FRAME_START (INT#33) 3 : MUXED_FECSS_FRAME_START_OFFSET_INTR_TIME1 (INT#35) 4 : FECSS_FRAME_START_OFFSET_INTR_TIME2 (INT#36) 5 : FECSS_FRAME_START_OFFSET_INTR_TIME3 (INT#37) 6 : FECSS_BURST_START_OFFSET_TIME(INT#38) 7 : MUXED_APPSS_RTI1_RTI2_INT_REQ0(INT#43) 8 : MUXED_APPSS_RTI1_RTI2_INT_REQ1(INT#44) 9 : MUXED_APPSS_RTI1_RTI2_INT_REQ2(INT#45) 10 : MUXED_APPSS_RTI1_RTI2_INT_REQ3(INT#46) 11 : APPSS_SPI_IRQ_REQ(INT#14) 12 : SPI2_IRQ_REQ (part of INT#28) 13 : APPSS_LIN_INT0 (INT#10) 14 : APPSS_LIN_INT0 (INT#11) 15 : APPSS_MCAN_INT0(INT#21) 16 : APPSS_MCAN_INT1(INT#22) 17 : APPSS_SCI2_INT0(INT#62) 18 : APPSS_SCI2_INT0(INT#63) 19 : APPSS_SPI_IRQ_REQ(INT#14) 20 : SPI2_IRQ_REQ (part of INT#28) 21 : APPSS_LIN_INT0 (INT#10) 22 : APPSS_LIN_INT0 (INT#11) 23 : APPSS_MCAN_INT0(INT#21) 24 : APPSS_MCAN_INT1(INT#22) 25 : APPSS_SCI2_INT0(INT#62) 26 : APPSS_SCI2_INT0(INT#63) 27 : SYNC_IN 28 : RADAR_DEVICESLEEP_WAKEUP_INTERRUPT 29 to 31 : Reserved"]
    #[inline(always)]
    pub fn wicmask(&self) -> WicmaskR {
        WicmaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
1 => The corresponding interrupt is Masked (interrupt will not be generated) 0 => The corresponding interrupt is UnMasked (interrupt will be generated) 0 : ESM_HI_IRQ (NMI) 1 : ESM_LO_IRQ (INT#1) 2 : FECSS_FRAMETIMER_FRAME_START (INT#33) 3 : MUXED_FECSS_FRAME_START_OFFSET_INTR_TIME1 (INT#35) 4 : FECSS_FRAME_START_OFFSET_INTR_TIME2 (INT#36) 5 : FECSS_FRAME_START_OFFSET_INTR_TIME3 (INT#37) 6 : FECSS_BURST_START_OFFSET_TIME(INT#38) 7 : MUXED_APPSS_RTI1_RTI2_INT_REQ0(INT#43) 8 : MUXED_APPSS_RTI1_RTI2_INT_REQ1(INT#44) 9 : MUXED_APPSS_RTI1_RTI2_INT_REQ2(INT#45) 10 : MUXED_APPSS_RTI1_RTI2_INT_REQ3(INT#46) 11 : APPSS_SPI_IRQ_REQ(INT#14) 12 : SPI2_IRQ_REQ (part of INT#28) 13 : APPSS_LIN_INT0 (INT#10) 14 : APPSS_LIN_INT0 (INT#11) 15 : APPSS_MCAN_INT0(INT#21) 16 : APPSS_MCAN_INT1(INT#22) 17 : APPSS_SCI2_INT0(INT#62) 18 : APPSS_SCI2_INT0(INT#63) 19 : APPSS_SPI_IRQ_REQ(INT#14) 20 : SPI2_IRQ_REQ (part of INT#28) 21 : APPSS_LIN_INT0 (INT#10) 22 : APPSS_LIN_INT0 (INT#11) 23 : APPSS_MCAN_INT0(INT#21) 24 : APPSS_MCAN_INT1(INT#22) 25 : APPSS_SCI2_INT0(INT#62) 26 : APPSS_SCI2_INT0(INT#63) 27 : SYNC_IN 28 : RADAR_DEVICESLEEP_WAKEUP_INTERRUPT 29 to 31 : Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn wicmask(&mut self) -> WicmaskW<WicCtrlSpec> {
        WicmaskW::new(self, 0)
    }
}
#[doc = "WIC_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`wic_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wic_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WicCtrlSpec;
impl crate::RegisterSpec for WicCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wic_ctrl::R`](R) reader structure"]
impl crate::Readable for WicCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wic_ctrl::W`](W) writer structure"]
impl crate::Writable for WicCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIC_CTRL to value 0"]
impl crate::Resettable for WicCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
