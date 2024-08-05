#[doc = "Register `CMULTSCALE3Q` reader"]
pub type R = crate::R<Cmultscale3qSpec>;
#[doc = "Register `CMULTSCALE3Q` writer"]
pub type W = crate::W<Cmultscale3qSpec>;
#[doc = "Field `CMULTSCALE3Q` reader - 20:0\\]
In CMULT_MODE : 101 , the input samples are multiplied by a different complex scalar CMULTSCALE1I, CMULTSCALE1Q to CMULTSCALE6I, CMULTSCALE6Q per-iteration based on REG_BCNT. Else, a constant complex scalar CMULTSCALE1I and CMULTSCALEQI is applied to all sample across all iterations."]
pub type Cmultscale3qR = crate::FieldReader<u32>;
#[doc = "Field `CMULTSCALE3Q` writer - 20:0\\]
In CMULT_MODE : 101 , the input samples are multiplied by a different complex scalar CMULTSCALE1I, CMULTSCALE1Q to CMULTSCALE6I, CMULTSCALE6Q per-iteration based on REG_BCNT. Else, a constant complex scalar CMULTSCALE1I and CMULTSCALEQI is applied to all sample across all iterations."]
pub type Cmultscale3qW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u16>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:20 - 20:0\\]
In CMULT_MODE : 101 , the input samples are multiplied by a different complex scalar CMULTSCALE1I, CMULTSCALE1Q to CMULTSCALE6I, CMULTSCALE6Q per-iteration based on REG_BCNT. Else, a constant complex scalar CMULTSCALE1I and CMULTSCALEQI is applied to all sample across all iterations."]
    #[inline(always)]
    pub fn cmultscale3q(&self) -> Cmultscale3qR {
        Cmultscale3qR::new(self.bits & 0x001f_ffff)
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
    pub fn cmultscale3q(&mut self) -> Cmultscale3qW<Cmultscale3qSpec> {
        Cmultscale3qW::new(self, 0)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Cmultscale3qSpec> {
        Nu1W::new(self, 21)
    }
}
#[doc = "CMULTSCALE3Q\n\nYou can [`read`](crate::Reg::read) this register and get [`cmultscale3q::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmultscale3q::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmultscale3qSpec;
impl crate::RegisterSpec for Cmultscale3qSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmultscale3q::R`](R) reader structure"]
impl crate::Readable for Cmultscale3qSpec {}
#[doc = "`write(|w| ..)` method takes [`cmultscale3q::W`](W) writer structure"]
impl crate::Writable for Cmultscale3qSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMULTSCALE3Q to value 0"]
impl crate::Resettable for Cmultscale3qSpec {
    const RESET_VALUE: u32 = 0;
}
