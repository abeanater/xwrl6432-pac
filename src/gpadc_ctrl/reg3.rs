#[doc = "Register `REG3` reader"]
pub type R = crate::R<Reg3Spec>;
#[doc = "Register `REG3` writer"]
pub type W = crate::W<Reg3Spec>;
#[doc = "Field `PARAM_VAL_IFM` reader - 7:0\\]
Param value to be passed to analog in IFM mode(after one hot encoding)"]
pub type ParamValIfmR = crate::FieldReader;
#[doc = "Field `PARAM_VAL_IFM` writer - 7:0\\]
Param value to be passed to analog in IFM mode(after one hot encoding)"]
pub type ParamValIfmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLLECT_SAMPLES_IFM` reader - 15:8\\]
number of GPADC readings to collect"]
pub type CollectSamplesIfmR = crate::FieldReader;
#[doc = "Field `COLLECT_SAMPLES_IFM` writer - 15:8\\]
number of GPADC readings to collect"]
pub type CollectSamplesIfmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SKIP_SAMPLES_IFM` reader - 22:16\\]
number of GPADC clocks to skip after trigger . Number of samples to skip = skip_samples_ifm\\[3:0\\]x(2skip_samples_ifm\\[6:4\\])"]
pub type SkipSamplesIfmR = crate::FieldReader;
#[doc = "Field `SKIP_SAMPLES_IFM` writer - 22:16\\]
number of GPADC clocks to skip after trigger . Number of samples to skip = skip_samples_ifm\\[3:0\\]x(2skip_samples_ifm\\[6:4\\])"]
pub type SkipSamplesIfmW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u16>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Param value to be passed to analog in IFM mode(after one hot encoding)"]
    #[inline(always)]
    pub fn param_val_ifm(&self) -> ParamValIfmR {
        ParamValIfmR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
number of GPADC readings to collect"]
    #[inline(always)]
    pub fn collect_samples_ifm(&self) -> CollectSamplesIfmR {
        CollectSamplesIfmR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
number of GPADC clocks to skip after trigger . Number of samples to skip = skip_samples_ifm\\[3:0\\]x(2skip_samples_ifm\\[6:4\\])"]
    #[inline(always)]
    pub fn skip_samples_ifm(&self) -> SkipSamplesIfmR {
        SkipSamplesIfmR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Param value to be passed to analog in IFM mode(after one hot encoding)"]
    #[inline(always)]
    #[must_use]
    pub fn param_val_ifm(&mut self) -> ParamValIfmW<Reg3Spec> {
        ParamValIfmW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
number of GPADC readings to collect"]
    #[inline(always)]
    #[must_use]
    pub fn collect_samples_ifm(&mut self) -> CollectSamplesIfmW<Reg3Spec> {
        CollectSamplesIfmW::new(self, 8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
number of GPADC clocks to skip after trigger . Number of samples to skip = skip_samples_ifm\\[3:0\\]x(2skip_samples_ifm\\[6:4\\])"]
    #[inline(always)]
    #[must_use]
    pub fn skip_samples_ifm(&mut self) -> SkipSamplesIfmW<Reg3Spec> {
        SkipSamplesIfmW::new(self, 16)
    }
    #[doc = "Bits 23:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Reg3Spec> {
        NuW::new(self, 23)
    }
}
#[doc = "gpadc param, skip samples and collect samples for IFM\n\nYou can [`read`](crate::Reg::read) this register and get [`reg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg3Spec;
impl crate::RegisterSpec for Reg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg3::R`](R) reader structure"]
impl crate::Readable for Reg3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg3::W`](W) writer structure"]
impl crate::Writable for Reg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG3 to value 0"]
impl crate::Resettable for Reg3Spec {
    const RESET_VALUE: u32 = 0;
}
