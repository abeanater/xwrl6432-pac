#[doc = "Register `REG22` reader"]
pub type R = crate::R<Reg22Spec>;
#[doc = "Register `REG22` writer"]
pub type W = crate::W<Reg22Spec>;
#[doc = "Field `SPARE_WR1` reader - 31:0\\]
TI reserved"]
pub type SpareWr1R = crate::FieldReader<u32>;
#[doc = "Field `SPARE_WR1` writer - 31:0\\]
TI reserved"]
pub type SpareWr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
TI reserved"]
    #[inline(always)]
    pub fn spare_wr1(&self) -> SpareWr1R {
        SpareWr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn spare_wr1(&mut self) -> SpareWr1W<Reg22Spec> {
        SpareWr1W::new(self, 0)
    }
}
#[doc = "REG22\n\nYou can [`read`](crate::Reg::read) this register and get [`reg22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg22Spec;
impl crate::RegisterSpec for Reg22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg22::R`](R) reader structure"]
impl crate::Readable for Reg22Spec {}
#[doc = "`write(|w| ..)` method takes [`reg22::W`](W) writer structure"]
impl crate::Writable for Reg22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG22 to value 0"]
impl crate::Resettable for Reg22Spec {
    const RESET_VALUE: u32 = 0;
}
