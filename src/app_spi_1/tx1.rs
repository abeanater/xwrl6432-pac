#[doc = "Register `TX1` reader"]
pub type R = crate::R<Tx1Spec>;
#[doc = "Register `TX1` writer"]
pub type W = crate::W<Tx1Spec>;
#[doc = "Field `TDATA` reader - 31:0\\]
Channel 1 Data to transmit - (RW )"]
pub type TdataR = crate::FieldReader<u32>;
#[doc = "Field `TDATA` writer - 31:0\\]
Channel 1 Data to transmit - (RW )"]
pub type TdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 1 Data to transmit - (RW )"]
    #[inline(always)]
    pub fn tdata(&self) -> TdataR {
        TdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 1 Data to transmit - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn tdata(&mut self) -> TdataW<Tx1Spec> {
        TdataW::new(self, 0)
    }
}
#[doc = "This register contains a single SPI word to transmit on the serial link what ever SPI word length is.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tx1Spec;
impl crate::RegisterSpec for Tx1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx1::R`](R) reader structure"]
impl crate::Readable for Tx1Spec {}
#[doc = "`write(|w| ..)` method takes [`tx1::W`](W) writer structure"]
impl crate::Writable for Tx1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX1 to value 0"]
impl crate::Resettable for Tx1Spec {
    const RESET_VALUE: u32 = 0;
}
