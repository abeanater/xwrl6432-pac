#[doc = "Register `MSS_QSPI_Reserved3` reader"]
pub type R = crate::R<MssQspiReserved3Spec>;
#[doc = "Register `MSS_QSPI_Reserved3` writer"]
pub type W = crate::W<MssQspiReserved3Spec>;
#[doc = "Field `Reserved_3` reader - 31:0\\]
Reserved"]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `Reserved_3` writer - 31:0\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn reserved_3(&self) -> Reserved3R {
        Reserved3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_3(&mut self) -> Reserved3W<MssQspiReserved3Spec> {
        Reserved3W::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_qspi_reserved3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_qspi_reserved3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssQspiReserved3Spec;
impl crate::RegisterSpec for MssQspiReserved3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_qspi_reserved3::R`](R) reader structure"]
impl crate::Readable for MssQspiReserved3Spec {}
#[doc = "`write(|w| ..)` method takes [`mss_qspi_reserved3::W`](W) writer structure"]
impl crate::Writable for MssQspiReserved3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_QSPI_Reserved3 to value 0"]
impl crate::Resettable for MssQspiReserved3Spec {
    const RESET_VALUE: u32 = 0;
}
