#[doc = "Register `REG10` reader"]
pub type R = crate::R<Reg10Spec>;
#[doc = "Register `REG10` writer"]
pub type W = crate::W<Reg10Spec>;
#[doc = "Field `PARAM_NOT_USED_TX_ENA2_OFF` reader - 31:0\\]
TI reserved"]
pub type ParamNotUsedTxEna2OffR = crate::FieldReader<u32>;
#[doc = "Field `PARAM_NOT_USED_TX_ENA2_OFF` writer - 31:0\\]
TI reserved"]
pub type ParamNotUsedTxEna2OffW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
TI reserved"]
    #[inline(always)]
    pub fn param_not_used_tx_ena2_off(&self) -> ParamNotUsedTxEna2OffR {
        ParamNotUsedTxEna2OffR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn param_not_used_tx_ena2_off(&mut self) -> ParamNotUsedTxEna2OffW<Reg10Spec> {
        ParamNotUsedTxEna2OffW::new(self, 0)
    }
}
#[doc = "REG10\n\nYou can [`read`](crate::Reg::read) this register and get [`reg10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg10Spec;
impl crate::RegisterSpec for Reg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg10::R`](R) reader structure"]
impl crate::Readable for Reg10Spec {}
#[doc = "`write(|w| ..)` method takes [`reg10::W`](W) writer structure"]
impl crate::Writable for Reg10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG10 to value 0"]
impl crate::Resettable for Reg10Spec {
    const RESET_VALUE: u32 = 0;
}
