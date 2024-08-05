#[doc = "Register `REG20` reader"]
pub type R = crate::R<Reg20Spec>;
#[doc = "Register `REG20` writer"]
pub type W = crate::W<Reg20Spec>;
#[doc = "Field `SPARE_RD1` reader - 31:0\\]
TI reserved"]
pub type SpareRd1R = crate::FieldReader<u32>;
#[doc = "Field `SPARE_RD1` writer - 31:0\\]
TI reserved"]
pub type SpareRd1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
TI reserved"]
    #[inline(always)]
    pub fn spare_rd1(&self) -> SpareRd1R {
        SpareRd1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn spare_rd1(&mut self) -> SpareRd1W<Reg20Spec> {
        SpareRd1W::new(self, 0)
    }
}
#[doc = "REG20\n\nYou can [`read`](crate::Reg::read) this register and get [`reg20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg20Spec;
impl crate::RegisterSpec for Reg20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg20::R`](R) reader structure"]
impl crate::Readable for Reg20Spec {}
#[doc = "`write(|w| ..)` method takes [`reg20::W`](W) writer structure"]
impl crate::Writable for Reg20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG20 to value 0"]
impl crate::Resettable for Reg20Spec {
    const RESET_VALUE: u32 = 0;
}
