#[doc = "Register `TXBCR` reader"]
pub type R = crate::R<TxbcrSpec>;
#[doc = "Register `TXBCR` writer"]
pub type W = crate::W<TxbcrSpec>;
#[doc = "Field `CR` reader - 31:0\\]
Cancellation Request"]
pub type CrR = crate::FieldReader<u32>;
#[doc = "Field `CR` writer - 31:0\\]
Cancellation Request"]
pub type CrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Cancellation Request"]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Cancellation Request"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CrW<TxbcrSpec> {
        CrW::new(self, 0)
    }
}
#[doc = "TXBCR\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbcrSpec;
impl crate::RegisterSpec for TxbcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcr::R`](R) reader structure"]
impl crate::Readable for TxbcrSpec {}
#[doc = "`write(|w| ..)` method takes [`txbcr::W`](W) writer structure"]
impl crate::Writable for TxbcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBCR to value 0"]
impl crate::Resettable for TxbcrSpec {
    const RESET_VALUE: u32 = 0;
}
