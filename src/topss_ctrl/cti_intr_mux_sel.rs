#[doc = "Register `CTI_INTR_MUX_SEL` reader"]
pub type R = crate::R<CtiIntrMuxSelSpec>;
#[doc = "Register `CTI_INTR_MUX_SEL` writer"]
pub type W = crate::W<CtiIntrMuxSelSpec>;
#[doc = "Field `CTI0_intr_mux_select` reader - 1:0\\]
CTI0 mux select 2'b00: ESM_LO_IRQ 2'b01: FEC_INTRundefined 2'b10: FEC_INTRundefined"]
pub type Cti0IntrMuxSelectR = crate::FieldReader;
#[doc = "Field `CTI0_intr_mux_select` writer - 1:0\\]
CTI0 mux select 2'b00: ESM_LO_IRQ 2'b01: FEC_INTRundefined 2'b10: FEC_INTRundefined"]
pub type Cti0IntrMuxSelectW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTI1_intr_mux_select` reader - 10:8\\]
CTI1 mux select 3'b000: app_rti_int_req0 3'b001: app_rti_int_req1 3'b010: tpcc_1_intagg 3'b011: tpcc_2_intagg 3'b100: hwa_loop_int 3'b101: hwa_paramdone_int"]
pub type Cti1IntrMuxSelectR = crate::FieldReader;
#[doc = "Field `CTI1_intr_mux_select` writer - 10:8\\]
CTI1 mux select 3'b000: app_rti_int_req0 3'b001: app_rti_int_req1 3'b010: tpcc_1_intagg 3'b011: tpcc_2_intagg 3'b100: hwa_loop_int 3'b101: hwa_paramdone_int"]
pub type Cti1IntrMuxSelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CTI2_intr_mux_select` reader - 17:16\\]
CTI2 mux select 2'b00: CHIRP_AVAIL_IRQ 2'b01: ADC_VALID_START 2'b10: CHIRPTIMER_CHIRP_START 2'b11: CHIRPTIMER_CHIRP_END"]
pub type Cti2IntrMuxSelectR = crate::FieldReader;
#[doc = "Field `CTI2_intr_mux_select` writer - 17:16\\]
CTI2 mux select 2'b00: CHIRP_AVAIL_IRQ 2'b01: ADC_VALID_START 2'b10: CHIRPTIMER_CHIRP_START 2'b11: CHIRPTIMER_CHIRP_END"]
pub type Cti2IntrMuxSelectW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTI3_intr_mux_select` reader - 26:24\\]
CTI3 mux select 3'b000: FRAMETIMER_FRAME_START 3'b001: FRAME_START_OFFSET_INTR_TIME1 3'b010: BURST_START_OFFSET_TIME 3'b011: CHIRPTIMER_BURST_START 3'b100: CHIRPTIMER_BURST_END"]
pub type Cti3IntrMuxSelectR = crate::FieldReader;
#[doc = "Field `CTI3_intr_mux_select` writer - 26:24\\]
CTI3 mux select 3'b000: FRAMETIMER_FRAME_START 3'b001: FRAME_START_OFFSET_INTR_TIME1 3'b010: BURST_START_OFFSET_TIME 3'b011: CHIRPTIMER_BURST_START 3'b100: CHIRPTIMER_BURST_END"]
pub type Cti3IntrMuxSelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
CTI0 mux select 2'b00: ESM_LO_IRQ 2'b01: FEC_INTRundefined 2'b10: FEC_INTRundefined"]
    #[inline(always)]
    pub fn cti0_intr_mux_select(&self) -> Cti0IntrMuxSelectR {
        Cti0IntrMuxSelectR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
CTI1 mux select 3'b000: app_rti_int_req0 3'b001: app_rti_int_req1 3'b010: tpcc_1_intagg 3'b011: tpcc_2_intagg 3'b100: hwa_loop_int 3'b101: hwa_paramdone_int"]
    #[inline(always)]
    pub fn cti1_intr_mux_select(&self) -> Cti1IntrMuxSelectR {
        Cti1IntrMuxSelectR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
CTI2 mux select 2'b00: CHIRP_AVAIL_IRQ 2'b01: ADC_VALID_START 2'b10: CHIRPTIMER_CHIRP_START 2'b11: CHIRPTIMER_CHIRP_END"]
    #[inline(always)]
    pub fn cti2_intr_mux_select(&self) -> Cti2IntrMuxSelectR {
        Cti2IntrMuxSelectR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
CTI3 mux select 3'b000: FRAMETIMER_FRAME_START 3'b001: FRAME_START_OFFSET_INTR_TIME1 3'b010: BURST_START_OFFSET_TIME 3'b011: CHIRPTIMER_BURST_START 3'b100: CHIRPTIMER_BURST_END"]
    #[inline(always)]
    pub fn cti3_intr_mux_select(&self) -> Cti3IntrMuxSelectR {
        Cti3IntrMuxSelectR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
CTI0 mux select 2'b00: ESM_LO_IRQ 2'b01: FEC_INTRundefined 2'b10: FEC_INTRundefined"]
    #[inline(always)]
    #[must_use]
    pub fn cti0_intr_mux_select(&mut self) -> Cti0IntrMuxSelectW<CtiIntrMuxSelSpec> {
        Cti0IntrMuxSelectW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
CTI1 mux select 3'b000: app_rti_int_req0 3'b001: app_rti_int_req1 3'b010: tpcc_1_intagg 3'b011: tpcc_2_intagg 3'b100: hwa_loop_int 3'b101: hwa_paramdone_int"]
    #[inline(always)]
    #[must_use]
    pub fn cti1_intr_mux_select(&mut self) -> Cti1IntrMuxSelectW<CtiIntrMuxSelSpec> {
        Cti1IntrMuxSelectW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
CTI2 mux select 2'b00: CHIRP_AVAIL_IRQ 2'b01: ADC_VALID_START 2'b10: CHIRPTIMER_CHIRP_START 2'b11: CHIRPTIMER_CHIRP_END"]
    #[inline(always)]
    #[must_use]
    pub fn cti2_intr_mux_select(&mut self) -> Cti2IntrMuxSelectW<CtiIntrMuxSelSpec> {
        Cti2IntrMuxSelectW::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
CTI3 mux select 3'b000: FRAMETIMER_FRAME_START 3'b001: FRAME_START_OFFSET_INTR_TIME1 3'b010: BURST_START_OFFSET_TIME 3'b011: CHIRPTIMER_BURST_START 3'b100: CHIRPTIMER_BURST_END"]
    #[inline(always)]
    #[must_use]
    pub fn cti3_intr_mux_select(&mut self) -> Cti3IntrMuxSelectW<CtiIntrMuxSelSpec> {
        Cti3IntrMuxSelectW::new(self, 24)
    }
}
#[doc = "CTI_INTR_MUX_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`cti_intr_mux_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cti_intr_mux_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtiIntrMuxSelSpec;
impl crate::RegisterSpec for CtiIntrMuxSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti_intr_mux_sel::R`](R) reader structure"]
impl crate::Readable for CtiIntrMuxSelSpec {}
#[doc = "`write(|w| ..)` method takes [`cti_intr_mux_sel::W`](W) writer structure"]
impl crate::Writable for CtiIntrMuxSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI_INTR_MUX_SEL to value 0"]
impl crate::Resettable for CtiIntrMuxSelSpec {
    const RESET_VALUE: u32 = 0;
}
