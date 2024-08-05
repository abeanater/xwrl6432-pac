#[doc = "Register `REG11` reader"]
pub type R = crate::R<Reg11Spec>;
#[doc = "Register `REG11` writer"]
pub type W = crate::W<Reg11Spec>;
#[doc = "Field `PARAM_NOT_USED_TX_ENA3_OFF` reader - 31:0\\]
TI reserved"]
pub type ParamNotUsedTxEna3OffR = crate::FieldReader<u32>;
#[doc = "Field `PARAM_NOT_USED_TX_ENA3_OFF` writer - 31:0\\]
TI reserved"]
pub type ParamNotUsedTxEna3OffW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
TI reserved"]
    #[inline(always)]
    pub fn param_not_used_tx_ena3_off(&self) -> ParamNotUsedTxEna3OffR {
        ParamNotUsedTxEna3OffR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn param_not_used_tx_ena3_off(&mut self) -> ParamNotUsedTxEna3OffW<Reg11Spec> {
        ParamNotUsedTxEna3OffW::new(self, 0)
    }
}
#[doc = "REG11\n\nYou can [`read`](crate::Reg::read) this register and get [`reg11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg11Spec;
impl crate::RegisterSpec for Reg11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg11::R`](R) reader structure"]
impl crate::Readable for Reg11Spec {}
#[doc = "`write(|w| ..)` method takes [`reg11::W`](W) writer structure"]
impl crate::Writable for Reg11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG11 to value 0"]
impl crate::Resettable for Reg11Spec {
    const RESET_VALUE: u32 = 0;
}
