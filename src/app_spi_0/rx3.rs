#[doc = "Register `RX3` reader"]
pub type R = crate::R<Rx3Spec>;
#[doc = "Register `RX3` writer"]
pub type W = crate::W<Rx3Spec>;
#[doc = "Field `RDATA` reader - 31:0\\]
Channel 3 Received Data - (RO )"]
pub type RdataR = crate::FieldReader<u32>;
#[doc = "Field `RDATA` writer - 31:0\\]
Channel 3 Received Data - (RO )"]
pub type RdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 3 Received Data - (RO )"]
    #[inline(always)]
    pub fn rdata(&self) -> RdataR {
        RdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 3 Received Data - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn rdata(&mut self) -> RdataW<Rx3Spec> {
        RdataW::new(self, 0)
    }
}
#[doc = "This register contains a single SPI word received through the serial link what ever SPI word length is.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rx3Spec;
impl crate::RegisterSpec for Rx3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx3::R`](R) reader structure"]
impl crate::Readable for Rx3Spec {}
#[doc = "`write(|w| ..)` method takes [`rx3::W`](W) writer structure"]
impl crate::Writable for Rx3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX3 to value 0"]
impl crate::Resettable for Rx3Spec {
    const RESET_VALUE: u32 = 0;
}
