#[doc = "Register `CUST` reader"]
pub type R = crate::R<CustSpec>;
#[doc = "Register `CUST` writer"]
pub type W = crate::W<CustSpec>;
#[doc = "Field `CUST` reader - 31:0\\]
Custom"]
pub type CustR = crate::FieldReader<u32>;
#[doc = "Field `CUST` writer - 31:0\\]
Custom"]
pub type CustW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Custom"]
    #[inline(always)]
    pub fn cust(&self) -> CustR {
        CustR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Custom"]
    #[inline(always)]
    #[must_use]
    pub fn cust(&mut self) -> CustW<CustSpec> {
        CustW::new(self, 0)
    }
}
#[doc = "CUST\n\nYou can [`read`](crate::Reg::read) this register and get [`cust::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cust::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CustSpec;
impl crate::RegisterSpec for CustSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cust::R`](R) reader structure"]
impl crate::Readable for CustSpec {}
#[doc = "`write(|w| ..)` method takes [`cust::W`](W) writer structure"]
impl crate::Writable for CustSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CUST to value 0"]
impl crate::Resettable for CustSpec {
    const RESET_VALUE: u32 = 0;
}
