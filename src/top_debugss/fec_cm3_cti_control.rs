#[doc = "Register `FEC_CM3_CTI_CONTROL` reader"]
pub type R = crate::R<FecCm3CtiControlSpec>;
#[doc = "Register `FEC_CM3_CTI_CONTROL` writer"]
pub type W = crate::W<FecCm3CtiControlSpec>;
#[doc = "Field `FEC_CM3_CTI_CONTROL` reader - "]
pub type FecCm3CtiControlR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_CONTROL` writer - "]
pub type FecCm3CtiControlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_control(&self) -> FecCm3CtiControlR {
        FecCm3CtiControlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_control(&mut self) -> FecCm3CtiControlW<FecCm3CtiControlSpec> {
        FecCm3CtiControlW::new(self, 0)
    }
}
#[doc = "http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdjefbi.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdefejc.html\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiControlSpec;
impl crate::RegisterSpec for FecCm3CtiControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_control::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiControlSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_control::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_CONTROL to value 0"]
impl crate::Resettable for FecCm3CtiControlSpec {
    const RESET_VALUE: u32 = 0;
}
