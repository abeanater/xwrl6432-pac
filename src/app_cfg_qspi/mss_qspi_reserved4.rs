#[doc = "Register `MSS_QSPI_Reserved4` reader"]
pub type R = crate::R<MssQspiReserved4Spec>;
#[doc = "Register `MSS_QSPI_Reserved4` writer"]
pub type W = crate::W<MssQspiReserved4Spec>;
#[doc = "Field `Reserved_4` reader - 31:0\\]
Reserved"]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `Reserved_4` writer - 31:0\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn reserved_4(&self) -> Reserved4R {
        Reserved4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_4(&mut self) -> Reserved4W<MssQspiReserved4Spec> {
        Reserved4W::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_qspi_reserved4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_qspi_reserved4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssQspiReserved4Spec;
impl crate::RegisterSpec for MssQspiReserved4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_qspi_reserved4::R`](R) reader structure"]
impl crate::Readable for MssQspiReserved4Spec {}
#[doc = "`write(|w| ..)` method takes [`mss_qspi_reserved4::W`](W) writer structure"]
impl crate::Writable for MssQspiReserved4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_QSPI_Reserved4 to value 0"]
impl crate::Resettable for MssQspiReserved4Spec {
    const RESET_VALUE: u32 = 0;
}
