#[doc = "Register `CMULTSCALE2I` reader"]
pub type R = crate::R<Cmultscale2iSpec>;
#[doc = "Register `CMULTSCALE2I` writer"]
pub type W = crate::W<Cmultscale2iSpec>;
#[doc = "Field `CMULTSCALE2I` reader - 20:0\\]
In CMULT_MODE : 101 , the input samples are multiplied by a different complex scalar CMULTSCALE1I, CMULTSCALE1Q to CMULTSCALE6I, CMULTSCALE6Q per-iteration based on REG_BCNT. Else, a constant complex scalar CMULTSCALE1I and CMULTSCALEQI is applied to all sample across all iterations."]
pub type Cmultscale2iR = crate::FieldReader<u32>;
#[doc = "Field `CMULTSCALE2I` writer - 20:0\\]
In CMULT_MODE : 101 , the input samples are multiplied by a different complex scalar CMULTSCALE1I, CMULTSCALE1Q to CMULTSCALE6I, CMULTSCALE6Q per-iteration based on REG_BCNT. Else, a constant complex scalar CMULTSCALE1I and CMULTSCALEQI is applied to all sample across all iterations."]
pub type Cmultscale2iW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u16>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:20 - 20:0\\]
In CMULT_MODE : 101 , the input samples are multiplied by a different complex scalar CMULTSCALE1I, CMULTSCALE1Q to CMULTSCALE6I, CMULTSCALE6Q per-iteration based on REG_BCNT. Else, a constant complex scalar CMULTSCALE1I and CMULTSCALEQI is applied to all sample across all iterations."]
    #[inline(always)]
    pub fn cmultscale2i(&self) -> Cmultscale2iR {
        Cmultscale2iR::new(self.bits & 0x001f_ffff)
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
    pub fn cmultscale2i(&mut self) -> Cmultscale2iW<Cmultscale2iSpec> {
        Cmultscale2iW::new(self, 0)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Cmultscale2iSpec> {
        Nu1W::new(self, 21)
    }
}
#[doc = "CMULTSCALE2I\n\nYou can [`read`](crate::Reg::read) this register and get [`cmultscale2i::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmultscale2i::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmultscale2iSpec;
impl crate::RegisterSpec for Cmultscale2iSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmultscale2i::R`](R) reader structure"]
impl crate::Readable for Cmultscale2iSpec {}
#[doc = "`write(|w| ..)` method takes [`cmultscale2i::W`](W) writer structure"]
impl crate::Writable for Cmultscale2iSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMULTSCALE2I to value 0"]
impl crate::Resettable for Cmultscale2iSpec {
    const RESET_VALUE: u32 = 0;
}
