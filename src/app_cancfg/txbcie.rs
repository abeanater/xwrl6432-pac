#[doc = "Register `TXBCIE` reader"]
pub type R = crate::R<TxbcieSpec>;
#[doc = "Register `TXBCIE` writer"]
pub type W = crate::W<TxbcieSpec>;
#[doc = "Field `CFIE` reader - 31:0\\]
Cancellation Finished Interrupt Enable"]
pub type CfieR = crate::FieldReader<u32>;
#[doc = "Field `CFIE` writer - 31:0\\]
Cancellation Finished Interrupt Enable"]
pub type CfieW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub fn cfie(&self) -> CfieR {
        CfieR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfie(&mut self) -> CfieW<TxbcieSpec> {
        CfieW::new(self, 0)
    }
}
#[doc = "TXBCIE\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbcieSpec;
impl crate::RegisterSpec for TxbcieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcie::R`](R) reader structure"]
impl crate::Readable for TxbcieSpec {}
#[doc = "`write(|w| ..)` method takes [`txbcie::W`](W) writer structure"]
impl crate::Writable for TxbcieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBCIE to value 0"]
impl crate::Resettable for TxbcieSpec {
    const RESET_VALUE: u32 = 0;
}
