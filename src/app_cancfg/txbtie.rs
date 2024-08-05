#[doc = "Register `TXBTIE` reader"]
pub type R = crate::R<TxbtieSpec>;
#[doc = "Register `TXBTIE` writer"]
pub type W = crate::W<TxbtieSpec>;
#[doc = "Field `TIE` reader - 31:0\\]
Transmission Interrupt Enable"]
pub type TieR = crate::FieldReader<u32>;
#[doc = "Field `TIE` writer - 31:0\\]
Transmission Interrupt Enable"]
pub type TieW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Transmission Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Transmission Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TieW<TxbtieSpec> {
        TieW::new(self, 0)
    }
}
#[doc = "TXBTIE\n\nYou can [`read`](crate::Reg::read) this register and get [`txbtie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbtie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbtieSpec;
impl crate::RegisterSpec for TxbtieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbtie::R`](R) reader structure"]
impl crate::Readable for TxbtieSpec {}
#[doc = "`write(|w| ..)` method takes [`txbtie::W`](W) writer structure"]
impl crate::Writable for TxbtieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBTIE to value 0"]
impl crate::Resettable for TxbtieSpec {
    const RESET_VALUE: u32 = 0;
}
