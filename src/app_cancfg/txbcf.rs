#[doc = "Register `TXBCF` reader"]
pub type R = crate::R<TxbcfSpec>;
#[doc = "Register `TXBCF` writer"]
pub type W = crate::W<TxbcfSpec>;
#[doc = "Field `CF` reader - 31:0\\]
Cancellation Finished"]
pub type CfR = crate::FieldReader<u32>;
#[doc = "Field `CF` writer - 31:0\\]
Cancellation Finished"]
pub type CfW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Cancellation Finished"]
    #[inline(always)]
    pub fn cf(&self) -> CfR {
        CfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Cancellation Finished"]
    #[inline(always)]
    #[must_use]
    pub fn cf(&mut self) -> CfW<TxbcfSpec> {
        CfW::new(self, 0)
    }
}
#[doc = "TXBCF\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbcfSpec;
impl crate::RegisterSpec for TxbcfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcf::R`](R) reader structure"]
impl crate::Readable for TxbcfSpec {}
#[doc = "`write(|w| ..)` method takes [`txbcf::W`](W) writer structure"]
impl crate::Writable for TxbcfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBCF to value 0"]
impl crate::Resettable for TxbcfSpec {
    const RESET_VALUE: u32 = 0;
}
