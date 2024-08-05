#[doc = "Register `FFTINTMEMWRDATA` reader"]
pub type R = crate::R<FftintmemwrdataSpec>;
#[doc = "Register `FFTINTMEMWRDATA` writer"]
pub type W = crate::W<FftintmemwrdataSpec>;
#[doc = "Field `FFTINTMEMWRDATA` reader - 31:0\\]
Reserved.TI internal"]
pub type FftintmemwrdataR = crate::FieldReader<u32>;
#[doc = "Field `FFTINTMEMWRDATA` writer - 31:0\\]
Reserved.TI internal"]
pub type FftintmemwrdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn fftintmemwrdata(&self) -> FftintmemwrdataR {
        FftintmemwrdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn fftintmemwrdata(&mut self) -> FftintmemwrdataW<FftintmemwrdataSpec> {
        FftintmemwrdataW::new(self, 0)
    }
}
#[doc = "FFTINTMEMWRDATA\n\nYou can [`read`](crate::Reg::read) this register and get [`fftintmemwrdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fftintmemwrdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FftintmemwrdataSpec;
impl crate::RegisterSpec for FftintmemwrdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fftintmemwrdata::R`](R) reader structure"]
impl crate::Readable for FftintmemwrdataSpec {}
#[doc = "`write(|w| ..)` method takes [`fftintmemwrdata::W`](W) writer structure"]
impl crate::Writable for FftintmemwrdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFTINTMEMWRDATA to value 0"]
impl crate::Resettable for FftintmemwrdataSpec {
    const RESET_VALUE: u32 = 0;
}
