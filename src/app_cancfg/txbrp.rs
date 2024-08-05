#[doc = "Register `TXBRP` reader"]
pub type R = crate::R<TxbrpSpec>;
#[doc = "Register `TXBRP` writer"]
pub type W = crate::W<TxbrpSpec>;
#[doc = "Field `TRP` reader - 31:0\\]
Transmission Request Pending"]
pub type TrpR = crate::FieldReader<u32>;
#[doc = "Field `TRP` writer - 31:0\\]
Transmission Request Pending"]
pub type TrpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Transmission Request Pending"]
    #[inline(always)]
    pub fn trp(&self) -> TrpR {
        TrpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Transmission Request Pending"]
    #[inline(always)]
    #[must_use]
    pub fn trp(&mut self) -> TrpW<TxbrpSpec> {
        TrpW::new(self, 0)
    }
}
#[doc = "TXBRP\n\nYou can [`read`](crate::Reg::read) this register and get [`txbrp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbrp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbrpSpec;
impl crate::RegisterSpec for TxbrpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbrp::R`](R) reader structure"]
impl crate::Readable for TxbrpSpec {}
#[doc = "`write(|w| ..)` method takes [`txbrp::W`](W) writer structure"]
impl crate::Writable for TxbrpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBRP to value 0"]
impl crate::Resettable for TxbrpSpec {
    const RESET_VALUE: u32 = 0;
}
