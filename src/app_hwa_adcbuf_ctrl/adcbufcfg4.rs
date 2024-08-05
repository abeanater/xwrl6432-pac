#[doc = "Register `ADCBUFCFG4` reader"]
pub type R = crate::R<Adcbufcfg4Spec>;
#[doc = "Register `ADCBUFCFG4` writer"]
pub type W = crate::W<Adcbufcfg4Spec>;
#[doc = "Field `ADCBUFSAMPCNT` reader - 15:0\\]
No of samples to store in each Ping and Pong register in continuous mode of ADC Buffer. In real only mode this refers to the number of real samples and in complex mode, this refers to number of complex samples. This refers to the number of samples per channel. This counter increments once for every new sample from DFE (as long as 1 or more channels are enabled). The max allowed value varies depending on other configurations (No of channels enabled and real/complex data). Continous mode is expected to be only used for CZ and ADC Buffer Testpattern mode"]
pub type AdcbufsampcntR = crate::FieldReader<u16>;
#[doc = "Field `ADCBUFSAMPCNT` writer - 15:0\\]
No of samples to store in each Ping and Pong register in continuous mode of ADC Buffer. In real only mode this refers to the number of real samples and in complex mode, this refers to number of complex samples. This refers to the number of samples per channel. This counter increments once for every new sample from DFE (as long as 1 or more channels are enabled). The max allowed value varies depending on other configurations (No of channels enabled and real/complex data). Continous mode is expected to be only used for CZ and ADC Buffer Testpattern mode"]
pub type AdcbufsampcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADCBUFNUMCHRPPING` reader - 20:16\\]
Number of chirps to be stored in Ping / Pong buffer. This register should be programmed with one less than the actual number needed."]
pub type AdcbufnumchrppingR = crate::FieldReader;
#[doc = "Field `ADCBUFNUMCHRPPING` writer - 20:16\\]
Number of chirps to be stored in Ping / Pong buffer. This register should be programmed with one less than the actual number needed."]
pub type AdcbufnumchrppingW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADCBUFPNGSELTGLDIS` reader - 30:30\\]
TI Internal Feature 0 --> Delay Interrupt Gen and Ping/Pong toggle together based on cfg_interrupt_gen_delay, 1 --> Delay only Interrupt Gen based on cfg_interrupt_gen_delay. But toggle Ping/Pong select signal as soon as the write is complete."]
pub type AdcbufpngseltgldisR = crate::BitReader;
#[doc = "Field `ADCBUFPNGSELTGLDIS` writer - 30:30\\]
TI Internal Feature 0 --> Delay Interrupt Gen and Ping/Pong toggle together based on cfg_interrupt_gen_delay, 1 --> Delay only Interrupt Gen based on cfg_interrupt_gen_delay. But toggle Ping/Pong select signal as soon as the write is complete."]
pub type AdcbufpngseltgldisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
No of samples to store in each Ping and Pong register in continuous mode of ADC Buffer. In real only mode this refers to the number of real samples and in complex mode, this refers to number of complex samples. This refers to the number of samples per channel. This counter increments once for every new sample from DFE (as long as 1 or more channels are enabled). The max allowed value varies depending on other configurations (No of channels enabled and real/complex data). Continous mode is expected to be only used for CZ and ADC Buffer Testpattern mode"]
    #[inline(always)]
    pub fn adcbufsampcnt(&self) -> AdcbufsampcntR {
        AdcbufsampcntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Number of chirps to be stored in Ping / Pong buffer. This register should be programmed with one less than the actual number needed."]
    #[inline(always)]
    pub fn adcbufnumchrpping(&self) -> AdcbufnumchrppingR {
        AdcbufnumchrppingR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
TI Internal Feature 0 --> Delay Interrupt Gen and Ping/Pong toggle together based on cfg_interrupt_gen_delay, 1 --> Delay only Interrupt Gen based on cfg_interrupt_gen_delay. But toggle Ping/Pong select signal as soon as the write is complete."]
    #[inline(always)]
    pub fn adcbufpngseltgldis(&self) -> AdcbufpngseltgldisR {
        AdcbufpngseltgldisR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
No of samples to store in each Ping and Pong register in continuous mode of ADC Buffer. In real only mode this refers to the number of real samples and in complex mode, this refers to number of complex samples. This refers to the number of samples per channel. This counter increments once for every new sample from DFE (as long as 1 or more channels are enabled). The max allowed value varies depending on other configurations (No of channels enabled and real/complex data). Continous mode is expected to be only used for CZ and ADC Buffer Testpattern mode"]
    #[inline(always)]
    #[must_use]
    pub fn adcbufsampcnt(&mut self) -> AdcbufsampcntW<Adcbufcfg4Spec> {
        AdcbufsampcntW::new(self, 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Number of chirps to be stored in Ping / Pong buffer. This register should be programmed with one less than the actual number needed."]
    #[inline(always)]
    #[must_use]
    pub fn adcbufnumchrpping(&mut self) -> AdcbufnumchrppingW<Adcbufcfg4Spec> {
        AdcbufnumchrppingW::new(self, 16)
    }
    #[doc = "Bit 30 - 30:30\\]
TI Internal Feature 0 --> Delay Interrupt Gen and Ping/Pong toggle together based on cfg_interrupt_gen_delay, 1 --> Delay only Interrupt Gen based on cfg_interrupt_gen_delay. But toggle Ping/Pong select signal as soon as the write is complete."]
    #[inline(always)]
    #[must_use]
    pub fn adcbufpngseltgldis(&mut self) -> AdcbufpngseltgldisW<Adcbufcfg4Spec> {
        AdcbufpngseltgldisW::new(self, 30)
    }
}
#[doc = "ADCBUFCFG4\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbufcfg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbufcfg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcbufcfg4Spec;
impl crate::RegisterSpec for Adcbufcfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcbufcfg4::R`](R) reader structure"]
impl crate::Readable for Adcbufcfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`adcbufcfg4::W`](W) writer structure"]
impl crate::Writable for Adcbufcfg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCBUFCFG4 to value 0"]
impl crate::Resettable for Adcbufcfg4Spec {
    const RESET_VALUE: u32 = 0;
}
