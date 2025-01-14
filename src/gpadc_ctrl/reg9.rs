#[doc = "Register `REG9` reader"]
pub type R = crate::R<Reg9Spec>;
#[doc = "Register `REG9` writer"]
pub type W = crate::W<Reg9Spec>;
#[doc = "Field `PARAM_NOT_USED_TX_ENA1_OFF` reader - 31:0\\]
TI reserved"]
pub type ParamNotUsedTxEna1OffR = crate::FieldReader<u32>;
#[doc = "Field `PARAM_NOT_USED_TX_ENA1_OFF` writer - 31:0\\]
TI reserved"]
pub type ParamNotUsedTxEna1OffW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
TI reserved"]
    #[inline(always)]
    pub fn param_not_used_tx_ena1_off(&self) -> ParamNotUsedTxEna1OffR {
        ParamNotUsedTxEna1OffR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn param_not_used_tx_ena1_off(&mut self) -> ParamNotUsedTxEna1OffW<Reg9Spec> {
        ParamNotUsedTxEna1OffW::new(self, 0)
    }
}
#[doc = "REG9\n\nYou can [`read`](crate::Reg::read) this register and get [`reg9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg9Spec;
impl crate::RegisterSpec for Reg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg9::R`](R) reader structure"]
impl crate::Readable for Reg9Spec {}
#[doc = "`write(|w| ..)` method takes [`reg9::W`](W) writer structure"]
impl crate::Writable for Reg9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG9 to value 0"]
impl crate::Resettable for Reg9Spec {
    const RESET_VALUE: u32 = 0;
}
