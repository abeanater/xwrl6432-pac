#[doc = "Register `RDSTATUS` reader"]
pub type R = crate::R<RdstatusSpec>;
#[doc = "Register `RDSTATUS` writer"]
pub type W = crate::W<RdstatusSpec>;
#[doc = "Field `PARAMADDR` reader - 4:0\\]
Index of the current parameter set being executed from PARAM RAM . For Debug only"]
pub type ParamaddrR = crate::FieldReader;
#[doc = "Field `PARAMADDR` writer - 4:0\\]
Index of the current parameter set being executed from PARAM RAM . For Debug only"]
pub type ParamaddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LOOPCNT` reader - 16:5\\]
Running value of the loop count when the HWA is executing from PARAM RAM . For Debug only"]
pub type LoopcntR = crate::FieldReader<u16>;
#[doc = "Field `LOOPCNT` writer - 16:5\\]
Running value of the loop count when the HWA is executing from PARAM RAM . For Debug only"]
pub type LoopcntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u16>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Index of the current parameter set being executed from PARAM RAM . For Debug only"]
    #[inline(always)]
    pub fn paramaddr(&self) -> ParamaddrR {
        ParamaddrR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:16 - 16:5\\]
Running value of the loop count when the HWA is executing from PARAM RAM . For Debug only"]
    #[inline(always)]
    pub fn loopcnt(&self) -> LoopcntR {
        LoopcntR::new(((self.bits >> 5) & 0x0fff) as u16)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Index of the current parameter set being executed from PARAM RAM . For Debug only"]
    #[inline(always)]
    #[must_use]
    pub fn paramaddr(&mut self) -> ParamaddrW<RdstatusSpec> {
        ParamaddrW::new(self, 0)
    }
    #[doc = "Bits 5:16 - 16:5\\]
Running value of the loop count when the HWA is executing from PARAM RAM . For Debug only"]
    #[inline(always)]
    #[must_use]
    pub fn loopcnt(&mut self) -> LoopcntW<RdstatusSpec> {
        LoopcntW::new(self, 5)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<RdstatusSpec> {
        NuW::new(self, 17)
    }
}
#[doc = "RDSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`rdstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdstatusSpec;
impl crate::RegisterSpec for RdstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdstatus::R`](R) reader structure"]
impl crate::Readable for RdstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`rdstatus::W`](W) writer structure"]
impl crate::Writable for RdstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RDSTATUS to value 0"]
impl crate::Resettable for RdstatusSpec {
    const RESET_VALUE: u32 = 0;
}
