#[doc = "Register `ESMLTCPR` reader"]
pub type R = crate::R<EsmltcprSpec>;
#[doc = "Register `ESMLTCPR` writer"]
pub type W = crate::W<EsmltcprSpec>;
#[doc = "Field `LTCP` reader - 15:0\\]
ERROR Pin Low-Time Counter Pre-load Value 16bit pre-load value for the ERROR pin low-time counter. Note: Only LTCP.15 and LTCP.14 are configurable (privileged mode write)."]
pub type LtcpR = crate::FieldReader<u16>;
#[doc = "Field `LTCP` writer - 15:0\\]
ERROR Pin Low-Time Counter Pre-load Value 16bit pre-load value for the ERROR pin low-time counter. Note: Only LTCP.15 and LTCP.14 are configurable (privileged mode write)."]
pub type LtcpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
ERROR Pin Low-Time Counter Pre-load Value 16bit pre-load value for the ERROR pin low-time counter. Note: Only LTCP.15 and LTCP.14 are configurable (privileged mode write)."]
    #[inline(always)]
    pub fn ltcp(&self) -> LtcpR {
        LtcpR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
ERROR Pin Low-Time Counter Pre-load Value 16bit pre-load value for the ERROR pin low-time counter. Note: Only LTCP.15 and LTCP.14 are configurable (privileged mode write)."]
    #[inline(always)]
    #[must_use]
    pub fn ltcp(&mut self) -> LtcpW<EsmltcprSpec> {
        LtcpW::new(self, 0)
    }
}
#[doc = "ESM Low-Time Counter Preload Register\n\nYou can [`read`](crate::Reg::read) this register and get [`esmltcpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmltcpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EsmltcprSpec;
impl crate::RegisterSpec for EsmltcprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esmltcpr::R`](R) reader structure"]
impl crate::Readable for EsmltcprSpec {}
#[doc = "`write(|w| ..)` method takes [`esmltcpr::W`](W) writer structure"]
impl crate::Writable for EsmltcprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESMLTCPR to value 0"]
impl crate::Resettable for EsmltcprSpec {
    const RESET_VALUE: u32 = 0;
}
