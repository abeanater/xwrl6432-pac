#[doc = "Register `dft_pbist_st_key` reader"]
pub type R = crate::R<DftPbistStKeySpec>;
#[doc = "Register `dft_pbist_st_key` writer"]
pub type W = crate::W<DftPbistStKeySpec>;
#[doc = "Field `reg` reader - 31:0\\]
dft_pbist_st_key"]
pub type RegR = crate::FieldReader<u32>;
#[doc = "Field `reg` writer - 31:0\\]
dft_pbist_st_key"]
pub type RegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
dft_pbist_st_key"]
    #[inline(always)]
    pub fn reg(&self) -> RegR {
        RegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
dft_pbist_st_key"]
    #[inline(always)]
    #[must_use]
    pub fn reg(&mut self) -> RegW<DftPbistStKeySpec> {
        RegW::new(self, 0)
    }
}
#[doc = "dft_pbist_st_key\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_pbist_st_key::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_pbist_st_key::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DftPbistStKeySpec;
impl crate::RegisterSpec for DftPbistStKeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dft_pbist_st_key::R`](R) reader structure"]
impl crate::Readable for DftPbistStKeySpec {}
#[doc = "`write(|w| ..)` method takes [`dft_pbist_st_key::W`](W) writer structure"]
impl crate::Writable for DftPbistStKeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dft_pbist_st_key to value 0"]
impl crate::Resettable for DftPbistStKeySpec {
    const RESET_VALUE: u32 = 0;
}
