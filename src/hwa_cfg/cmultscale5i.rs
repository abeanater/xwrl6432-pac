#[doc = "Register `CMULTSCALE5I` reader"]
pub type R = crate::R<Cmultscale5iSpec>;
#[doc = "Register `CMULTSCALE5I` writer"]
pub type W = crate::W<Cmultscale5iSpec>;
#[doc = "Field `CMULTSCALE5I` reader - 20:0\\]
In CMULT_MODE : 101 , the input samples are multiplied by a different complex scalar CMULTSCALE1I, CMULTSCALE1Q to CMULTSCALE6I, CMULTSCALE6Q per-iteration based on REG_BCNT. Else, a constant complex scalar CMULTSCALE1I and CMULTSCALEQI is applied to all sample across all iterations."]
pub type Cmultscale5iR = crate::FieldReader<u32>;
#[doc = "Field `CMULTSCALE5I` writer - 20:0\\]
In CMULT_MODE : 101 , the input samples are multiplied by a different complex scalar CMULTSCALE1I, CMULTSCALE1Q to CMULTSCALE6I, CMULTSCALE6Q per-iteration based on REG_BCNT. Else, a constant complex scalar CMULTSCALE1I and CMULTSCALEQI is applied to all sample across all iterations."]
pub type Cmultscale5iW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u16>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:20 - 20:0\\]
In CMULT_MODE : 101 , the input samples are multiplied by a different complex scalar CMULTSCALE1I, CMULTSCALE1Q to CMULTSCALE6I, CMULTSCALE6Q per-iteration based on REG_BCNT. Else, a constant complex scalar CMULTSCALE1I and CMULTSCALEQI is applied to all sample across all iterations."]
    #[inline(always)]
    pub fn cmultscale5i(&self) -> Cmultscale5iR {
        Cmultscale5iR::new(self.bits & 0x001f_ffff)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:20 - 20:0\\]
In CMULT_MODE : 101 , the input samples are multiplied by a different complex scalar CMULTSCALE1I, CMULTSCALE1Q to CMULTSCALE6I, CMULTSCALE6Q per-iteration based on REG_BCNT. Else, a constant complex scalar CMULTSCALE1I and CMULTSCALEQI is applied to all sample across all iterations."]
    #[inline(always)]
    #[must_use]
    pub fn cmultscale5i(&mut self) -> Cmultscale5iW<Cmultscale5iSpec> {
        Cmultscale5iW::new(self, 0)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Cmultscale5iSpec> {
        Nu1W::new(self, 21)
    }
}
#[doc = "CMULTSCALE5I\n\nYou can [`read`](crate::Reg::read) this register and get [`cmultscale5i::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmultscale5i::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmultscale5iSpec;
impl crate::RegisterSpec for Cmultscale5iSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmultscale5i::R`](R) reader structure"]
impl crate::Readable for Cmultscale5iSpec {}
#[doc = "`write(|w| ..)` method takes [`cmultscale5i::W`](W) writer structure"]
impl crate::Writable for Cmultscale5iSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMULTSCALE5I to value 0"]
impl crate::Resettable for Cmultscale5iSpec {
    const RESET_VALUE: u32 = 0;
}
