#[doc = "Register `FFTINTMEMRDDATA` reader"]
pub type R = crate::R<FftintmemrddataSpec>;
#[doc = "Register `FFTINTMEMRDDATA` writer"]
pub type W = crate::W<FftintmemrddataSpec>;
#[doc = "Field `FFTINTMEMRDDATA` reader - 31:0\\]
Reserved.TI internal"]
pub type FftintmemrddataR = crate::FieldReader<u32>;
#[doc = "Field `FFTINTMEMRDDATA` writer - 31:0\\]
Reserved.TI internal"]
pub type FftintmemrddataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn fftintmemrddata(&self) -> FftintmemrddataR {
        FftintmemrddataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn fftintmemrddata(&mut self) -> FftintmemrddataW<FftintmemrddataSpec> {
        FftintmemrddataW::new(self, 0)
    }
}
#[doc = "FFTINTMEMRDDATA\n\nYou can [`read`](crate::Reg::read) this register and get [`fftintmemrddata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fftintmemrddata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FftintmemrddataSpec;
impl crate::RegisterSpec for FftintmemrddataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fftintmemrddata::R`](R) reader structure"]
impl crate::Readable for FftintmemrddataSpec {}
#[doc = "`write(|w| ..)` method takes [`fftintmemrddata::W`](W) writer structure"]
impl crate::Writable for FftintmemrddataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFTINTMEMRDDATA to value 0"]
impl crate::Resettable for FftintmemrddataSpec {
    const RESET_VALUE: u32 = 0;
}
