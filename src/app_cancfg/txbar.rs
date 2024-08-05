#[doc = "Register `TXBAR` reader"]
pub type R = crate::R<TxbarSpec>;
#[doc = "Register `TXBAR` writer"]
pub type W = crate::W<TxbarSpec>;
#[doc = "Field `AR` reader - 31:0\\]
Add request"]
pub type ArR = crate::FieldReader<u32>;
#[doc = "Field `AR` writer - 31:0\\]
Add request"]
pub type ArW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Add request"]
    #[inline(always)]
    pub fn ar(&self) -> ArR {
        ArR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Add request"]
    #[inline(always)]
    #[must_use]
    pub fn ar(&mut self) -> ArW<TxbarSpec> {
        ArW::new(self, 0)
    }
}
#[doc = "TXBAR\n\nYou can [`read`](crate::Reg::read) this register and get [`txbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbarSpec;
impl crate::RegisterSpec for TxbarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbar::R`](R) reader structure"]
impl crate::Readable for TxbarSpec {}
#[doc = "`write(|w| ..)` method takes [`txbar::W`](W) writer structure"]
impl crate::Writable for TxbarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBAR to value 0"]
impl crate::Resettable for TxbarSpec {
    const RESET_VALUE: u32 = 0;
}
