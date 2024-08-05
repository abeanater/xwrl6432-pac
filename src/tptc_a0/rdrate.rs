#[doc = "Register `RDRATE` reader"]
pub type R = crate::R<RdrateSpec>;
#[doc = "Register `RDRATE` writer"]
pub type W = crate::W<RdrateSpec>;
#[doc = "Field `READ_RATE_CONTROL` reader - 2:0\\]
Read Rate Control: Controls the number of cycles between read commands. This is a global setting that applies to all TRs for this TC."]
pub type ReadRateControlR = crate::FieldReader;
#[doc = "Field `READ_RATE_CONTROL` writer - 2:0\\]
Read Rate Control: Controls the number of cycles between read commands. This is a global setting that applies to all TRs for this TC."]
pub type ReadRateControlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Read Rate Control: Controls the number of cycles between read commands. This is a global setting that applies to all TRs for this TC."]
    #[inline(always)]
    pub fn read_rate_control(&self) -> ReadRateControlR {
        ReadRateControlR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Read Rate Control: Controls the number of cycles between read commands. This is a global setting that applies to all TRs for this TC."]
    #[inline(always)]
    #[must_use]
    pub fn read_rate_control(&mut self) -> ReadRateControlW<RdrateSpec> {
        ReadRateControlW::new(self, 0)
    }
}
#[doc = "Read Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdrate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdrate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdrateSpec;
impl crate::RegisterSpec for RdrateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdrate::R`](R) reader structure"]
impl crate::Readable for RdrateSpec {}
#[doc = "`write(|w| ..)` method takes [`rdrate::W`](W) writer structure"]
impl crate::Writable for RdrateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RDRATE to value 0"]
impl crate::Resettable for RdrateSpec {
    const RESET_VALUE: u32 = 0;
}
