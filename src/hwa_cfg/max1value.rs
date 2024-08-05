#[doc = "Register `MAX1VALUE` reader"]
pub type R = crate::R<Max1valueSpec>;
#[doc = "Register `MAX1VALUE` writer"]
pub type W = crate::W<Max1valueSpec>;
#[doc = "Field `MAX1VALUE` reader - 23:0\\]
Max value: These registers contain the max value on a per-iteration basis. These registers are meaningful only when Magnitude or Log-Magnitude is enabled. Only the max values for up to four iterations are recorded in these registers. For larger number of iterations, use Statistics output mode (FFT_OUT_MODE in HW_ACC_PARAM register set)."]
pub type Max1valueR = crate::FieldReader<u32>;
#[doc = "Field `MAX1VALUE` writer - 23:0\\]
Max value: These registers contain the max value on a per-iteration basis. These registers are meaningful only when Magnitude or Log-Magnitude is enabled. Only the max values for up to four iterations are recorded in these registers. For larger number of iterations, use Statistics output mode (FFT_OUT_MODE in HW_ACC_PARAM register set)."]
pub type Max1valueW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Max value: These registers contain the max value on a per-iteration basis. These registers are meaningful only when Magnitude or Log-Magnitude is enabled. Only the max values for up to four iterations are recorded in these registers. For larger number of iterations, use Statistics output mode (FFT_OUT_MODE in HW_ACC_PARAM register set)."]
    #[inline(always)]
    pub fn max1value(&self) -> Max1valueR {
        Max1valueR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Max value: These registers contain the max value on a per-iteration basis. These registers are meaningful only when Magnitude or Log-Magnitude is enabled. Only the max values for up to four iterations are recorded in these registers. For larger number of iterations, use Statistics output mode (FFT_OUT_MODE in HW_ACC_PARAM register set)."]
    #[inline(always)]
    #[must_use]
    pub fn max1value(&mut self) -> Max1valueW<Max1valueSpec> {
        Max1valueW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Max1valueSpec> {
        NuW::new(self, 24)
    }
}
#[doc = "MAX1VALUE\n\nYou can [`read`](crate::Reg::read) this register and get [`max1value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max1value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Max1valueSpec;
impl crate::RegisterSpec for Max1valueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max1value::R`](R) reader structure"]
impl crate::Readable for Max1valueSpec {}
#[doc = "`write(|w| ..)` method takes [`max1value::W`](W) writer structure"]
impl crate::Writable for Max1valueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAX1VALUE to value 0"]
impl crate::Resettable for Max1valueSpec {
    const RESET_VALUE: u32 = 0;
}
