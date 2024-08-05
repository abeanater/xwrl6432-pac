#[doc = "Register `ADCBUFCFG1_EXTD` reader"]
pub type R = crate::R<Adcbufcfg1ExtdSpec>;
#[doc = "Register `ADCBUFCFG1_EXTD` writer"]
pub type W = crate::W<Adcbufcfg1ExtdSpec>;
#[doc = "Field `ADCBUFINTGENDLY` reader - 31:0\\]
TI Intenal Feature. No of clocks to delay the ping-pong switch and interrupt generation w.r.t ADC Valid fall pulse. This will enable dithering the DSP activity for successive ping-pong switch cycles. This will not delay the ping pong toggle which will happen immediately after ADC Valid fall."]
pub type AdcbufintgendlyR = crate::FieldReader<u32>;
#[doc = "Field `ADCBUFINTGENDLY` writer - 31:0\\]
TI Intenal Feature. No of clocks to delay the ping-pong switch and interrupt generation w.r.t ADC Valid fall pulse. This will enable dithering the DSP activity for successive ping-pong switch cycles. This will not delay the ping pong toggle which will happen immediately after ADC Valid fall."]
pub type AdcbufintgendlyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
TI Intenal Feature. No of clocks to delay the ping-pong switch and interrupt generation w.r.t ADC Valid fall pulse. This will enable dithering the DSP activity for successive ping-pong switch cycles. This will not delay the ping pong toggle which will happen immediately after ADC Valid fall."]
    #[inline(always)]
    pub fn adcbufintgendly(&self) -> AdcbufintgendlyR {
        AdcbufintgendlyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
TI Intenal Feature. No of clocks to delay the ping-pong switch and interrupt generation w.r.t ADC Valid fall pulse. This will enable dithering the DSP activity for successive ping-pong switch cycles. This will not delay the ping pong toggle which will happen immediately after ADC Valid fall."]
    #[inline(always)]
    #[must_use]
    pub fn adcbufintgendly(&mut self) -> AdcbufintgendlyW<Adcbufcfg1ExtdSpec> {
        AdcbufintgendlyW::new(self, 0)
    }
}
#[doc = "ADCBUFCFG1_EXTD\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbufcfg1_extd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbufcfg1_extd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcbufcfg1ExtdSpec;
impl crate::RegisterSpec for Adcbufcfg1ExtdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcbufcfg1_extd::R`](R) reader structure"]
impl crate::Readable for Adcbufcfg1ExtdSpec {}
#[doc = "`write(|w| ..)` method takes [`adcbufcfg1_extd::W`](W) writer structure"]
impl crate::Writable for Adcbufcfg1ExtdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCBUFCFG1_EXTD to value 0"]
impl crate::Resettable for Adcbufcfg1ExtdSpec {
    const RESET_VALUE: u32 = 0;
}
